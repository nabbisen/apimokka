use std::sync::Arc;

use apimock::core::config::ConfigUrlPaths;
use fltk::{
    group::Tabs,
    prelude::{GroupExt, WidgetBase},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

mod client_tab;
mod config_tab;
mod config_url_paths_tab;
mod server_tab;

pub fn handle(
    config_filepath: &str,
    config_url_paths: Option<ConfigUrlPaths>,
    server_rx: Arc<Mutex<Receiver<String>>>,
    restart_server_tx: Arc<Mutex<Sender<()>>>,
) -> Tabs {
    let mut tabs = Tabs::default_fill();

    let _ = server_tab::handle(server_rx);
    let _ = client_tab::handle();
    let _ = config_tab::handle(config_filepath, restart_server_tx);
    let _ = config_url_paths_tab::handle(config_url_paths);

    tabs.end();
    tabs.auto_layout();

    tabs
}
