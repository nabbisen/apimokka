use std::{fs, path::Path, sync::Arc};

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
    consts::{CONTAINER_HEIGHT, CONTAINER_WIDTH, DEFAULT_CONFIG_FILEPATH, DEV_CONFIG_FILEPATH},
    tabs,
};

/// entry point
pub fn handle() -> Window {
    let mut env_args = EnvArgs::init_with_default();

    // todo: integrate to apimock ? env vars are used only in apimock ?
    let config_filepath = match config_filepath() {
        Ok(x) => x,
        Err(err) => {
            return error_window(err.as_str());
        }
    };
    env_args.config_filepath = Some(config_filepath.to_owned());

    // todo: get middleware file location and validate it
    //       temporary values below ?
    let middleware_filepath_str = (if cfg!(debug_assertions) {
        "tests/fixtures/middleware.rhai"
    } else {
        "middleware.rhai"
    })
    .to_owned();
    let middleware_filepath = if Path::new(middleware_filepath_str.as_str()).exists() {
        Some(middleware_filepath_str)
    } else {
        None
    };
    env_args.middleware_filepath = middleware_filepath;

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

    let config_url_paths = match config_url_paths_rx.recv() {
        Ok(x) => x,
        Err(err) => {
            return error_window(format!("failed to receive config url paths - {}", err).as_str())
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

/// error window (instead of app window)
fn error_window(msg: &str) -> Window {
    let window = Window::default().with_size(640, 400);
    let mut frame = Frame::default_fill().with_label(msg);
    frame.set_color(Color::Yellow);
    window.end();
    window
}
