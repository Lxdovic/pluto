use engine::packing::*;

pub struct TunerParam {
    pub value: i32,
    pub min: i32,
    pub min_init: i32,
    pub max_init: i32,
    pub max: i32,
}

impl TunerParam {
    pub fn new(value: i32, min: i32, min_init: i32, max_init: i32, max: i32) -> Self {
        Self {
            value,
            min,
            min_init,
            max_init,
            max,
        }
    }
}

// #[derive(Debug)]
// pub struct Param {
//     pub name: String,
//     pub original_mg: i32,
//     pub original_eg: i32,
//     pub mg: f32,
//     pub eg: f32,
//     pub tunable: bool,
// }
//
// impl Param {
//     pub fn from_packed(name: String, v: i32, tunable: bool) -> Self {
//         let mg = extract_mg(v);
//         let eg = extract_eg(v);
//
//         Self {
//             name,
//             original_mg: mg,
//             original_eg: eg,
//             mg: mg as f32,
//             eg: eg as f32,
//             tunable,
//         }
//     }
// }
