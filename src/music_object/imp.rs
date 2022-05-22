use crate::music_object::MusicData;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MusicObject {
    pub data: Rc<RefCell<MusicData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MusicObject {
    const NAME: &'static str = "MusicObject";
    type Type = super::MusicObject;
}

impl ObjectImpl for MusicObject {}
