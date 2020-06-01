use serde::{Deserialize, Serialize};
use std::os::raw::c_float;

#[derive(Debug, Serialize, Deserialize)]
pub struct BBModel {
    // meta: Meta,
    pub name: String,
    pub resolution: Resolution,
    pub elements: Vec<Element>,
    // outliner: Vec<Outline>,
    pub textures: Vec<Texture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    pub name: String,
    pub id: String,
    pub particle: bool,
    pub mode: TextureMode,
    pub uuid: String,

    /// base64 data url
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextureMode {
    Bitmap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub name: String,
    pub from: [c_float; 3],
    pub to: [c_float; 3],

    // so far only 0?
    pub autouv: usize,

    // some kind of index?
    pub color: usize,

    // so far only false?
    pub locked: bool,
    pub rotation: Option<[c_float; 3]>,

    /// "Pivot Point"
    pub origin: [c_float; 3],

    pub uv_offset: Option<[usize; 2]>,
    pub faces: Faces,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faces {
    pub north: Face,
    pub east: Face,
    pub south: Face,
    pub west: Face,
    pub up: Face,
    pub down: Face,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Face {
    pub uv: [usize; 4],
    pub texture: usize,
}

#[test]
fn test_json() {
    use std::fs::File;

    let mut f = File::open("tests/Player.bbmodel").unwrap();

    let json: BBModel = serde_json::from_reader(&mut f).unwrap();
    println!("{:#?}", json);
}
