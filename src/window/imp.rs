use crate::consts::*;
use crate::crab_tabs::CrabTabs;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CustomFilter, ScrolledWindow};
use gtk::{CompositeTemplate, Entry, ListView};
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/wm/crab/launcher/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub scrolled_window: TemplateChild<ScrolledWindow>,
    #[template_child]
    pub crab_items_list: TemplateChild<ListView>,
    #[template_child]
    pub tabs: TemplateChild<CrabTabs>,
    pub current_items: RefCell<Option<gio::ListStore>>,
    pub current_filter: RefCell<CustomFilter>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "CrabWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.setup_window();
        obj.setup_factory();

        obj.set_decorated(false);
        obj.set_title(Some(APP_TITLE));
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
