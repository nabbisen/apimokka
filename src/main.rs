use app::window;
use fltk::app::{App, Scheme};

mod app;
mod core;

#[tokio::main]
async fn main() {
    let config_filepath = "./tests/fixtures/apimock.toml";

    let app = App::default().with_scheme(Scheme::Gtk);
    let _ = window::handle(config_filepath);
    app.run().unwrap();
}
