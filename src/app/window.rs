use std::sync::Arc;

use fltk::{
    enums::Color,
    frame::Frame,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use tokio::sync::{mpsc::channel, Mutex};

use apimock::{
    core::{args::EnvArgs, config::ConfigUrlPaths},
    server,
};

use super::{
    consts::{CONTAINER_HEIGHT, CONTAINER_WIDTH},
    tabs,
};

/// entry point
pub fn handle() -> Window {
    let env_args = EnvArgs::init_with_default();

    // server connector
    // - default
    let (server_proc_tx, server_proc_rx) = channel::<String>(255);
    let server_proc_rx = Arc::new(Mutex::new(server_proc_rx));
    // - order to restart
    let (restart_server_tx, mut restart_server_rx) = channel::<()>(1);
    let restart_server_tx = Arc::new(Mutex::new(restart_server_tx));
    // - shared config url paths
    let (config_url_paths_tx, config_url_paths_rx) =
        std::sync::mpsc::channel::<Option<ConfigUrlPaths>>();

    // server process
    let start_server_proc_tx = server_proc_tx.clone();
    let start_server_env_args = env_args.clone();
    let mut handle = tokio::spawn(async move {
        let server = server(start_server_env_args, start_server_proc_tx, true).await;

        let _ = config_url_paths_tx.send(server.config_url_paths());

        let _ = server.start().await;
    });

    let restart_server_env_args = env_args.clone();
    let _ = tokio::spawn(async move {
        loop {
            match restart_server_rx.recv().await {
                Some(_) => {
                    // stop server
                    handle.abort();

                    // restart server
                    let restart_server_env_args = restart_server_env_args.clone();
                    let server_proc_tx = server_proc_tx.clone();
                    handle = tokio::spawn(async move {
                        let server = server(restart_server_env_args, server_proc_tx, true).await;
                        let _ = server.start().await;
                    });
                }
                None => {
                    // todo: printed out below once when client tab button clicked after server started ?
                    // println!("Receiver closed");
                    break;
                }
            }
        }
    });

    let mut window = Window::default()
        .with_size(CONTAINER_WIDTH, CONTAINER_HEIGHT)
        .with_label("API mokka");
    window.set_color(Color::White);

    let config_url_paths = match config_url_paths_rx.recv() {
        Ok(x) => x,
        Err(err) => {
            return error_window(format!("failed to receive config url paths -\n{}", err).as_str())
        }
    };

    let _ = tabs::handle(
        &env_args,
        config_url_paths,
        server_proc_rx,
        restart_server_tx,
    );

    window.make_resizable(true);
    window.end();
    window.show();

    window
}

/// error window (instead of app window)
fn error_window(msg: &str) -> Window {
    let mut window = Window::default().with_size(640, 400);
    let mut frame = Frame::default_fill().with_label(msg);
    frame.set_color(Color::Yellow);
    window.end();
    window.show();
    window
}
