use gtk::{CompositeTemplate, glib, Label};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/wm/crab/launcher/song_info.ui")]
pub struct SongInfo {
    #[template_child]
    pub song_title: TemplateChild<Label>,
    #[template_child]
    pub song_artist: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for SongInfo {
    const NAME: &'static str = "SongInfo";
    type Type = super::SongInfo;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SongInfo {}

impl WidgetImpl for SongInfo {}

impl BoxImpl for SongInfo {}
