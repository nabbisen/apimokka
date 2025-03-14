use std::sync::Arc;

use fltk::{
    group::Tabs,
    prelude::{GroupExt, WidgetBase},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

mod config_tab;
mod log_tab;
mod server_tab;

pub fn handle(
    config_filepath: &str,
    shutdown_tx: Arc<Mutex<Sender<()>>>,
    server_rx: Arc<Mutex<Receiver<String>>>,
) -> Tabs {
    let mut tabs = Tabs::default_fill();

    let _ = server_tab::handle(server_rx);
    let _ = config_tab::handle(config_filepath, shutdown_tx);
    let _ = log_tab::handle();

    tabs.end();
    tabs.auto_layout();

    tabs
}
