use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Image, Label};
use gtk::gio::{AppInfo, Icon};

pub trait CrabRowExt {
    fn get_name(&self) -> String;
    fn get_icon(&self) -> Option<Icon>;
}

impl CrabRowExt for AppInfo {
    fn get_name(&self) -> String {
        self.name().to_string()
    }

    fn get_icon(&self) -> Option<Icon> {
        self.icon()
    }
}

#[derive(Default, CompositeTemplate)]
#[template(resource = "/wm/crab/launcher/crab_row.ui")]
pub struct CrabRow {
    #[template_child]
    pub name: TemplateChild<Label>,
    #[template_child]
    pub image: TemplateChild<Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for CrabRow {
    const NAME: &'static str = "CrabRow";
    type Type = super::CrabRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CrabRow {}

impl WidgetImpl for CrabRow {}

impl BoxImpl for CrabRow {}
