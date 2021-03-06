mod error;
mod logger;
mod plugin;

use classicube_sys::*;
use log::*;
use std::{os::raw::c_int, ptr};

extern "C" fn init() {
    logger::initialize(true, false);
    debug!("blockbench-loader init");

    plugin::init();
}

extern "C" fn free() {
    debug!("blockbench-loader free");

    plugin::free();
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
    // Called when the game is being loaded.
    Init: Some(init),
    // Called when the component is being freed. (e.g. due to game being closed)
    Free: Some(free),
    // Called to reset the component's state. (e.g. reconnecting to server)
    Reset: None,
    // Called to update the component's state when the user begins loading a new map.
    OnNewMap: None,
    // Called to update the component's state when the user has finished loading a new map.
    OnNewMapLoaded: None,
    // Next component in linked list of components.
    next: ptr::null_mut(),
};
