pub(crate) mod imp;

use crate::crab_tabs::imp::CrabTab;
use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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
        Object::new(&[("current-tab", &0.to_value())]).expect("Failed to create CrabTabs")
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
