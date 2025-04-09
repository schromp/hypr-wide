use std::{cell::RefCell, rc::Rc};

use hyprland::{event_listener::EventListener, shared::Address};
use log::{debug, info};

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    info!("Starting the hypr-wide daemon.");

    let mut listener = EventListener::new();

    let mut managed_windows = Rc::new(RefCell::new(Vec::<Address>::new()));
    let m = managed_windows.clone();

    listener.add_window_opened_handler(|id| {
        m.borrow_mut().push(id.window_address);
    });

    let _ = listener.start_listener();
}

fn window_opened(managed_windows: Rc<RefCell<Vec<Address>>>, address: Address) {
    debug!("window with address {address:?} opened");
    managed_windows.borrow_mut().push(address);
}
