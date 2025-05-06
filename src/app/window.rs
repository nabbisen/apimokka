use std::{fs, sync::Arc};

use fltk::{
    enums::Color,
    frame::Frame,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use tokio::sync::{mpsc::channel, Mutex};

use apimock::{core::config::ConfigUrlPaths, server};

use super::{
    consts::{CONTAINER_HEIGHT, CONTAINER_WIDTH, DEFAULT_CONFIG_FILEPATH, DEV_CONFIG_FILEPATH},
    tabs,
};

/// entry point
pub fn handle() -> Window {
    let config_filepath = match config_filepath() {
        Ok(x) => x,
        Err(err) => {
            let window = Window::default_fill();
            let _frame = Frame::default_fill().with_label(err.as_str());
            window.end();
            return window;
        }
    };

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
    let mut handle = tokio::spawn(async move {
        let server = server(
            config_filepath,
            // todo: middleware
            None,
            start_server_proc_tx,
            true,
        )
        .await;

        let _ = config_url_paths_tx.send(server.config_url_paths());

        let _ = server.start().await;
    });

    let restart_server_config_filepath = config_filepath.to_owned();
    let _ = tokio::spawn(async move {
        loop {
            match restart_server_rx.recv().await {
                Some(_) => {
                    // stop server
                    handle.abort();

                    // restart server
                    let restart_server_config_filepath = restart_server_config_filepath.clone();
                    let server_proc_tx = server_proc_tx.clone();
                    handle = tokio::spawn(async move {
                        let server = server(
                            restart_server_config_filepath.as_str(),
                            // todo: middleware
                            None,
                            server_proc_tx,
                            true,
                        )
                        .await;
                        let _ = server.start().await;
                    });
                }
                None => {
                    println!("Receiver closed");
                    break;
                }
            }
        }
    });

    let mut window = Window::default()
        .with_size(CONTAINER_WIDTH, CONTAINER_HEIGHT)
        .with_label("API mokka");
    window.set_color(Color::White);

    let config_url_paths = config_url_paths_rx
        .recv()
        .expect("failed to receive config url paths");
    let _ = tabs::handle(
        config_filepath,
        config_url_paths,
        server_proc_rx,
        restart_server_tx,
    );

    window.make_resizable(true);
    window.end();
    window.show();

    window
}

/// detect config file
fn config_filepath<'a>() -> Result<&'a str, String> {
    for x in [DEFAULT_CONFIG_FILEPATH, DEV_CONFIG_FILEPATH] {
        if fs::metadata(x).is_ok_and(|x| x.is_file()) {
            return Ok(x);
        }
    }
    Err(format!(
        "Config file is missing: either `{}` or `{}` is required",
        DEFAULT_CONFIG_FILEPATH, DEV_CONFIG_FILEPATH
    ))
}
