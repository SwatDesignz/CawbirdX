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

    // Load CSS
    let provider = gtk4::CssProvider::new();
    provider.load_from_resource("/com/github/cawbirdx/style.css");
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Failed to get display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Set application icon
    gtk4::IconTheme::for_display(&gtk4::gdk::Display::default().expect("Failed to get display"))
        .add_resource_path("/com/github/cawbirdx/icons");

    // Create a new application
    let app = app::CawbirdXApp::default();
    app.upcast::<gtk4::Application>().run()
}