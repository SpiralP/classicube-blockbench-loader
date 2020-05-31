mod json;

use self::json::BBModel;
use super::Model;
use crate::error::*;
use classicube_sys::{Bitmap, BoxDesc, BoxDesc_Box, BoxDesc_Tex};
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
        if !texture.source.starts_with(DATA_URL_START) {
            bail!("texture not base64 png data url");
        }

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
            "texture bitdepth not 8"
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
            bail!("unimplemented color_type {:?}", info.color_type);
        }

        for i in 0..pixels.len() {
            pixels[i] = 255;
        }

        Ok(Self { pixels, bb })
    }

    pub fn register_model(mut self) {
        let bmp = Bitmap {
            Scan0: self.pixels.as_mut_ptr(),
            Width: self.bb.resolution.width as c_int,
            Height: self.bb.resolution.height as c_int,
        };

        let mut parts = Vec::new();

        for e in self.bb.elements {
            parts.push(BoxDesc::from_macros(
                BoxDesc_Tex!(
                    0, // e.uv_offset.map(|a| a[0]).unwrap_or(0) as _,
                    0  // e.uv_offset.map(|a| a[1]).unwrap_or(0) as _
                ),
                BoxDesc_Box!(e.from[0], e.from[1], e.from[2], e.to[0], e.to[1], e.to[2]),
            ));
        }

        Model::register(&self.bb.name, bmp, parts);
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
