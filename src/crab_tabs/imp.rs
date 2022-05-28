use crate::gio::glib::{ParamSpec, Value};
use gtk::glib::{ParamFlags, ParamSpecInt};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::cell::RefCell;

pub enum CrabTab {
    Programs,
    Music,
}

impl CrabTab {
    pub fn to_value(&self) -> i32 {
        match self {
            Self::Programs => 0,
            Self::Music => 1,
        }
    }
}

impl Default for CrabTab {
    fn default() -> Self {
        Self::Programs
    }
}

#[derive(Default, CompositeTemplate)]
#[template(resource = "/wm/crab/launcher/crab_tabs.ui")]
pub struct CrabTabs {
    #[template_child]
    pub tab_programs: TemplateChild<Label>,
    #[template_child]
    pub tab_music: TemplateChild<Label>,
    pub current_tab: RefCell<CrabTab>,
}

#[glib::object_subclass]
impl ObjectSubclass for CrabTabs {
    const NAME: &'static str = "CrabTabs";
    type Type = super::CrabTabs;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CrabTabs {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecInt::new(
                "current-tab",
                "current-tab",
                "current-tab",
                0,
                1,
                0,
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "current-tab" => {
                let new_tab: i32 = value.get().unwrap();
                self.current_tab.replace(match new_tab {
                    0 => CrabTab::Programs,
                    1 => CrabTab::Music,
                    _ => CrabTab::Programs,
                });
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "current-tab" => match self.current_tab.take().borrow() {
                CrabTab::Programs => 0.to_value(),
                CrabTab::Music => 1.to_value(),
            },
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.setup_tabs();
    }
}

impl WidgetImpl for CrabTabs {}

impl BoxImpl for CrabTabs {}
