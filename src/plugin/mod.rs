mod json;
mod model;

use self::model::Model;
use classicube_sys::Bitmap;
use std::cell::RefCell;

thread_local!(
    static MODELS: RefCell<Vec<Model>> = Default::default();
);

pub fn init() {
    let texture_width = 32;
    let texture_height = 32;

    // must be a vec or else we try to fit huge array onto stack and crash!
    let mut pixels: Vec<u8> = vec![255; 4 * texture_width * texture_height];

    let bmp = Bitmap {
        Scan0: pixels.as_mut_ptr(),
        Width: texture_width as i32,
        Height: texture_height as i32,
    };

    let model = Model::register("testie", bmp);

    MODELS.with(|cell| {
        let models = &mut *cell.borrow_mut();

        models.push(model);
    });
}

pub fn free() {
    MODELS.with(|cell| {
        let models = &mut *cell.borrow_mut();
        models.clear();
    });
}
