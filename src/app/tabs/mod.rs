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
    server_rx: Arc<Mutex<Receiver<String>>>,
    restart_server_tx: Arc<Mutex<Sender<()>>>,
) -> Tabs {
    let mut tabs = Tabs::default_fill();

    let _ = server_tab::handle(server_rx);
    let _ = config_tab::handle(config_filepath, restart_server_tx);
    let _ = log_tab::handle();

    tabs.end();
    tabs.auto_layout();

    tabs
}
