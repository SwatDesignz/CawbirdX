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

    // Register resources
    gtk4::gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Create a new application
    let app = app::CawbirdXApp::default();
    app.upcast::<gtk4::Application>().run()
}