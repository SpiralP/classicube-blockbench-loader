use classicube_sys::{
    cc_uint16, BoxDesc_XQuad, BoxDesc_YQuad, BoxDesc_ZQuad, ModelPart, ModelPart_Init, Models,
    MODEL_BOX_VERTICES,
};
use std::{
    mem,
    os::raw::{c_float, c_int},
};

#[derive(Debug)]
pub struct Cube {
    pub from: [c_float; 3],
    pub to: [c_float; 3],

    pub tex_x: c_int,
    pub tex_y: c_int,

    pub tex_sides_w: c_int,
    // pub tex_sides_h: c_int,
    pub tex_body_w: c_int,
    pub tex_body_h: c_int,
}

impl Cube {
    #[rustfmt::skip]
    pub fn build_model_part(self) -> ModelPart {
        unsafe {
            let m = &mut *Models.Active;
            let mut part: ModelPart = mem::zeroed();

            let x1 = self.from[0] / 16.0;
            let y1 = self.from[1] / 16.0;
            let z1 = self.from[2] / 16.0;

            let x2 = self.to[0] / 16.0;
            let y2 = self.to[1] / 16.0;
            let z2 = self.to[2] / 16.0;

            let x = self.tex_x;
            let y = self.tex_y;

            let sidesW = self.tex_sides_w;
            // let sidesH = self.tex_sides_h;
            let bodyW = self.tex_body_w;
            let bodyH = self.tex_body_h;

            let rotX = 0.0;
            let rotY = 0.0;
            let rotZ = 0.0;

            BoxDesc_YQuad(m, x + sidesW,                  y,          bodyW, sidesW, x1, x2, z2, z1, y2, 1);  /* top */
            BoxDesc_YQuad(m, x + sidesW + bodyW,          y,          bodyW, sidesW, x2, x1, z2, z1, y1, 0); /* bottom */
            BoxDesc_ZQuad(m, x + sidesW,                  y + sidesW, bodyW,  bodyH, x1, x2, y1, y2, z1, 1);  /* front */
            BoxDesc_ZQuad(m, x + sidesW + bodyW + sidesW, y + sidesW, bodyW,  bodyH, x2, x1, y1, y2, z2, 1);  /* back */
            BoxDesc_XQuad(m, x,                           y + sidesW, sidesW, bodyH, z1, z2, y1, y2, x2, 1);  /* left */
            BoxDesc_XQuad(m, x + sidesW + bodyW,          y + sidesW, sidesW, bodyH, z2, z1, y1, y2, x1, 1);  /* right */

            ModelPart_Init(&mut part, m.index as cc_uint16 - MODEL_BOX_VERTICES as cc_uint16, MODEL_BOX_VERTICES as _, rotX, rotY, rotZ);

            part
        }
    }
}
