//use std::any::type_name;

mod title_parser;
use inotify::{
    Inotify,
    WatchMask,
};

fn main() {
    let mut inotify = Inotify::init()
    .expect("Error while initializing inotify instance");

    inotify
        .watches()
        .add(
            "/tmp",
            WatchMask::MODIFY | WatchMask::CLOSE,
        )
        .expect("Failed to add file watch");

    let mut buffer = [0; 1024];
    let events = inotify.read_events_blocking(&mut buffer)
        .expect("Error while reading events");

    for event in events {
        if let Some(name_os) = event.name {
            if let Some(name) = name_os.to_str() {
                println!("{}", name);

            }
        }
    }

}
