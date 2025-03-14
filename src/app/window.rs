use std::sync::Arc;

use fltk::{
    enums::Color,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};
use tokio::sync::{mpsc::channel, Mutex};

use apimock::server;

use super::{
    consts::{CONTAINER_HEIGHT, CONTAINER_WIDTH},
    tabs,
};

pub fn handle(config_filepath: &str) -> Window {
    // connector
    let (server_tx, server_rx) = channel::<String>(255);
    let server_rx = Arc::new(Mutex::new(server_rx));
    let (shutdown_tx, mut shutdown_rx) = channel::<()>(1);
    let shutdown_tx = Arc::new(Mutex::new(shutdown_tx));

    // server process
    let server_start_config_filepath = config_filepath.to_owned();
    let mut handle = tokio::spawn(async move {
        let apimock = server(server_start_config_filepath.as_str(), server_tx, true).await;
        let _ = apimock.start().await;
    });

    let _ = tokio::spawn(async move {
        loop {
            match shutdown_rx.recv().await {
                Some(_) => {
                    println!("Stopping http server");
                    handle.abort();

                    // handle = tokio::spawn(async move {
                    //     let apimock = server(config_filepath, server_tx, true).await;
                    //     let _ = apimock.start().await;
                    // });
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
        .with_label("API mimic");
    window.set_color(Color::White);

    let _ = tabs::handle(config_filepath, shutdown_tx, server_rx);

    window.make_resizable(true);
    window.end();
    window.show();

    window
}
