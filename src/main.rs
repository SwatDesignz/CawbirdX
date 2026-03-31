mod api;
mod app;
mod auth;
mod state;
mod ui;

use gtk4::prelude::*;
use std::env;

fn main() -> gtk4::glib::ExitCode {
    // Initialize logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Create a new application
    let app = app::CawbirdXApp::default();
    app.upcast::<gtk4::Application>().run()
}