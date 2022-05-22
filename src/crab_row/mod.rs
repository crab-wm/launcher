pub mod imp;

use crate::crab_row::imp::CrabRowExt;
use gtk::glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct CrabRow(ObjectSubclass<imp::CrabRow>)
    @extends gtk::Box, gtk::Widget;
}

impl Default for CrabRow {
    fn default() -> Self {
        Self::new()
    }
}

impl CrabRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create CrabRow")
    }

    pub fn set_row_data(&self, app_info: &impl CrabRowExt) {
        let imp = self.imp();
        imp.name.set_text(&app_info.get_name());
        if let Some(icon) = app_info.get_icon() {
            imp.image.set_from_gicon(&icon);
        } else {
            imp.image
                .set_from_gicon(&gio::Icon::for_string("media-playback-start").unwrap());
        }
    }
}
