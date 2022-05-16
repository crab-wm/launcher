use super::consts::*;
use crate::crab_row::CrabRow;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, Inhibit, ListView, SelectionModel, SignalListItemFactory, SingleSelection};
use gtk::{glib, Application};

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    fn current_items(&self) -> gio::ListStore {
        self.imp()
            .current_items
            .borrow()
            .clone()
            .expect(ERROR_ITEMS)
    }

    fn setup_items(&self) {
        let model = gio::ListStore::new(gio::AppInfo::static_type());
        gio::AppInfo::all().iter().for_each(|app_info| {
            model.append(app_info);
        });

        self.imp().current_items.replace(Some(model));

        let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
            let app_info1 = obj1.downcast_ref::<gio::AppInfo>().unwrap();
            let app_info2 = obj2.downcast_ref::<gio::AppInfo>().unwrap();

            app_info1
                .name()
                .to_lowercase()
                .cmp(&app_info2.name().to_lowercase())
                .into()
        });
        let sorted_model = gtk::SortListModel::new(Some(&self.current_items()), Some(&sorter));

        let selection_model = SingleSelection::new(Some(&sorted_model));
        self.imp().crab_items_list.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(clone!(@strong self as window => move |_, _, keycode, _| {
            match keycode {
                111 => {
                    window.imp().crab_items_list.grab_focus();
                    select_item(&window.imp().crab_items_list.model().unwrap(), SelectionType::Previous);
                    Inhibit(true)
                }
                116 => {
                    window.imp().crab_items_list.grab_focus();
                    select_item(&window.imp().crab_items_list.model().unwrap(), SelectionType::Next);
                    Inhibit(true)
                }
                23 => {
                    activate_item(&window.imp().crab_items_list);
                    Inhibit(true)
                }
                9 => {
                    window.close();
                    Inhibit(true)
                }
                _ => {
                    println!("{}", keycode);
                    window.imp().entry.grab_focus();

                    Inhibit(false)
                }
            }
        }));

        self.add_controller(&controller);
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let task_row = CrabRow::new();
            list_item.set_child(Some(&task_row));
        });

        factory.connect_bind(move |_, list_item| {
            let app_info = list_item
                .item()
                .unwrap()
                .downcast::<gio::AppInfo>()
                .unwrap();

            let task_row = list_item.child().unwrap().downcast::<CrabRow>().unwrap();

            task_row.set_app_info(&app_info);
        });

        self.imp().crab_items_list.set_factory(Some(&factory));
    }
}

enum SelectionType {
    Next,
    Previous,
}

fn activate_item(tasks: &TemplateChild<ListView>) {
    let selection_model = tasks.model().unwrap();

    let mut selected_index = 0;

    for i in 0..selection_model.n_items() {
        let is_selected = selection_model.selection().contains(i);

        if is_selected {
            break;
        }

        selected_index += 1;
    }

    let item = selection_model.item(selected_index as u32).unwrap();

    item.set_property(
        "completed",
        !item.property_value("completed").get::<bool>().unwrap(),
    );
}

fn select_item(selection_model: &SelectionModel, selection_type: SelectionType) {
    let selection_value: i32 = match selection_type {
        SelectionType::Next => 1,
        SelectionType::Previous => -1,
    };

    let mut selected_index = 0;

    for i in 0..selection_model.n_items() {
        let is_selected = selection_model.selection().contains(i);

        if is_selected {
            break;
        }

        selected_index += 1;
    }

    if selection_model.n_items() > 0 {
        selection_model.select_item(
            if selected_index + selection_value >= selection_model.n_items() as i32
                || selected_index + selection_value < 0
            {
                match selection_type {
                    SelectionType::Next => 0,
                    SelectionType::Previous => selection_model.n_items() - 1,
                }
            } else {
                (selected_index + selection_value).try_into().unwrap()
            },
            true,
        );
    }
}
