use app::window;
use fltk::app::{App, Scheme};

mod app;
mod core;

#[tokio::main]
async fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let _ = window::handle();
    app.run().unwrap();
}
