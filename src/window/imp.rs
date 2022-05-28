use crate::consts::*;
use crate::crab_tabs::CrabTabs;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CustomFilter, ScrolledWindow, SingleSelection};
use gtk::{CompositeTemplate, Entry, ListView};
use std::cell::{Cell, RefCell};
use gtk::glib::{ParamFlags, ParamSpecBoolean};
use once_cell::sync::Lazy;
use crate::gio::glib::{ParamSpec, Value};

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
    pub current_selection_model: RefCell<SingleSelection>,
    pub is_daemon: Cell<bool>
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
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecBoolean::new(
                "is-daemon",
                "is-daemon",
                "is-daemon",
                false,
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &Value,
        pspec: &ParamSpec,
    ) {
        match pspec.name() {
            "is-daemon" => {
                let input_value =
                    value.get().expect("The value needs to be of type `bool`.");
                self.is_daemon.replace(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "is-daemon" => self.is_daemon.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.setup_window();
        obj.setup_factory();
        obj.setup_keybinds();

        obj.set_decorated(false);
        obj.set_title(Some(APP_TITLE));
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
