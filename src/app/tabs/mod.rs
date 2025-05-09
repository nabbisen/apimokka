use std::sync::Arc;

use apimock::core::{
    args::EnvArgs, config::ConfigUrlPaths, constant::config::DEFAULT_LISTENER_PORT,
};
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
mod middleware_tab;
mod server_tab;

pub fn handle(
    env_args: &EnvArgs,
    config_url_paths: Option<ConfigUrlPaths>,
    server_rx: Arc<Mutex<Receiver<String>>>,
    restart_server_tx: Arc<Mutex<Sender<()>>>,
) -> Tabs {
    let mut tabs = Tabs::default_fill();

    let _ = server_tab::handle(server_rx);
    let port = env_args.port.unwrap_or(DEFAULT_LISTENER_PORT);
    let _ = client_tab::handle(port);

    if let Some(config_filepath) = env_args.config_filepath.clone() {
        let _ = config_tab::handle(config_filepath.as_str(), restart_server_tx);
        let _ = config_url_paths_tab::handle(config_url_paths);
    }

    if let Some(middleware_filepath) = env_args.middleware_filepath.clone() {
        let _ = middleware_tab::handle(middleware_filepath.as_str());
    }

    tabs.end();
    tabs.auto_layout();

    tabs
}
