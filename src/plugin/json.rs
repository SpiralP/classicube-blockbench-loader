use serde::{Deserialize, Serialize};
use std::os::raw::c_float;

#[derive(Debug, Serialize, Deserialize)]
pub struct BBModel {
    // meta: Meta,
    name: String,
    resolution: Resolution,
    elements: Vec<Element>,
    // outliner: Vec<Outline>,
    textures: Vec<Texture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    name: String,
    id: String,
    particle: bool,
    mode: TextureMode,
    uuid: String,

    /// base64 data url
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextureMode {
    Bitmap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    width: usize,
    height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    name: String,
    from: [c_float; 3],
    to: [c_float; 3],

    /// ??
    autouv: usize,

    /// ??
    color: usize,

    /// ??
    locked: bool,
    origin: [c_float; 3],
    uv_offset: Option<[usize; 2]>,
    faces: Faces,
    uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faces {
    north: Face,
    east: Face,
    south: Face,
    west: Face,
    up: Face,
    down: Face,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Face {
    uv: [usize; 4],
    texture: usize,
}

#[test]
fn test_json() {
    use std::fs::File;

    let mut f = File::open("tests/Player.bbmodel").unwrap();

    let json: BBModel = serde_json::from_reader(&mut f).unwrap();
    println!("{:#?}", json);
}
