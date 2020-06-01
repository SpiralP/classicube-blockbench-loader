mod blockbench;
mod model;

use self::{blockbench::Blockbench, model::Model};
use classicube_helpers::tick::TickEventHandler;
use log::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{cell::RefCell, sync::mpsc::channel, time::Duration};

use std::{fs, io::Read, path::Path};

thread_local!(
    static TICK_HANDLER: RefCell<Option<TickEventHandler>> = Default::default();
);

thread_local!(
    static WATCHER: RefCell<Option<RecommendedWatcher>> = Default::default();
);

fn load(path: &Path) {
    let data = {
        debug!("opening file {:?}", path);
        let mut file = fs::File::open(&path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        data
    };

    let bb = Blockbench::parse_bbmodel(&data).unwrap();
    bb.register_model(path.file_stem().unwrap().to_str().unwrap());
}

pub fn init() {
    let plugins_path = Path::new("plugins");
    assert!(plugins_path.is_dir());

    let blockbench_path = plugins_path.join("blockbench");
    if !blockbench_path.is_dir() {
        fs::create_dir(&blockbench_path).unwrap();
    }

    for entry in fs::read_dir(&blockbench_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "bbmodel" && path.is_file() {
            load(&path);
        }
    }

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    WATCHER.with(move |cell| {
        let opt = &mut *cell.borrow_mut();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(&blockbench_path, RecursiveMode::Recursive)
            .unwrap();

        *opt = Some(watcher);
    });

    TICK_HANDLER.with(move |cell| {
        let opt = &mut *cell.borrow_mut();

        let mut tick_handler = TickEventHandler::new();
        tick_handler.on(move |_| {
            for event in rx.try_iter() {
                debug!("{:?}", event);

                let maybe = match event {
                    DebouncedEvent::Create(path) => Some(path),
                    DebouncedEvent::Write(path) => Some(path),
                    DebouncedEvent::Rename(_old, path) => Some(path),

                    _ => None,
                };

                if let Some(path) = maybe {
                    if path.extension().unwrap() == "bbmodel" && path.is_file() {
                        load(&path);
                    }
                }
            }
        });

        *opt = Some(tick_handler);
    });
}

pub fn free() {
    TICK_HANDLER.with(|cell| {
        let opt = &mut *cell.borrow_mut();
        drop(opt.take());
    });

    WATCHER.with(|cell| {
        let opt = &mut *cell.borrow_mut();
        drop(opt.take());
    });

    model::free();
}
