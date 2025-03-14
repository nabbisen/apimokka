use std::{fs, io::Read, sync::Arc};

use fltk::{
    button::Button,
    enums::{CallbackTrigger, Color},
    frame::Frame,
    group::{Flex, Pack, PackType, Scroll},
    output::Output,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextEditor},
};

use tokio::sync::{mpsc::Sender, Mutex};

use crate::app::consts::{
    BUTTON_HEIGHT, BUTTON_WIDTH, CONFIG_EDITOR_HEIGHT, CONTAINER_WIDTH, FLEX_SPACING, LABEL_HEIGHT,
};

/// entry point
pub fn handle(config_filepath: &str, restart_server_tx: Arc<Mutex<Sender<()>>>) -> Flex {
    let tab = Flex::default_fill().with_label("Config\t\t").row();
    let mut vpack = Pack::default_fill();
    vpack.set_spacing(FLEX_SPACING);

    let _filepath_label = Frame::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, LABEL_HEIGHT)
        .with_label(config_filepath);

    let scroll = Scroll::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, CONFIG_EDITOR_HEIGHT);

    let mut config_buffer = TextBuffer::default();
    let config_content = file_content(config_filepath);
    config_buffer.append(&config_content);

    let mut editor = TextEditor::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, CONFIG_EDITOR_HEIGHT);
    editor.set_buffer(Some(config_buffer.clone()));
    editor.set_color(Color::from_hex(0xffeecc));

    // todo
    editor.set_trigger(CallbackTrigger::Changed);
    editor.set_callback(move |_| {});

    scroll.end();

    let mut hpack = Pack::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, BUTTON_HEIGHT + FLEX_SPACING);
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(FLEX_SPACING);

    let mut save_and_restart_server_button = Button::default()
        // todo: size
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("Save and Restart");

    let mut output = Output::default().with_size(BUTTON_WIDTH * 3, BUTTON_HEIGHT);
    output.set_readonly(true);
    output.set_color(Color::Blue);
    output.set_text_color(Color::White);

    let save_and_restart_server_config_filepath = config_filepath.to_owned();
    let save_and_restart_server_config_buffer = config_buffer.clone();
    save_and_restart_server_button.set_callback(move |_button| {
        match fs::write(
            save_and_restart_server_config_filepath.as_str(),
            save_and_restart_server_config_buffer.text(),
        ) {
            Ok(_) => (),
            Err(err) => {
                output.set_value(format!("{}", err).as_str());
                return;
            }
        };

        output.set_value("Config saved. Restarting...");

        let restart_server_tx = restart_server_tx.clone();
        let restart_server_output = output.clone();
        tokio::spawn(async move {
            // todo: how to lock
            let _ = restart_server_tx.lock().await.send(()).await;

            restart_server_output.clone().set_value("");
        });
    });

    hpack.end();

    vpack.end();
    tab.end();

    tab
}

/// get file content
fn file_content(config_filepath: &str) -> String {
    let mut filepath_output = Output::default();
    filepath_output
        .append(config_filepath)
        .expect("Failed to show file path");

    let mut file = std::fs::File::open(config_filepath).expect("Failed to open config file");
    let mut read_buffer = String::new();
    let _ = file
        .read_to_string(&mut read_buffer)
        .expect("Failed to read content in file");

    read_buffer
}
