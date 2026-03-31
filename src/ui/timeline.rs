use crate::api::Tweet;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TweetWidget {
        pub(super) tweet: RefCell<Option<Tweet>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TweetWidget {
        const NAME: &'static str = "TweetWidget";
        type Type = super::TweetWidget;
        type ParentType = gtk4::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("tweet-widget");
        }
    }

    impl ObjectImpl for TweetWidget {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_ui();
        }
    }

    impl WidgetImpl for TweetWidget {}
    impl BoxImpl for TweetWidget {}
}

glib::wrapper! {
    pub struct TweetWidget(ObjectSubclass<imp::TweetWidget>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Orientable;
}

impl TweetWidget {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn setup_ui(&self) {
        self.set_orientation(gtk4::Orientation::Vertical);
        self.set_spacing(8);
        self.set_margin_start(12);
        self.set_margin_end(12);
        self.set_margin_top(8);
        self.set_margin_bottom(8);

        // Header (avatar + name + timestamp)
        let header = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .build();

        let avatar = gtk4::Image::builder()
            .width_request(48)
            .height_request(48)
            .css_classes(vec!["avatar".to_string()])
            .build();

        let name_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_label = gtk4::Label::builder()
            .css_classes(vec!["name-label".to_string()])
            .halign(gtk4::Align::Start)
            .build();

        let username_label = gtk4::Label::builder()
            .css_classes(vec!["username-label".to_string()])
            .halign(gtk4::Align::Start)
            .build();

        let timestamp_label = gtk4::Label::builder()
            .css_classes(vec!["timestamp-label".to_string()])
            .halign(gtk4::Align::End)
            .build();

        name_box.append(&name_label);
        name_box.append(&username_label);

        header.append(&avatar);
        header.append(&name_box);
        header.append(&timestamp_label);

        // Content
        let content_label = gtk4::Label::builder()
            .css_classes(vec!["content-label".to_string()])
            .halign(gtk4::Align::Start)
            .wrap(true)
            .xalign(0.0)
            .build();

        // Actions (like, retweet, reply)
        let actions = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(16)
            .margin_top(8)
            .build();

        let reply_btn = gtk4::Button::builder()
            .icon_name("mail-reply-symbolic")
            .css_classes(vec!["action-btn".to_string()])
            .build();

        let retweet_btn = gtk4::Button::builder()
            .icon_name("media-playlist-repeat-symbolic")
            .css_classes(vec!["action-btn".to_string()])
            .build();

        let like_btn = gtk4::Button::builder()
            .icon_name("starred-symbolic")
            .css_classes(vec!["action-btn".to_string()])
            .build();

        actions.append(&reply_btn);
        actions.append(&retweet_btn);
        actions.append(&like_btn);

        self.append(&header);
        self.append(&content_label);
        self.append(&actions);
    }

    fn update_ui(&self) {
        let imp = self.imp();
        if let Some(_tweet) = imp.tweet.borrow().as_ref() {
            // Update UI with tweet data
            // This will be populated when we have the actual widget references
        }
    }

    pub fn set_tweet(&self, tweet: Option<Tweet>) {
        let imp = self.imp();
        imp.tweet.replace(tweet);
        self.update_ui();
    }
}

/// Timeline widget showing a list of tweets
pub struct TimelineWidget {
    list_box: gtk4::ListBox,
    scrolled_window: gtk4::ScrolledWindow,
}

impl TimelineWidget {
    pub fn new() -> Self {
        let list_box = gtk4::ListBox::builder()
            .css_classes(vec!["timeline-list".to_string()])
            .build();

        let scrolled_window = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&list_box)
            .build();

        Self {
            list_box,
            scrolled_window,
        }
    }

    pub fn widget(&self) -> &gtk4::ScrolledWindow {
        &self.scrolled_window
    }

    pub fn add_tweet(&self, tweet: Tweet) {
        let tweet_widget = TweetWidget::new();
        tweet_widget.set_tweet(Some(tweet));
        self.list_box.append(&tweet_widget);
    }

    pub fn clear(&self) {
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }
    }

    pub fn set_tweets(&self, tweets: Vec<Tweet>) {
        self.clear();
        for tweet in tweets {
            self.add_tweet(tweet);
        }
    }
}

impl Default for TimelineWidget {
    fn default() -> Self {
        Self::new()
    }
}
