use crate::api::TwitterClient;
use crate::auth::CredentialStore;
use crate::state::CacheStore;
use crate::ui::TimelineWidget;
use anyhow::Result;
use gtk4::glib::{self, clone};
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use libadwaita::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;
    use glib::subclass::InitializingObject;

    #[derive(gtk4::CompositeTemplate, Default)]
    #[template(resource = "/com/github/cawbirdx/window.ui")]
    pub struct CawbirdXWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,

        pub client: RefCell<Option<TwitterClient>>,
        pub cache: RefCell<Option<CacheStore>>,
        pub timeline: RefCell<Option<TimelineWidget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CawbirdXWindow {
        const NAME: &'static str = "CawbirdXWindow";
        type Type = super::CawbirdXWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_css_name("cawbirdx-window");
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CawbirdXWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_ui();
        }
    }

    impl WidgetImpl for CawbirdXWindow {}
    impl WindowImpl for CawbirdXWindow {}
    impl ApplicationWindowImpl for CawbirdXWindow {}
    impl AdwApplicationWindowImpl for CawbirdXWindow {}
}

glib::wrapper! {
    pub struct CawbirdXWindow(ObjectSubclass<imp::CawbirdXWindow>)
        @extends gtk4::Widget, gtk4::Window, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CawbirdXWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn setup_ui(&self) {
        // Set window properties
        self.set_title(Some("CawbirdX"));
        self.set_default_size(1024, 768);

        // Setup header bar
        let title_label = gtk4::Label::builder()
            .label("CawbirdX")
            .css_classes(vec!["title-label".to_string()])
            .build();
        self.imp().header_bar.set_title_widget(Some(&title_label));

        // Create main content box
        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();

        // Create toolbar
        let toolbar = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .margin_start(8)
            .margin_end(8)
            .margin_top(8)
            .margin_bottom(8)
            .build();

        let refresh_btn = gtk4::Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Refresh Timeline")
            .build();

        let compose_btn = gtk4::Button::builder()
            .icon_name("document-edit-symbolic")
            .tooltip_text("Compose Tweet")
            .build();

        let search_entry = gtk4::Entry::builder()
            .placeholder_text("Search tweets...")
            .hexpand(true)
            .build();

        let search_btn = gtk4::Button::builder()
            .icon_name("system-search-symbolic")
            .tooltip_text("Search")
            .build();

        toolbar.append(&refresh_btn);
        toolbar.append(&compose_btn);
        toolbar.append(&gtk4::Separator::new(gtk4::Orientation::Vertical));
        toolbar.append(&search_entry);
        toolbar.append(&search_btn);

        // Create timeline
        let timeline = TimelineWidget::new();
        content.append(&toolbar);
        content.append(timeline.widget());

        // Set as content
        self.set_content(Some(&content));

        // Store references
        self.imp().timeline.replace(Some(timeline));

        // Connect signals
        refresh_btn.connect_clicked(clone!(@weak self as window => move |_| {
            glib::MainContext::default().spawn_local(clone!(@weak window => async move {
                let _ = window.refresh_timeline().await;
            }));
        }));

        compose_btn.connect_clicked(clone!(@weak self as window => move |_| {
            window.show_compose_dialog();
        }));

        search_btn.connect_clicked(clone!(@weak self as window, @weak search_entry => move |_| {
            let query = search_entry.text().to_string();
            if !query.is_empty() {
                glib::MainContext::default().spawn_local(clone!(@weak window => async move {
                    let _ = window.search_tweets(&query).await;
                }));
            }
        }));

        // Initialize on first show
        self.connect_show(|window| {
            glib::MainContext::default().spawn_local(clone!(@weak window => async move {
                let _ = window.initialize().await;
            }));
        });
    }

    async fn initialize(&self) -> Result<()> {
        // Check for credentials
        if !CredentialStore::has_credentials() {
            self.show_credentials_dialog();
            return Ok(());
        }

        // Create API client
        let api_key = CredentialStore::get_api_key()?;
        let api_host = CredentialStore::get_api_host()?;

        let client = TwitterClient::new(api_key, api_host);
        self.imp().client.replace(Some(client));

        // Initialize cache
        let cache_dir = glib::user_cache_dir()
            .join("cawbirdx")
            .join("cache.mdb");
        std::fs::create_dir_all(cache_dir.parent().unwrap()).ok();
        let cache = CacheStore::open(&cache_dir)?;
        self.imp().cache.replace(Some(cache));

        // Load initial timeline
        self.refresh_timeline().await?;

        Ok(())
    }

    async fn refresh_timeline(&self) -> Result<()> {
        let imp = self.imp();
        let client = imp.client.borrow();
        let client = client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not initialized"))?;

        let tweets = client.get_timeline(Some(50)).await?;

        if let Some(timeline) = imp.timeline.borrow().as_ref() {
            timeline.set_tweets(tweets);
        }

        Ok(())
    }

    async fn search_tweets(&self, query: &str) -> Result<()> {
        let imp = self.imp();
        let client = imp.client.borrow();
        let client = client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not initialized"))?;

        let tweets = client.search(query, Some(50)).await?;

        if let Some(timeline) = imp.timeline.borrow().as_ref() {
            timeline.set_tweets(tweets);
        }

        Ok(())
    }

    fn show_credentials_dialog(&self) {
        let dialog = gtk4::MessageDialog::builder()
            .message_type(gtk4::MessageType::Question)
            .buttons(gtk4::ButtonsType::OkCancel)
            .text("Setup API Credentials")
            .secondary_text("Enter your RapidAPI credentials to use CawbirdX")
            .modal(true)
            .transient_for(self)
            .build();

        let api_key_entry = gtk4::Entry::builder()
            .placeholder_text("RapidAPI Key")
            .build();

        let api_host_entry = gtk4::Entry::builder()
            .placeholder_text("API Host (e.g., twitter241.p.rapidapi.com)")
            .text("twitter241.p.rapidapi.com")
            .build();

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        content.append(&gtk4::Label::builder()
            .label("RapidAPI Key:")
            .halign(gtk4::Align::Start)
            .build());
        content.append(&api_key_entry);

        content.append(&gtk4::Label::builder()
            .label("API Host:")
            .halign(gtk4::Align::Start)
            .build());
        content.append(&api_host_entry);

        dialog.content_area().append(&content);

        dialog.connect_response(clone!(@weak self as window, @weak api_key_entry, @weak api_host_entry => move |dialog, response| {
            if response == gtk4::ResponseType::Ok {
                let key = api_key_entry.text().to_string();
                let host = api_host_entry.text().to_string();

                if !key.is_empty() && !host.is_empty() {
                    if let Err(e) = CredentialStore::set_api_key(&key) {
                        eprintln!("Failed to save API key: {}", e);
                    }
                    if let Err(e) = CredentialStore::set_api_host(&host) {
                        eprintln!("Failed to save API host: {}", e);
                    }

                    glib::MainContext::default().spawn_local(clone!(@weak window => async move {
                        let _ = window.initialize().await;
                    }));
                }
            }
            dialog.close();
        }));

        dialog.show();
    }

    fn show_compose_dialog(&self) {
        let dialog = gtk4::MessageDialog::builder()
            .message_type(gtk4::MessageType::Question)
            .buttons(gtk4::ButtonsType::OkCancel)
            .text("Compose Tweet")
            .modal(true)
            .transient_for(self)
            .build();

        let text_view = gtk4::TextView::builder()
            .wrap_mode(gtk4::WrapMode::WordChar)
            .build();

        let buffer = text_view.buffer();
        let char_count_label = gtk4::Label::builder()
            .label("280")
            .halign(gtk4::Align::End)
            .css_classes(vec!["char-count".to_string()])
            .build();

        buffer.connect_changed(clone!(@weak char_count_label => move |buf| {
            let (start, end) = buf.bounds();
            let text = buf.text(&start, &end, false);
            let count = text.chars().count();
            let remaining = 280_i32.saturating_sub(count as i32);
            char_count_label.set_label(&remaining.to_string());
        }));

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(8)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        content.append(&text_view);
        content.append(&char_count_label);

        dialog.content_area().append(&content);

        dialog.connect_response(clone!(@weak self as window, @weak text_view => move |dialog, response| {
            if response == gtk4::ResponseType::Ok {
                let buffer = text_view.buffer();
                let (start, end) = buffer.bounds();
                let text = buffer.text(&start, &end, false).trim().to_string();

                if !text.is_empty() {
                    glib::MainContext::default().spawn_local(clone!(@weak window => async move {
                        let _ = window.post_tweet(&text).await;
                    }));
                }
            }
            dialog.close();
        }));

        dialog.show();
    }

    async fn post_tweet(&self, text: &str) -> Result<()> {
        let imp = self.imp();
        let client = imp.client.borrow();
        let client = client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not initialized"))?;

        let _tweet = client.post_tweet(text).await?;

        // Refresh timeline after posting
        self.refresh_timeline().await?;

        Ok(())
    }
}

