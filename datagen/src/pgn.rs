use std::error::Error;
use std::ops::ControlFlow;
use std::path::Path;
use std::{fs, io, process};

use pgn_reader::{RawComment, Reader, SanPlus, Skip, Visitor};
use shakmaty::{Chess, Position};

use shakmaty::CastlingMode;
use shakmaty::fen::Fen;

use pgn_reader::RawTag;

pub fn extract_pgn(input: &Path, output: &Path) {
    println!("Reading PGN file");

    let contents = match fs::read_to_string(input) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file {}", err);
            process::exit(1);
        }
    };

    println!("File read successfully ({} bytes)", contents.len());
    println!("Starting PGN parsing...");

    let mut reader = Reader::new(io::Cursor::new(&contents));

    let mut out = Vec::new();
    let mut collector = PositionCollector::new();

    let mut game_index = 0;
    let games = reader.read_games(&mut collector).flatten().flatten();

    for positions in games {
        game_index += 1;
        println!("Finished parsing game #{}", game_index);

        let mut pos_index = 0;
        for pos in positions {
            pos_index += 1;

            if pos_index % 1000 == 0 {
                println!(
                    "  Processed {} positions in game #{}",
                    pos_index, game_index
                );
            }

            if let Some(c) = pos.comment.as_ref() {
                let score = c
                    .split_whitespace()
                    .next()
                    .and_then(|num| num.parse::<f64>().ok())
                    .map(|val| (val * 100.0).round() as i32)
                    .unwrap_or(0);

                if score.abs() > 10000 {
                    continue;
                }
            }

            out.push(format!("{}; c0 \"result: {}\"", pos.fen, pos.result));
        }
    }

    println!("Parsing finished. Total positions collected: {}", out.len());

    let content = out.join("\n");

    println!("Writing output");
    fs::write(output, content).expect("Error writing output file");
    println!("Done!");
}

#[derive(Debug)]
struct PositionWithComment {
    fen: String,
    comment: Option<String>,
    result: String,
}

struct PositionCollector {
    positions: Vec<PositionWithComment>,
    current_result: String,
}

impl PositionCollector {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            current_result: "*".to_string(),
        }
    }
}

impl Visitor for PositionCollector {
    type Tags = Option<Chess>;
    type Movetext = Chess;
    type Output = Result<Vec<PositionWithComment>, Box<dyn Error>>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        self.current_result = "*".to_string();
        ControlFlow::Continue(None)
    }

    fn tag(
        &mut self,
        tags: &mut Self::Tags,
        name: &[u8],
        value: RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        if name == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => return ControlFlow::Break(Err(err.into())),
            };
            let pos = match fen.into_position(CastlingMode::Standard) {
                Ok(pos) => pos,
                Err(err) => return ControlFlow::Break(Err(err.into())),
            };
            tags.replace(pos);
        } else if name == b"Result" {
            if let Ok(s) = std::str::from_utf8(value.as_bytes()) {
                self.current_result = s.to_string()
            }
        }
        ControlFlow::Continue(())
    }

    fn begin_variation(
        &mut self,
        _movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true))
    }

    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        let chess = tags.unwrap_or_default();
        self.positions.push(PositionWithComment {
            fen: Fen::from_position(&chess, shakmaty::EnPassantMode::Legal).to_string(),
            comment: None,
            result: self.current_result.clone(),
        });
        ControlFlow::Continue(chess)
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        match san_plus.san.to_move(movetext) {
            Ok(mv) => {
                movetext.play_unchecked(mv);
                self.positions.push(PositionWithComment {
                    fen: Fen::from_position(movetext, shakmaty::EnPassantMode::Legal).to_string(),
                    comment: None,
                    result: self.current_result.clone(),
                });
                ControlFlow::Continue(())
            }
            Err(err) => ControlFlow::Break(Err(err.into())),
        }
    }
    fn comment(
        &mut self,
        _movetext: &mut Self::Movetext,
        comment: RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        if let Ok(s) = std::str::from_utf8(comment.as_bytes()) {
            if let Some(last) = self.positions.last_mut() {
                last.comment = Some(s.to_owned());
            }
        }
        ControlFlow::Continue(())
    }

    fn end_game(&mut self, _movetext: Self::Movetext) -> Self::Output {
        Ok(std::mem::take(&mut self.positions))
    }
}
