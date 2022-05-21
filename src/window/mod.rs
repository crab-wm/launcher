use super::consts::*;
use crate::crab_row::CrabRow;
use gtk::glib::{clone, MainContext, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Adjustment, CustomFilter, FilterListModel, gio, Inhibit, ScrollType, SignalListItemFactory, SingleSelection};
use gtk::{glib, Application, FilterChange};
use gtk::gio::{AppInfo};

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

    fn setup_window(&self) {
        let model = gio::ListStore::new(AppInfo::static_type());
        AppInfo::all().iter().for_each(|app_info| {
            model.append(app_info);
        });

        self.imp().current_items.replace(Some(model));

        let filter = CustomFilter::new(clone!(@strong self as window => move |obj| {
            let crab_entry = obj.downcast_ref::<gio::AppInfo>().unwrap();
            let search = window.imp().entry.buffer().text();

            if !search.is_empty() { crab_entry.name().to_lowercase().contains(&search.as_str().to_lowercase()) } else { true }
        }));

        let filter_model = FilterListModel::new(Some(&self.current_items()), Some(&filter));

        let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
            let app_info1 = obj1.downcast_ref::<AppInfo>().unwrap();
            let app_info2 = obj2.downcast_ref::<AppInfo>().unwrap();

            app_info1
                .name()
                .to_lowercase()
                .cmp(&app_info2.name().to_lowercase())
                .into()
        });
        let sorted_model = gtk::SortListModel::new(Some(&filter_model), Some(&sorter));

        let selection_model = SingleSelection::new(Some(&sorted_model));
        self.imp().crab_items_list.set_model(Some(&selection_model));

        self.imp().entry.connect_changed(clone!(@strong self as window => move |_| {
            filter.changed(FilterChange::Different);
        }));

        self.imp().entry.connect_activate(clone!(@weak self as window, @weak selection_model => move |_| {
            open_app(
                &selection_model.selected_item().unwrap().downcast::<AppInfo>().unwrap(),
                &window,
            );
        }));

        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(clone!(@strong self as window => move |_, key, keycode, _| {
            match keycode {
                KEY_UP_ARROW => {
                    let new_selection = if selection_model.selected() > 0 { selection_model.selected() - 1 } else { 0 };
                    selection_model.select_item(new_selection, true);
                    window.imp().crab_items_list.activate_action("list.scroll-to-item", Some(&new_selection.to_variant())).unwrap();

                    Inhibit(false)
                }
                KEY_DOWN_ARROW => {
                    let new_selection = std::cmp::min(selection_model.n_items() - 1, selection_model.selected() + 1);
                    selection_model.select_item(new_selection, true);
                    window.imp().crab_items_list.activate_action("list.scroll-to-item", Some(&new_selection.to_variant())).unwrap();

                    Inhibit(false)
                }
                KEY_ESC => {
                    window.close();

                    Inhibit(false)
                }
                KEY_ENTER => {
                    open_app(
                        &selection_model.selected_item().unwrap().downcast::<AppInfo>().unwrap(),
                        &window,
                    );

                    Inhibit(false)
                }
                _ => {
                    if keycode != KEY_LEFT_SHIFT && keycode != KEY_RIGHT_SHIFT {
                        window.imp().entry.grab_focus();
                    }

                    if !(key.is_lower() && key.is_upper()) {
                        if let Some(key_name) = key.name() {
                            let buffer = window.imp().entry.buffer();

                            let mut content = buffer.text();
                            content.push(key_name.chars().next().unwrap());

                            buffer.set_text(content.as_str());
                            window.imp().entry.set_position((content.len()) as i32);
                            window.imp().entry.set_placeholder_text(None);
                        }
                    }

                    Inhibit(false)
                }
            }
        }));

        self.add_controller(&controller);
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let crab_row = CrabRow::new();
            list_item.set_child(Some(&crab_row));
        });

        factory.connect_bind(move |_, list_item| {
            let app_info = list_item
                .item()
                .unwrap()
                .downcast::<AppInfo>()
                .unwrap();

            let crab_row = list_item.child().unwrap().downcast::<CrabRow>().unwrap();

            crab_row.set_app_info(&app_info);
        });

        self.imp().crab_items_list.set_factory(Some(&factory));
    }
}

fn open_app(app_info: &AppInfo, window: &Window) {
    let parent_window = window.imp().crab_items_list.root().unwrap().downcast::<gtk::Window>().unwrap();
    let context = gtk::Window::new().display().app_launch_context();

    window.hide();

    let commandline = app_info.commandline().unwrap();
    let main_context = MainContext::default();

    main_context.spawn_local(clone!(@strong commandline, @strong window, @strong app_info, @strong context => async move {
        if let Err(_) = async_process::Command::new(commandline.as_os_str()).output().await {
            if let Err(err) = app_info.launch(&[], Some(&context)) {
                gtk::MessageDialog::builder()
                    .text(&format!("Failed to start {}!", app_info.name()))
                    .secondary_text(&err.to_string())
                    .message_type(gtk::MessageType::Error)
                    .modal(true)
                    .transient_for(&window)
                    .build()
                    .show();
            }
        }

        parent_window.close();
    }));
}