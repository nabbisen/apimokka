use std::{io::Read, sync::Arc};

use fltk::{
    button::Button,
    enums::{CallbackTrigger, Color},
    group::{Flex, Pack, Scroll},
    output::Output,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextEditor},
};

use tokio::sync::{mpsc::Sender, Mutex};

use super::super::consts::{BUTTON_HEIGHT, CONTAINER_HEIGHT, CONTAINER_WIDTH};

pub fn handle(config_filepath: &str, shutdown_tx: Arc<Mutex<Sender<()>>>) -> Flex {
    let tab = Flex::default_fill().with_label("Config\t\t").row();
    let scroll = Scroll::default_fill();
    let mut vpack = Pack::default_fill();
    vpack.set_spacing(10);

    let mut filepath_output = Output::default();
    filepath_output
        .append(config_filepath)
        .expect("Failed to show file path");

    let mut file = std::fs::File::open(config_filepath).expect("Failed to open config file");
    let mut read_buffer = String::new();
    let _ = file
        .read_to_string(&mut read_buffer)
        .expect("Failed to read content in file");

    let mut config_buffer = TextBuffer::default();
    config_buffer.append(&read_buffer);
    let mut editor = TextEditor::default();
    editor.set_size(CONTAINER_WIDTH, CONTAINER_HEIGHT - BUTTON_HEIGHT * 3 - 20);
    editor.set_buffer(Some(config_buffer));
    editor.set_color(Color::from_hex(0xffeecc));

    // todo
    editor.set_trigger(CallbackTrigger::Changed);
    editor.set_callback(move |_| {});

    let mut stop_server_button = Button::default().with_size(0, BUTTON_HEIGHT);
    stop_server_button.set_label("stop server");

    stop_server_button.set_callback(move |_button| {
        println!("clicked.");
        let shutdown_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            // todo: how to lock
            let _ = shutdown_tx.lock().await.send(()).await;
        });
    });

    vpack.end();
    scroll.end();
    tab.end();

    tab
}
