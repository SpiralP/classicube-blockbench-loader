use crate::plugin::blockbench;
use classicube_sys::{
    cc_uint16, BoxDesc_XQuad, BoxDesc_YQuad, BoxDesc_ZQuad, ModelPart, ModelPart_Init,
    Model_DrawPart, Model_DrawRotate, Models, MODEL_BOX_VERTICES,
};
use std::{
    mem,
    os::raw::{c_float, c_int},
};

#[derive(Debug, Default)]
pub struct Cube {
    pub from: [c_float; 3],
    pub to: [c_float; 3],

    pub tex_x: c_int,
    pub tex_y: c_int,

    pub tex_sides_w: c_int,
    pub tex_body_w: c_int,
    pub tex_body_h: c_int,

    /// pivot point/origin
    pub pivot_origin: [c_float; 3],

    /// angled rotation, passed to Model_DrawRotate
    pub rot: Option<[c_float; 3]>,

    model_part: Option<ModelPart>,
}

impl Cube {
    #[rustfmt::skip]
    pub fn make_part(&mut self) {
        unsafe {
            let m = &mut *Models.Active;
            let mut part: ModelPart = mem::zeroed();

            let x1 = self.from[0];
            let y1 = self.from[1];
            let z1 = self.from[2];

            let x2 = self.to[0];
            let y2 = self.to[1];
            let z2 = self.to[2];

            let x = self.tex_x;
            let y = self.tex_y;

            let sidesW = self.tex_sides_w;
            let bodyW = self.tex_body_w;
            let bodyH = self.tex_body_h;

            let rotX = self.pivot_origin[0];
            let rotY = self.pivot_origin[1];
            let rotZ = self.pivot_origin[2];

            BoxDesc_YQuad(m, x + sidesW,                  y,          bodyW, sidesW, x1, x2, z2, z1, y2, 1);  /* top */
            BoxDesc_YQuad(m, x + sidesW + bodyW,          y,          bodyW, sidesW, x2, x1, z2, z1, y1, 0);  /* bottom */
            BoxDesc_ZQuad(m, x + sidesW,                  y + sidesW, bodyW,  bodyH, x1, x2, y1, y2, z1, 1);  /* front */
            BoxDesc_ZQuad(m, x + sidesW + bodyW + sidesW, y + sidesW, bodyW,  bodyH, x2, x1, y1, y2, z2, 1);  /* back */
            BoxDesc_XQuad(m, x,                           y + sidesW, sidesW, bodyH, z1, z2, y1, y2, x2, 1);  /* left */
            BoxDesc_XQuad(m, x + sidesW + bodyW,          y + sidesW, sidesW, bodyH, z2, z1, y1, y2, x1, 1);  /* right */

            ModelPart_Init(
                &mut part,
                m.index as cc_uint16 - MODEL_BOX_VERTICES as cc_uint16,
                MODEL_BOX_VERTICES as cc_uint16,
                rotX,
                rotY,
                rotZ
            );

            self.model_part = Some(part);
        }
    }

    /// must call `make_part` first!
    pub fn draw(&mut self) {
        if let Some(rot) = self.rot {
            unsafe {
                // TODO head last arg bool
                Model_DrawRotate(rot[0], rot[1], rot[2], self.model_part.as_mut().unwrap(), 0);
            }
        } else {
            unsafe {
                Model_DrawPart(self.model_part.as_mut().unwrap());
            }
        }
    }
}

impl Cube {
    pub fn from_bbmodel_element(e: blockbench::json::Element) -> Self {
        let from = [e.from[0] / 16.0, e.from[1] / 16.0, e.from[2] / 16.0];
        let to = [e.to[0] / 16.0, e.to[1] / 16.0, e.to[2] / 16.0];

        let tex_x = e.uv_offset.map(|a| a[0]).unwrap_or(0) as c_int;
        let tex_y = e.uv_offset.map(|a| a[1]).unwrap_or(0) as c_int;

        let tex_sides_w = (e.faces.east.uv[2] as c_int - e.faces.east.uv[0] as c_int).abs();
        let tex_body_w = (e.faces.up.uv[2] as c_int - e.faces.up.uv[0] as c_int).abs();
        let tex_body_h = (e.faces.east.uv[3] as c_int - e.faces.east.uv[1] as c_int).abs();

        let pivot_origin = [e.origin[0] / 16.0, e.origin[1] / 16.0, e.origin[2] / 16.0];

        let rot = e.rotation.map(|r| [r[0], r[1], r[2]]);

        Self {
            from,
            to,
            tex_x,
            tex_y,
            tex_sides_w,
            tex_body_w,
            tex_body_h,
            pivot_origin,
            rot,
            model_part: None,
        }
    }
}
