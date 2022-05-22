use crate::crab_row::imp::CrabRowExt;
use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::borrow::Borrow;
use std::cell::RefCell;

mod imp;

glib::wrapper! {
    pub struct MusicObject(ObjectSubclass<imp::MusicObject>);
}

impl MusicObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `MusicObject`.")
    }
}

impl CrabRowExt for MusicObject {
    fn get_name(&self) -> String {
        let music_data: &RefCell<MusicData> = self.imp().data.borrow();

        music_data.borrow().title.clone()
    }
}

#[derive(Default)]
pub struct MusicData {
    pub id: String,
    pub title: String,
}
