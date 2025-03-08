use std::io::Read;

use fltk::{
    app::{App, Scheme},
    button::Button,
    enums::{CallbackTrigger, Color},
    group::*,
    output::Output,
    prelude::*,
    text::{TextBuffer, TextDisplay, TextEditor},
    window::Window,
};

use apimock::{core::config::config_path, server};
use tokio::sync::mpsc::channel;

mod app;
mod core;

// consts
const INPUT_OUTPUT_WIDTH: i32 = 360;
const INPUT_HEIGHT: i32 = 120;
const OUTPUT_HEIGHT: i32 = 40;
const BUTTON_HEIGHT: i32 = 30;
const LINE_NUMBER_WIDTH: i32 = 14;
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
    // connector
    let (server_tx, mut server_rx) = channel::<String>(255);
    let (shutdown_tx, mut shutdown_rx) = channel::<()>(1);

    // gui
    let app = App::default().with_scheme(Scheme::Gtk);

    let mut window = Window::default()
        .with_size(
            INPUT_OUTPUT_WIDTH + 28,
            INPUT_HEIGHT * 2 + OUTPUT_HEIGHT * 2 + 28,
        )
        .with_label("API mimic");
    window.set_color(Color::White);

    let mut tab = Tabs::default_fill();
    let grp1 = Flex::default_fill().with_label("Log\t\t").row();
    let mut log_buffer = TextBuffer::default();
    let mut text_display = TextDisplay::default_fill();
    text_display.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
    text_display.set_buffer(Some(log_buffer.clone()));
    text_display.set_linenumber_width(LINE_NUMBER_WIDTH);
    text_display.set_linenumber_size(LINE_NUMBER_WIDTH - 3);
    grp1.end();

    let grp2 = Flex::default_fill().with_label("Config\t\t").row();
    let mut col = Flex::default().column();
    col.set_pad(10);
    col.set_margin(10);

    let filepath = "./tests/fixtures/apimock.toml";
    let mut filepath_output = Output::default();
    filepath_output
        .append(filepath)
        .expect("Failed to show file path");

    let mut file = std::fs::File::open(filepath).expect("Failed to open config file");
    let mut read_buffer = String::new();
    let _ = file
        .read_to_string(&mut read_buffer)
        .expect("Failed to read content in file");

    let mut config_buffer = TextBuffer::default();
    config_buffer.append(&read_buffer);
    let mut editor = TextEditor::default();
    editor.set_size(INPUT_OUTPUT_WIDTH, INPUT_HEIGHT);
    editor.set_buffer(Some(config_buffer));
    editor.set_color(Color::from_hex(0xffeecc));

    let mut button = Button::default().with_size(INPUT_OUTPUT_WIDTH, BUTTON_HEIGHT);
    button.set_label("stop server");

    col.end();
    grp2.end();
    tab.end();
    tab.auto_layout();

    window.make_resizable(true);
    window.end();
    window.show();

    editor.set_trigger(CallbackTrigger::Changed);
    editor.set_callback(move |_| {});

    button.set_callback(move |_button| {
        println!("clicked.");
        let shutdown_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = shutdown_tx.send(()).await;
        });
    });

    // server process
    let handle = tokio::spawn(async move {
        let apimock = server(config_path().as_str(), server_tx, false).await;
        let _ = apimock.start().await;
    });

    let _ = tokio::spawn(async move {
        loop {
            match server_rx.recv().await {
                Some(message) => {
                    log_buffer.append(format!("{}\n", message).as_str());
                    // scroll to bottom
                    text_display.scroll(log_buffer.text().split("\n").count() as i32, 0);
                }
                None => {
                    println!("Receiver closed");
                    break;
                }
            }
        }
    });

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

    app.run().unwrap();
}
