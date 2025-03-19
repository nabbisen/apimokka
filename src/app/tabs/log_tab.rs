use std::process::Command;

use fltk::{
    button::Button,
    enums::{Color, Event},
    group::{Flex, FlexType},
    input::Input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    terminal::Terminal,
};

use crate::app::consts::{BUTTON_HEIGHT, CONTAINER_WIDTH, FLEX_SPACING, LOG_TERMINAL_HEIGHT};

/// entry point
pub fn handle() -> Flex {
    let tab = Flex::default_fill().with_label("Log\t\t").row();
    // let mut log_buffer = TextBuffer::default();
    // let mut text_display = TextDisplay::default_fill();
    let mut vflex = Flex::default_fill();
    vflex.set_spacing(FLEX_SPACING);
    vflex.set_type(FlexType::Column);

    let mut terminal = Terminal::default();
    terminal.set_size(CONTAINER_WIDTH, LOG_TERMINAL_HEIGHT);
    terminal.set_ansi(true);
    terminal.set_color(Color::Blue);

    terminal.handle(|stuff, event| match event {
        Event::Push => {
            fltk::app::copy(stuff.selection_text().as_str());
            stuff.clear_mouse_selection();
            true
        }
        _ => false,
    });

    // text_display.set_color(Color::Dark3);
    // text_display.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
    // text_display.set_linenumber_width(LINE_NUMBER_WIDTH);
    // text_display.set_linenumber_size(LINE_NUMBER_WIDTH - 3);
    // text_display.set_buffer(Some(log_buffer.clone()));

    let flex = Flex::new(0, 0, CONTAINER_WIDTH, BUTTON_HEIGHT * 2, "");

    let mut command_line_input = Input::default();
    command_line_input.set_size(CONTAINER_WIDTH, BUTTON_HEIGHT);
    let mut command_button = Button::default().with_size(0, BUTTON_HEIGHT);
    command_button.set_label("run command");
    command_button.set_callback(move |_| command_button_onclick(&command_line_input, &terminal));

    flex.end();

    vflex.fixed(&flex, BUTTON_HEIGHT * 2);
    vflex.end();
    tab.end();

    tab
}

fn command_button_onclick(command_line_input: &Input, terminal: &Terminal) {
    let command = command_line_input.clone().value();

    let terminal = terminal.clone();
    let _ = tokio::spawn(async move {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                // utf-8 specified
                .args(&["/C", "chcp 65001"])
                .output()
                .expect("Failed to run chcp command");
            Command::new("cmd").args(&["/C", &command]).output()
        } else {
            Command::new("sh").args(&["-c", &command]).output()
        }
        .expect("Failed to run command");
        let s = format!(
            "$ {} ->\n{}\n",
            command,
            String::from_utf8_lossy(&output.stdout).to_string()
        );
        // println!(
        //     "xxx {:?}",
        //     String::from_utf8_lossy(&output.stdout).to_string()
        // );
        terminal.clone().append(s.as_str());
    });
}
