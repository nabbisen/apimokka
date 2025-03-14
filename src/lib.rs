use app::window;
use fltk::app::{App, Scheme};

mod app;

/// entry point
pub async fn run() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let _ = window::handle();
    app.run().unwrap();
}
