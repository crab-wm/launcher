pub(crate) mod imp;

use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use crate::crab_tabs::imp::CrabTab;

glib::wrapper! {
    pub struct CrabTabs(ObjectSubclass<imp::CrabTabs>)
    @extends gtk::Box, gtk::Widget;
}

impl Default for CrabTabs {
    fn default() -> Self {
        Self::new()
    }
}

impl CrabTabs {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create CrabTabs")
    }

    fn setup_tabs(&self) {
        self.imp().tab_programs.add_css_class("active-tab");
    }

    pub fn change_tab(&self) {
        if self.imp().tab_music.has_css_class("active-tab") {
            self.imp().tab_music.remove_css_class("active-tab");
            self.imp().tab_programs.add_css_class("active-tab");
            self.set_property("current-tab", CrabTab::Programs.value());
        } else if self.imp().tab_programs.has_css_class("active-tab") {
            self.imp().tab_programs.remove_css_class("active-tab");
            self.imp().tab_music.add_css_class("active-tab");
            self.set_property("current-tab", CrabTab::Music.value());
        }
    }
}
