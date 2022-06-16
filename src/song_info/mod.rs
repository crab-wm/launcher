use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

pub mod imp;

glib::wrapper! {
    pub struct SongInfo(ObjectSubclass<imp::SongInfo>)
    @extends gtk::Box, gtk::Widget;
}

impl Default for SongInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SongInfo {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create SongInfo")
    }

    pub fn set_song_data(&self, title: Option<String>, artist: Option<String>) {
        let imp = self.imp();

        if title.is_none() || artist.is_none() {
            self.set_visible(false);
            return;
        }

        imp.song_title.set_text(&title.unwrap());
        imp.song_artist.set_text(&artist.unwrap());
    }
}
