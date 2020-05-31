mod blockbench;
mod model;

use self::{blockbench::Blockbench, model::Model};

pub fn init() {
    let data = include_bytes!("../../tests/Player.bbmodel");
    let bb = Blockbench::parse_bbmodel(data).unwrap();
    bb.register_model();
}

pub fn free() {
    model::free();
}
