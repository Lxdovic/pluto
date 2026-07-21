pub mod uci;
pub mod search;

fn main() {
    uci::Uci::default().run();
}   
