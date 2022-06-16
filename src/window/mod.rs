use std::process::Command;

use gtk::{CustomFilter, gio, Inhibit, SignalListItemFactory, SingleSelection};
use gtk::{Application, FilterChange, glib};
use gtk::gio::AppInfo;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::crab_row::CrabRow;
use crate::crab_tabs::imp::CrabTab;
use crate::music_object::MusicObject;

use super::consts::*;
use super::utils::*;

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application, is_daemon: bool) -> Self {
        Object::new(&[("application", app), ("is-daemon", &is_daemon.to_value())])
            .expect("Failed to create `Window`.")
    }

    pub fn current_filter(&self) -> CustomFilter {
        self.imp().current_filter.borrow().clone()
    }

    pub fn current_selection_model(&self) -> SingleSelection {
        self.imp().current_selection_model.borrow().clone()
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

        let (filter, selection_model) = setup_programs_model(self);

        self.imp().crab_items_list.set_model(Some(&selection_model));
        self.imp().current_filter.replace(filter);
        self.imp().current_selection_model.replace(selection_model);

        self.imp().tabs.connect_notify_local(Some("current-tab"), clone!(@weak self as window => move |crab_tabs, _| {
            let current_tab = crab_tabs.property::<i32>("current-tab");

            if current_tab == CrabTab::Music.to_value() {
                let (title, artist) = get_current_song();

                window.imp().song_info.set_visible(true);
                window.imp().song_info.set_song_data(title, artist);
            }
            else {
                window.imp().song_info.set_visible(false);
            }

            let (filter, selection_model) = setup_list_model(&window, &match current_tab {
                0 => CrabTab::Programs,
                1 => CrabTab::Music,
                _ => CrabTab::Programs
            });

            window.imp().crab_items_list.set_model(Some(&selection_model));
            window.imp().current_filter.replace(filter);
            window.imp().current_selection_model.replace(selection_model);
        }));

        self.imp()
            .entry
            .connect_changed(clone!(@strong self as window => move |_| {
                window.current_filter().changed(FilterChange::Different);
            }));

        self.imp().entry.connect_activate(
            clone!(@weak self as window => move |_| {
                let row_data = &window.current_selection_model().selected_item();
                if row_data.is_none() { return; }
                let row_data = row_data.as_ref().unwrap().clone().downcast::<AppInfo>();

                if let Ok(row_data) = row_data {
                    open_app(&row_data, &window);
                }
                else {
                    let row_data = &window.current_selection_model().selected_item().unwrap().downcast::<MusicObject>().unwrap();

                    if row_data.get_uri().is_none() {
                        return;
                    }

                    Command::new("xdg-open").arg(row_data.get_uri().unwrap()).spawn().unwrap();

                    window.hide_or_close();
                }
            }),
        );
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let crab_row = CrabRow::new();
            list_item.set_child(Some(&crab_row));
        });

        factory.connect_bind(clone!(@weak self as window => move |_, list_item| {
            let crab_row = list_item.child().unwrap().downcast::<CrabRow>().unwrap();

            let row_data = list_item.item().unwrap().downcast::<AppInfo>();
            if row_data.is_err() {
                let row_data = list_item.item().unwrap().downcast::<MusicObject>().unwrap();
                crab_row.set_row_data(&row_data);
            }
            else {
                crab_row.set_row_data(&row_data.unwrap());
            }
        }));

        self.imp().crab_items_list.set_factory(Some(&factory));
    }

    fn setup_keybinds(&self) {
        let controller = gtk::EventControllerKey::new();

        controller.connect_key_pressed(clone!(@strong self as window => move |_, key, keycode, _| {
            let selection_model = window.current_selection_model();

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
                    window.hide_or_close();

                    Inhibit(true)
                }
                KEY_ENTER => {
                    let row_data = &window.current_selection_model().selected_item();
                    if row_data.is_none() { return Inhibit(false); }
                    let row_data = row_data.as_ref().unwrap().clone().downcast::<AppInfo>();

                    if let Ok(row_data) = row_data {
                        open_app(&row_data, &window);
                    }
                    else {
                        let row_data = &selection_model.selected_item().unwrap().downcast::<MusicObject>().unwrap();

                        if row_data.get_uri().is_none() {
                            return Inhibit(false);
                        }

                        row_data.start_playing();

                        window.hide_or_close();
                    }

                    Inhibit(false)
                }
                KEY_TAB => {
                    window.imp().tabs.change_tab(None);

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

    pub fn clean_up(&self) {
        self.current_selection_model().select_item(0, true);
        self.imp()
            .crab_items_list
            .activate_action("list.scroll-to-item", Some(&0.to_variant()))
            .unwrap();

        self.imp().entry.buffer().set_text("");
        self.imp()
            .entry
            .set_placeholder_text(Some(PLACEHOLDER_PROGRAMS));

        self.imp().tabs.change_tab(Some(CrabTab::Programs));
    }

    pub fn hide_or_close(&self) {
        if self.property("is-daemon") {
            self.hide();
        } else {
            self.close();
        }
    }
}
