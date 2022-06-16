use gtk::{gio, glib};
use gtk::glib::Object;
use gtk::subclass::prelude::*;

use crate::crab_row::imp::CrabRowExt;
use crate::MAX_CHARS_IN_ROW;
use crate::utils::ellipse_string;

pub mod imp;

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

    pub fn set_row_data(&self, crab_row_info: &impl CrabRowExt) {
        let imp = self.imp();

        let name = &crab_row_info.get_name();
        imp.name.set_text(&ellipse_string(name, MAX_CHARS_IN_ROW));

        if let Some(icon) = crab_row_info.get_icon() {
            imp.image.set_from_gicon(&icon);
        } else {
            imp.image
                .set_from_gicon(&gio::Icon::for_string("media-playback-start").unwrap());
        }
    }
}
