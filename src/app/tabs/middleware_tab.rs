use std::io::Read;

use fltk::{
    button::Button,
    enums::{CallbackTrigger, Color},
    frame::Frame,
    group::{Flex, FlexType},
    output::Output,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextEditor},
};

use crate::app::consts::{
    BUTTON_HEIGHT, BUTTON_WIDTH, CONFIG_EDITOR_HEIGHT, CONTAINER_WIDTH, FLEX_SPACING, LABEL_HEIGHT,
};

/// entry point
pub fn handle(middleware_filepath: &str) -> Flex {
    let tab = Flex::default_fill().with_label("Middleware\t\t").row();

    let mut vflex = Flex::default_fill();
    vflex.set_spacing(FLEX_SPACING);
    vflex.set_type(FlexType::Column);

    let filepath_label = Frame::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, LABEL_HEIGHT)
        .with_label(middleware_filepath);
    // let mut filepath_output = Output::default();
    // filepath_output
    //     .append(config_filepath)
    //     .expect("Failed to show file path");

    // let scroll = Scroll::default()
    //     // todo: size
    //     .with_size(CONTAINER_WIDTH, CONFIG_EDITOR_HEIGHT);

    let mut config_buffer = TextBuffer::default();
    let config_content = file_content(middleware_filepath);
    config_buffer.append(&config_content);

    let mut editor = TextEditor::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, CONFIG_EDITOR_HEIGHT);
    editor.set_color(Color::from_hex(0xffeecc));
    editor.set_linenumber_width(16);
    editor.set_linenumber_size(10);
    editor.set_buffer(config_buffer);

    // todo
    editor.set_trigger(CallbackTrigger::Changed);
    editor.set_callback(move |_| {});

    // scroll.end();

    let mut flex = Flex::default()
        // todo: size
        .with_size(CONTAINER_WIDTH, BUTTON_HEIGHT + FLEX_SPACING);
    flex.set_spacing(FLEX_SPACING);

    // todo
    let mut save_and_restart_server_button = Button::default()
        // todo: size
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("Save and Restart (todo)");
    // todo: save middleware file
    save_and_restart_server_button.set_label_color(Color::Inactive);

    let mut output = Output::default().with_size(BUTTON_WIDTH * 3, BUTTON_HEIGHT);
    output.set_readonly(true);
    output.set_color(Color::Blue);
    output.set_text_color(Color::White);

    // todo
    let save_and_restart_server_middleware_filepath = middleware_filepath.to_owned();
    save_and_restart_server_button.set_callback(move |_button| {
        println!("{}", save_and_restart_server_middleware_filepath);
    });

    flex.end();

    vflex.fixed(&filepath_label, LABEL_HEIGHT);
    vflex.fixed(&flex, BUTTON_HEIGHT);
    vflex.end();
    tab.end();

    tab
}

/// get file content
fn file_content(config_filepath: &str) -> String {
    let mut file = std::fs::File::open(config_filepath).expect("Failed to open file");
    let mut read_buffer = String::new();
    let _ = file
        .read_to_string(&mut read_buffer)
        .expect("Failed to read content in file");

    read_buffer
}
