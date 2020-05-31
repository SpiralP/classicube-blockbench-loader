#![allow(non_snake_case)]
#![allow(clippy::box_vec)]

use classicube_sys::{
    Bitmap, Entity, GfxResourceID, Model as CCModel, ModelTex, ModelVertex, Model_Init,
    Model_Register, OwnedGfxTexture, SKIN_TYPE_SKIN_64x64, MODEL_BOX_VERTICES,
};
use log::*;
use std::{ffi::CString, mem, os::raw::c_float, pin::Pin};

#[allow(dead_code)]
pub struct Model {
    model: Pin<Box<CCModel>>,

    name: Pin<Box<CString>>,
    vertices: Pin<Box<Vec<ModelVertex>>>,
    default_tex: Pin<Box<ModelTex>>,

    default_tex_name: Pin<Box<CString>>,
    default_tex_texture: OwnedGfxTexture,
}

impl Model {
    pub fn register(name: &str, bmp: Bitmap) -> Self {
        debug!("registering {}", name);

        let mut vertices = Box::pin(vec![unsafe { mem::zeroed() }; MODEL_BOX_VERTICES as usize]);

        let default_tex_texture = Self::create_gfx_texture(bmp);
        let (mut default_tex, default_tex_name) = Self::create_model_tex(
            &format!("{}_texture", name),
            default_tex_texture.resource_id,
        );

        // we don't need to register our texture!
        // Model_RegisterTexture(default_tex.as_mut().get_unchecked_mut());

        let (mut model, model_name) = Self::create_model(name, &mut vertices, &mut default_tex);

        unsafe {
            Model_Init(model.as_mut().get_unchecked_mut());
            Model_Register(model.as_mut().get_unchecked_mut());
        }

        Self {
            model,
            name: model_name,
            vertices,
            default_tex,
            default_tex_name,
            default_tex_texture,
        }
    }
}

impl Model {
    fn create_gfx_texture(mut bmp: Bitmap) -> OwnedGfxTexture {
        OwnedGfxTexture::create(&mut bmp, true, false)
    }

    fn create_model_tex(
        name: &str,
        resource_id: GfxResourceID,
    ) -> (Pin<Box<ModelTex>>, Pin<Box<CString>>) {
        let mut tex: ModelTex = unsafe { mem::zeroed() };

        let name = Box::pin(CString::new(name).unwrap());
        tex.name = name.as_ptr();
        tex.skinType = SKIN_TYPE_SKIN_64x64 as _;
        tex.texID = resource_id;

        (Box::pin(tex), name)
    }

    fn create_model(
        name: &str,
        vertices: &mut Pin<Box<Vec<ModelVertex>>>,
        model_tex: &mut Pin<Box<ModelTex>>,
    ) -> (Pin<Box<CCModel>>, Pin<Box<CString>>) {
        let model_name = Box::pin(CString::new(name).unwrap());

        let mut model: CCModel = unsafe { mem::zeroed() };
        model.name = model_name.as_ptr();
        model.vertices = vertices.as_mut_ptr();
        model.defaultTex = unsafe { model_tex.as_mut().get_unchecked_mut() };
        model.MakeParts = Some(Self::MakeParts);
        model.Draw = Some(Self::Draw);
        model.GetNameY = Some(Self::GetNameY);
        model.GetEyeY = Some(Self::GetEyeY);
        model.GetCollisionSize = Some(Self::GetCollisionSize);
        model.GetPickingBounds = Some(Self::GetPickingBounds);

        (Box::pin(model), model_name)
    }

    /// Creates the ModelParts of this model and fills out vertices.
    extern "C" fn MakeParts() {}

    /// Draws/Renders this model for the given entity.
    extern "C" fn Draw(_entity: *mut Entity) {}

    /// Returns height the 'nametag' gets drawn at above the entity's feet.
    extern "C" fn GetNameY(_entity: *mut Entity) -> c_float {
        0.0
    }

    /// Returns height the 'eye' is located at above the entity's feet.
    extern "C" fn GetEyeY(_entity: *mut Entity) -> c_float {
        0.0
    }

    /// Sets entity->Size to the collision size of this model.
    extern "C" fn GetCollisionSize(_entity: *mut Entity) {}

    /// Sets entity->ModelAABB to the 'picking' bounds of this model.
    /// This is the AABB around the entity in which mouse clicks trigger 'interaction'.
    /// NOTE: These bounds are not transformed. (i.e. no rotation, centered around 0,0,0)
    extern "C" fn GetPickingBounds(_entity: *mut Entity) {}
}
