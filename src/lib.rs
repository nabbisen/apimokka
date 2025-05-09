use app::window;
use fltk::app::{App, Scheme};

#[cfg(feature = "napi")]
use napi_derive::napi;

mod app;

/// entry point
pub async fn run() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let _ = window::handle();
    app.run().unwrap();
}

/// node.js binding entry point
#[cfg(feature = "napi")]
#[napi]
pub async fn napi_run() {
    run().await
}
