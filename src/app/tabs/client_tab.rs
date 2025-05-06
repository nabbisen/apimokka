use std::fs;

use chrono::Local;
use fltk::{
    button::Button,
    dialog::{FileDialog, FileDialogAction, FileDialogType},
    enums::{Color, Event},
    group::{Flex, FlexType, Group},
    input::Input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    terminal::Terminal,
};

use crate::app::{
    consts::{BUTTON_HEIGHT, CONTAINER_WIDTH, FLEX_SPACING, LOG_TERMINAL_HEIGHT},
    utils::command_output,
};

const DEFAULT_COMMAND_LINE: &str = "curl -i http://localhost:3001/";

/// entry point
pub fn handle() -> Flex {
    let tab = Flex::default_fill().with_label("Client\t\t").row();

    // container for floating components such as buttons
    let group = Group::default_fill();

    // let mut log_buffer = TextBuffer::default();
    // let mut text_display = TextDisplay::default_fill();

    let mut vflex = Flex::default_fill();
    vflex.set_spacing(FLEX_SPACING);
    vflex.set_type(FlexType::Column);

    let flex = Flex::new(0, 0, CONTAINER_WIDTH, BUTTON_HEIGHT * 2, "");

    let mut command_line_input = Input::default();
    command_line_input.set_size(CONTAINER_WIDTH, BUTTON_HEIGHT);
    command_line_input.set_value(DEFAULT_COMMAND_LINE);
    let mut command_button = Button::default().with_size(0, BUTTON_HEIGHT);
    command_button.set_label("run command");

    flex.end();

    let mut terminal = Terminal::default();
    terminal.set_size(CONTAINER_WIDTH, LOG_TERMINAL_HEIGHT);
    terminal.set_ansi(true);
    terminal.set_color(Color::DarkBlue);

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

    vflex.fixed(&flex, BUTTON_HEIGHT * 2);
    vflex.end();

    // todo: integrate export buttons ?
    let mut export_button = Button::new(group.width() - 100, group.height() - 50, 70, 30, "Export");
    export_button.set_color(Color::White);
    let export_button_terminal = terminal.clone();
    export_button.set_callback(move |_| {
        let mut file_dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
        file_dialog.set_preset_file("apimokka-client.log");
        match file_dialog.try_show() {
            Ok(x) => match x {
                FileDialogAction::Success => (),
                _ => return,
            },
            Err(_) => return,
        }
        let save_as_filepath = file_dialog.filename();
        let save_as_content = export_button_terminal.text(false);
        match fs::write(save_as_filepath, save_as_content) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        }
    });

    command_button.set_callback(move |_| command_button_onclick(&command_line_input, &terminal));

    group.end();

    tab.end();

    tab
}

fn command_button_onclick(command_line_input: &Input, terminal: &Terminal) {
    let command = command_line_input.clone().value();

    let terminal = terminal.clone();
    let _ = tokio::spawn(async move {
        let output = command_output(command.as_str());
        let trailing = match output {
            Ok(output) => output,
            Err(err) => format!("Failed to run command: {}", err),
        };
        let appened = format!(
            "  [{}]\n-------------------------\n$ {}\n\n{}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            command,
            trailing
        );
        terminal.clone().append(appened.as_str());
    });
}
