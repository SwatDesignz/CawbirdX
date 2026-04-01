use crate::api::User;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct UserProfileWidget {
        pub(super) user: RefCell<Option<User>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UserProfileWidget {
        const NAME: &'static str = "UserProfileWidget";
        type Type = super::UserProfileWidget;
        type ParentType = gtk4::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("user-profile-widget");
        }
    }

    impl ObjectImpl for UserProfileWidget {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_ui();
        }
    }

    impl WidgetImpl for UserProfileWidget {}
    impl BoxImpl for UserProfileWidget {}
}

glib::wrapper! {
    pub struct UserProfileWidget(ObjectSubclass<imp::UserProfileWidget>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Orientable;
}

impl UserProfileWidget {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn setup_ui(&self) {
        self.set_orientation(gtk4::Orientation::Vertical);
        self.set_spacing(12);
        self.set_margin_start(16);
        self.set_margin_end(16);
        self.set_margin_top(16);
        self.set_margin_bottom(16);

        // Avatar
        let avatar = gtk4::Image::builder()
            .width_request(80)
            .height_request(80)
            .css_classes(vec!["profile-avatar".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        // Name
        let name_label = gtk4::Label::builder()
            .css_classes(vec!["profile-name".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        // Username
        let username_label = gtk4::Label::builder()
            .css_classes(vec!["profile-username".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        // Stats
        let stats_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(24)
            .halign(gtk4::Align::Center)
            .build();

        let followers_label = gtk4::Label::builder()
            .css_classes(vec!["profile-stat".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        let following_label = gtk4::Label::builder()
            .css_classes(vec!["profile-stat".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        stats_box.append(&followers_label);
        stats_box.append(&gtk4::Separator::new(gtk4::Orientation::Vertical));
        stats_box.append(&following_label);

        // Verified badge
        let verified_badge = gtk4::Image::builder()
            .icon_name("emblem-ok-symbolic")
            .css_classes(vec!["verified-badge".to_string()])
            .halign(gtk4::Align::Center)
            .build();

        self.append(&avatar);
        self.append(&name_label);
        self.append(&username_label);
        self.append(&verified_badge);
        self.append(&stats_box);
    }

    pub fn set_user(&self, user: Option<User>) {
        let imp = self.imp();
        imp.user.replace(user);
        self.update_ui();
    }

    fn update_ui(&self) {
        let imp = self.imp();
        if let Some(_user) = imp.user.borrow().as_ref() {
            // Update UI with user data
            // This will be populated when we have the actual widget references
        }
    }
}
