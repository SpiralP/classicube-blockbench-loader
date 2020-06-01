pub mod json;

use self::json::BBModel;
use super::{model::Cube, Model};
use crate::error::*;
use classicube_sys::Bitmap;
use log::*;
use std::{io::Cursor, os::raw::c_int};

#[derive(Debug)]
pub struct Blockbench {
    bb: BBModel,
    pixels: Vec<u8>,
}

impl Blockbench {
    pub fn parse_bbmodel(data: &[u8]) -> Result<Self> {
        let bb: BBModel = serde_json::from_slice(data)?;

        let texture = bb.textures.get(0).chain_err(|| "no texture at index 0")?;
        // if let json::TextureMode::Bitmap = texture.mode {
        // } else {
        //     bail!("not bitmap");
        // }

        const DATA_URL_START: &str = "data:image/png;base64,";
        ensure!(
            texture.source.starts_with(DATA_URL_START),
            "unimplemented: texture not base64 png data url"
        );

        let base64 = &texture.source[DATA_URL_START.len()..];
        let data = base64::decode(base64)?;
        let decoder = png::Decoder::new(Cursor::new(data));
        let (info, mut reader) = decoder.read_info()?;
        debug!("{:#?}", info);
        ensure!(
            info.width as usize == bb.resolution.width,
            "texture width mismatch"
        );
        ensure!(
            info.height as usize == bb.resolution.height,
            "texture height mismatch"
        );
        ensure!(
            info.bit_depth == png::BitDepth::Eight,
            "unimplemented: texture bitdepth not 8"
        );

        // Allocate the output buffer.
        let mut pixels = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut pixels)?;
        debug!("{} pixels", pixels.len());

        if info.color_type == png::ColorType::RGBA {
            // cc uses BGRA, so we need to convert based on info.color_type
            for i in (0..pixels.len()).step_by(4) {
                // swap blue with red
                pixels.swap(i, i + 2);
            }
        } else {
            bail!("unimplemented: color_type {:?}", info.color_type);
        }

        for e in &bb.elements {
            ensure!(e.autouv == 0, "unimplemented: autouv not 0");
            ensure!(!e.locked, "unimplemented: locked not false");
            ensure!(
                e.name == "cube",
                "unimplemented: different name {:?}",
                e.name
            );
        }

        let mut found_non_zero = false;
        for pixel in &pixels {
            if *pixel != 0 {
                found_non_zero = true;
                break;
            }
        }
        if !found_non_zero {
            bail!("image is all 0's?");
        }

        Ok(Self { pixels, bb })
    }

    pub fn register_model(mut self, name: &str) {
        let bmp = Bitmap {
            Scan0: self.pixels.as_mut_ptr(),
            Width: self.bb.resolution.width as c_int,
            Height: self.bb.resolution.height as c_int,
        };

        let mut parts = Vec::new();

        // east is left
        // top is top
        for e in self.bb.elements {
            parts.push(Cube::from_bbmodel_element(e));
        }

        Model::register(&name, bmp, parts);
    }
}

#[test]
fn test_blockbench() {
    use std::{fs::File, io::Read};

    let mut data = Vec::new();
    let mut f = File::open("tests/Player.bbmodel").unwrap();
    f.read_to_end(&mut data).unwrap();

    drop(Blockbench::parse_bbmodel(&data).unwrap());
}
