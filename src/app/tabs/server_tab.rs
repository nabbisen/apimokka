use std::{fs, sync::Arc};

use fltk::{
    app,
    button::Button,
    dialog::{FileDialog, FileDialogAction, FileDialogType},
    enums::{Color, Event},
    group::{Flex, Group},
    prelude::{GroupExt, WidgetBase, WidgetExt},
    terminal::Terminal,
};
use tokio::sync::{mpsc::Receiver, Mutex};

/// entry point
pub fn handle(server_proc_tx: Arc<Mutex<Receiver<String>>>) -> Flex {
    let tab = Flex::default_fill().with_label("Server\t\t").row();

    // container for floating components such as buttons
    let group = Group::default_fill();

    let mut terminal = Terminal::default_fill();
    terminal.set_ansi(true);
    terminal.handle(|stuff, event| match event {
        Event::Push => {
            app::copy(stuff.selection_text().as_str());
            stuff.clear_mouse_selection();
            true
        }
        _ => false,
    });

    // todo: integrate export buttons ?
    let mut export_button = Button::new(group.width() - 100, group.height() - 50, 70, 30, "Export");
    export_button.set_color(Color::White);
    let export_button_terminal = terminal.clone();
    export_button.set_callback(move |_| {
        let mut file_dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
        file_dialog.set_preset_file("apimokka-server.log");
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

    let _ = tokio::spawn(async move {
        loop {
            // todo: how to lock
            match server_proc_tx.try_lock().unwrap().recv().await {
                Some(message) => {
                    // log_buffer.append(format!("{}\n", message).as_str());
                    // // scroll to bottom
                    // text_display.scroll(log_buffer.text().split("\n").count() as i32, 0);
                    terminal.append(format!("{}\n", message).as_str());
                }
                None => {
                    println!("Receiver closed");
                    break;
                }
            }
        }
    });

    group.end();

    tab.end();

    tab
}
