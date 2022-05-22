use std::borrow::Borrow;
use super::consts::*;
use super::utils::*;
use crate::crab_row::CrabRow;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CustomFilter, gio, Inhibit, SignalListItemFactory, SingleSelection};
use gtk::{glib, Application, FilterChange};
use gtk::gio::{AppInfo};
use crate::crab_tabs::imp::CrabTab;

pub(crate) mod imp;

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

    pub fn current_filter(&self) -> CustomFilter {
        self.imp()
            .current_filter
            .borrow()
            .clone()
    }

    pub fn current_items(&self) -> gio::ListStore {
        self.imp()
            .current_items
            .borrow()
            .clone()
            .expect(ERROR_ITEMS)
    }

    fn setup_window(&self) {
        self.imp().crab_items_list.set_can_focus(false);
        self.imp().tabs.set_can_focus(false);

        let (filter, selection_model) = setup_list_model(&self, self.imp().tabs.imp().current_tab.take().borrow());
        self.imp().crab_items_list.set_model(Some(&selection_model));
        self.imp().current_filter.replace(filter);

        self.imp().tabs.connect_notify_local(Some("current-tab"), clone!(@weak self as window => move |crab_tabs, _| {
            let (filter, selection_model) = setup_list_model(&window, &match crab_tabs.property::<i32>("current-tab") {
                0 => CrabTab::Programs,
                1 => CrabTab::Music,
                _ => CrabTab::Programs
            });

            window.imp().crab_items_list.set_model(Some(&selection_model));
            window.imp().current_filter.replace(filter);
        }));

        self.imp().entry.connect_changed(clone!(@strong self as window => move |_| {
            window.current_filter().changed(FilterChange::Different);
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
                    let new_selection = if selection_model.n_items() > 0 { std::cmp::min(selection_model.n_items() - 1, selection_model.selected() + 1) } else { 0 };
                    selection_model.select_item(new_selection, true);
                    window.imp().crab_items_list.activate_action("list.scroll-to-item", Some(&new_selection.to_variant())).unwrap();

                    Inhibit(false)
                }
                KEY_ESC => {
                    window.close();

                    Inhibit(true)
                }
                KEY_ENTER => {
                    open_app(
                        &selection_model.selected_item().unwrap().downcast::<AppInfo>().unwrap(),
                        &window,
                    );

                    Inhibit(false)
                }
                KEY_TAB => {
                    window.imp().tabs.change_tab();

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