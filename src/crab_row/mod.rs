mod imp;

use gtk::glib::Object;
use gtk::prelude::*;
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
        Object::new(&[]).expect("Failed to create ApplicationRow")
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let imp = self.imp();
        imp.name.set_text(&app_info.name());
        if let Some(icon) = app_info.icon() {
            imp.image.set_from_gicon(&icon);
        }
    }
}
