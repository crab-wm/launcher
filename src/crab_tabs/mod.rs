use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::crab_tabs::imp::CrabTab;

pub mod imp;

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

    pub fn change_tab(&self, tab: Option<CrabTab>) {
        if let Some(tab) = tab {
            match tab {
                CrabTab::Programs => {
                    self.imp().tab_music.remove_css_class("active-tab");
                    self.imp().tab_programs.add_css_class("active-tab");
                    self.set_property("current-tab", CrabTab::Programs.to_value());
                }
                CrabTab::Music => {
                    self.imp().tab_programs.remove_css_class("active-tab");
                    self.imp().tab_music.add_css_class("active-tab");
                    self.set_property("current-tab", CrabTab::Music.to_value());
                }
            };

            return;
        }

        match self.get_current_tab() {
            CrabTab::Programs => {
                self.imp().tab_programs.remove_css_class("active-tab");
                self.imp().tab_music.add_css_class("active-tab");
                self.set_property("current-tab", CrabTab::Music.to_value());
            }
            CrabTab::Music => {
                self.imp().tab_music.remove_css_class("active-tab");
                self.imp().tab_programs.add_css_class("active-tab");
                self.set_property("current-tab", CrabTab::Programs.to_value());
            }
        }
    }

    fn get_current_tab(&self) -> CrabTab {
        if self.imp().tab_programs.has_css_class("active-tab") {
            CrabTab::Programs
        } else if self.imp().tab_music.has_css_class("active-tab") {
            CrabTab::Music
        } else {
            CrabTab::Programs
        }
    }
}
