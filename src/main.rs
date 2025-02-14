use fltk::{
    app::{App, Scheme},
    button::Button,
    enums::{CallbackTrigger, Color},
    group::*,
    input::Input,
    prelude::*,
    window::Window,
};

use apimock::{core::config::config_path, server};
use tokio::sync::mpsc::channel;

// consts
const INPUT_OUTPUT_WIDTH: i32 = 360;
const INPUT_HEIGHT: i32 = 30;
const OUTPUT_HEIGHT: i32 = 40;
// const STYLE_TABLE: [StyleTableEntryExt; 3] = [
//     StyleTableEntryExt {
//         color: Color::from_hex(0x000000),
//         font: Font::Courier,
//         size: 16,
//         attr: TextAttr::None,
//         bgcolor: Color::TransparentBg,
//     },
// ];

#[tokio::main]
async fn main() {
    let (server_tx, mut server_rx) = channel::<String>(255);

    let _ = tokio::spawn(async move {
        loop {
            match server_rx.recv().await {
                Some(message) => {
                    println!("Received message: {}", message);
                }
                None => {
                    println!("Receiver closed");
                    break;
                }
            }
        }
    });
    let handle = tokio::spawn(async move {
        let apimock = server(config_path().as_str(), server_tx).await;
        let _ = apimock.start().await;
    });

    let (shutdown_tx, mut shutdown_rx) = channel::<()>(1);
    let _ = tokio::spawn(async move {
        loop {
            match shutdown_rx.recv().await {
                Some(_) => {
                    println!("Stopping http server");
                    handle.abort();
                }
                None => {
                    println!("Receiver closed");
                    break;
                }
            }
        }
    });

    let app = App::default().with_scheme(Scheme::Gtk);

    let mut window = Window::default()
        .with_size(
            INPUT_OUTPUT_WIDTH + 28,
            INPUT_HEIGHT * 2 + OUTPUT_HEIGHT * 2 + 28,
        )
        .with_label("API mimic");
    window.set_color(Color::White);

    let mut vpack = Pack::new(
        4,
        4,
        INPUT_OUTPUT_WIDTH + 20,
        INPUT_HEIGHT * 2 + OUTPUT_HEIGHT * 2 + 20,
        "",
    );
    vpack.set_spacing(4);
    let mut source_input = Input::default().with_size(INPUT_OUTPUT_WIDTH, INPUT_HEIGHT);
    source_input.set_color(Color::from_hex(0xffeecc));

    let mut button = Button::default().with_size(INPUT_OUTPUT_WIDTH, INPUT_HEIGHT);

    vpack.end();

    // wind.make_resizable(true);
    window.end();
    window.show();

    source_input.set_trigger(CallbackTrigger::Changed);
    source_input.set_callback(move |_| {});

    button.set_callback(move |_button| {
        println!("clicked.");
        let shutdown_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = shutdown_tx.send(()).await;
        });
    });

    app.run().unwrap();
}
