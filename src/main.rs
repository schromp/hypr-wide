use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet},
    rc::Rc,
};

use hyprland::{
    data::{Client, Clients, Workspaces},
    event_listener::EventListener,
    shared::{Address, HyprData},
};
use log::{debug, error, info};

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    info!("Starting the hypr-wide daemon.");

    let mut listener = EventListener::new();

    let managed_windows = Rc::new(RefCell::new(HashMap::<Address, Client>::new()));

    let m = managed_windows.clone();
    listener.add_window_opened_handler(move |event| {
        check_workspace(m.borrow_mut(), event.window_address)
    });

    let m = managed_windows.clone();
    listener.add_window_closed_handler(move |address| {
        check_workspace(m.borrow_mut(), address);
    });

    let _ = listener.start_listener();
}

fn check_workspace(mut managed_clients: RefMut<HashMap<Address, Client>>, address: Address) {
    let clients = if let Ok(client) = Clients::get() {
        client
    } else {
        error!("Could not get clients.");
        return;
    };

    let client = if let Some(client) = clients.iter().find(|&c| c.address == address) {
        client
    } else {
        error!("Could not find the client.");
        return;
    };

    let workspaces = if let Ok(ws) = Workspaces::get() {
        ws
    } else {
        error!("Could not get workspaces.");
        return;
    };

    let workspace = if let Some(ws) = workspaces.iter().find(|&w| w.id == client.workspace.id) {
        ws
    } else {
        error!("Could not find the workspace name of the opened window.");
        return;
    };

    if workspace.windows == 1 {
        debug!("Started managing {:?}", &client.title);
        managed_clients.insert(client.address.clone(), client.clone());

        // TODO: pseudotile the window and resize it
    }
}