mod app_imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct CawbirdXApp {
        window: RefCell<Option<CawbirdXWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CawbirdXApp {
        const NAME: &'static str = "CawbirdXApp";
        type Type = super::CawbirdXApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for CawbirdXApp {
        fn constructed(&self) {
            self.parent_constructed();
            self.setup_actions();
        }
    }

    impl ApplicationImpl for CawbirdXApp {
        fn activate(&self) {
            // Ensure we have a window
            let window = {
                let window_ref = self.window.borrow();
                if let Some(window) = window_ref.as_ref() {
                    window.clone()
                } else {
                    drop(window_ref);
                    let window = CawbirdXWindow::new();
                    self.window.borrow_mut().replace(window.clone());
                    window
                }
            };

            // Present window
            window.present();
        }
    }

    impl GtkApplicationImpl for CawbirdXApp {}
    impl AdwApplicationImpl for CawbirdXApp {}

    impl CawbirdXApp {
        fn setup_actions(&self) {
            // Quit action
            let quit_action = gio::SimpleAction::new("quit", None);
            let app = self.obj().clone().upcast::<gtk4::Application>();
            quit_action.connect_activate(move |_, _| {
                app.quit();
            });
            self.obj().add_action(&quit_action);

            // Add accelerator for Ctrl+Q
            self.obj().set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }
}

glib::wrapper! {
    pub struct CawbirdXApp(ObjectSubclass<app_imp::CawbirdXApp>)
        @extends gtk4::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CawbirdXApp {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "com.github.cawbirdx")
            .build()
    }
}

impl Default for CawbirdXApp {
    fn default() -> Self {
        Self::new()
    }
}
