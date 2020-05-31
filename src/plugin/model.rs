#![allow(non_snake_case)]
#![allow(clippy::box_vec)]

use classicube_sys::{
    Bitmap, BoxDesc, BoxDesc_BuildBox, Entity, GfxResourceID, Model as CCModel, ModelPart,
    ModelTex, ModelVertex, Model_ApplyTexture, Model_DrawPart, Model_Init, Model_Register,
    Model_RetAABB, Model_RetSize, Model_UpdateVB, OwnedGfxTexture, SKIN_TYPE_SKIN_64x64,
    MODEL_BOX_VERTICES,
};
use log::*;
use std::{cell::RefCell, collections::HashMap, ffi::CString, mem, os::raw::c_float, pin::Pin};

// just so we keep them alive
thread_local!(
    static MODELS: RefCell<HashMap<*const CCModel, Model>> = Default::default();
);

pub fn free() {
    debug!("model::free()");

    MODELS.with(|cell| {
        let models = &mut *cell.borrow_mut();
        models.clear();
    });
}

#[allow(dead_code)]
pub struct Model {
    name: String,

    model: Pin<Box<CCModel>>,

    model_name: Pin<Box<CString>>,
    vertices: Pin<Box<Vec<ModelVertex>>>,
    default_tex: Pin<Box<ModelTex>>,

    default_tex_name: Pin<Box<CString>>,
    default_tex_texture: OwnedGfxTexture,

    box_descs: Vec<BoxDesc>,
    model_parts: Option<Vec<ModelPart>>,
}

impl Model {
    pub fn register(name: &str, bmp: Bitmap, box_descs: Vec<BoxDesc>) {
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

        let model = Self {
            model,
            name: name.to_string(),
            model_name,
            vertices,
            default_tex,
            default_tex_name,
            default_tex_texture,
            box_descs,
            model_parts: None,
        };

        MODELS.with(move |cell| {
            let models = &mut *cell.borrow_mut();
            models.insert(model.model.as_ref().get_ref(), model);
        });
    }

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
        let name = Box::pin(CString::new(name).unwrap());

        let mut model: CCModel = unsafe { mem::zeroed() };
        model.name = name.as_ptr();
        model.vertices = vertices.as_mut_ptr();
        model.defaultTex = unsafe { model_tex.as_mut().get_unchecked_mut() };
        model.MakeParts = Some(Self::MakeParts);
        model.Draw = Some(Self::Draw);
        model.GetNameY = Some(Self::GetNameY);
        model.GetEyeY = Some(Self::GetEyeY);
        model.GetCollisionSize = Some(Self::GetCollisionSize);
        model.GetPickingBounds = Some(Self::GetPickingBounds);

        (Box::pin(model), name)
    }

    fn with_by_model_ptr<F, T>(ptr: *const CCModel, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        MODELS.with(move |cell| {
            let models = &mut *cell.borrow_mut();

            f(models.get_mut(&ptr).unwrap())
        })
    }

    fn get_model_parts(&mut self) -> &mut [ModelPart] {
        if self.model_parts.is_none() {
            debug!(
                "creating {} model parts for {}",
                self.box_descs.len(),
                self.name
            );
            let mut model_parts = Vec::new();

            for desc in &self.box_descs {
                debug!("{:#?}", desc);
                unsafe {
                    let mut part: ModelPart = mem::zeroed();
                    BoxDesc_BuildBox(&mut part, &*desc);
                    model_parts.push(part);
                }
            }

            self.model_parts = Some(model_parts);
        }

        self.model_parts.as_mut().unwrap()
    }

    /// Creates the ModelParts of this model and fills out vertices.
    extern "C" fn MakeParts() {
        // do nothing because we can't know which model this was
    }

    /// Draws/Renders this model for the given entity.
    unsafe extern "C" fn Draw(entity: *mut Entity) {
        let entity = &mut *entity;

        Model_ApplyTexture(entity);

        Self::with_by_model_ptr(entity.Model, |model| {
            for part in model.get_model_parts() {
                Model_DrawPart(&mut *part);
            }
        });

        Model_UpdateVB();
    }

    /// Returns height the 'nametag' gets drawn at above the entity's feet.
    extern "C" fn GetNameY(_entity: *mut Entity) -> c_float {
        32.5 / 16.0
    }

    /// Returns height the 'eye' is located at above the entity's feet.
    extern "C" fn GetEyeY(_entity: *mut Entity) -> c_float {
        26.0 / 16.0
    }

    /// Sets entity->Size to the collision size of this model.
    unsafe extern "C" fn GetCollisionSize(entity: *mut Entity) {
        let entity = &mut *entity;
        Model_RetSize!(entity, 8.6, 28.1, 8.6);
    }

    /// Sets entity->ModelAABB to the 'picking' bounds of this model.
    /// This is the AABB around the entity in which mouse clicks trigger 'interaction'.
    /// NOTE: These bounds are not transformed. (i.e. no rotation, centered around 0,0,0)
    unsafe extern "C" fn GetPickingBounds(entity: *mut Entity) {
        let entity = &mut *entity;
        Model_RetAABB!(entity, -8.0, 0.0, -4.0, 8.0, 32.0, 4.0);
    }
}
