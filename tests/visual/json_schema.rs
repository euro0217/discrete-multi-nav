use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    pub seats: Vec<Seat>,
    pub agents: HashMap<u32, HashMap<i32, Agent>>
}

#[derive(Serialize)]
pub struct Seat {
    pub x: i32,
    pub y: i32,
    pub nexts: Vec<usize>,
    pub agent: Vec<Option<u32>>,
}


#[derive(Serialize)]
pub struct Agent {
    pub shape: Vec<Option<(f32, f32)>>,
    pub state: String,
}
