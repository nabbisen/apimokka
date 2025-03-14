use std::sync::Arc;

use fltk::{
    app,
    enums::Event,
    group::Flex,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    terminal::Terminal,
};
use tokio::sync::{mpsc::Receiver, Mutex};

pub fn handle(server_proc_tx: Arc<Mutex<Receiver<String>>>) -> Flex {
    let tab = Flex::default_fill().with_label("Server\t\t").row();

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

    tab.end();

    tab
}
