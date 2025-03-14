use std::{fs, sync::Arc};

use fltk::{
    enums::Color,
    frame::Frame,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use tokio::sync::{mpsc::channel, Mutex};

use apimock::server;

use super::{
    consts::{CONTAINER_HEIGHT, CONTAINER_WIDTH, DEFAULT_CONFIG_FILEPATH, DEV_CONFIG_FILEPATH},
    tabs,
};

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

    // connector
    let (server_proc_tx, server_proc_rx) = channel::<String>(255);
    let server_proc_rx = Arc::new(Mutex::new(server_proc_rx));
    let (restart_server_tx, mut restart_server_rx) = channel::<()>(1);
    let restart_server_tx = Arc::new(Mutex::new(restart_server_tx));

    // server process
    let server_start_config_filepath = config_filepath.to_owned();
    let start_server_proc_tx = server_proc_tx.clone();
    let mut handle = tokio::spawn(async move {
        let apimock = server(
            server_start_config_filepath.as_str(),
            start_server_proc_tx,
            true,
        )
        .await;
        let _ = apimock.start().await;
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
                        let apimock = server(
                            restart_server_config_filepath.as_str(),
                            server_proc_tx,
                            true,
                        )
                        .await;
                        let _ = apimock.start().await;
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

    let _ = tabs::handle(config_filepath, server_proc_rx, restart_server_tx);

    window.make_resizable(true);
    window.end();
    window.show();

    window
}

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
