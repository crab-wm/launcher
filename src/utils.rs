use std::process::exit;
use gtk::gio::AppInfo;
use gtk::glib::{clone, MainContext};
use gtk::prelude::*;
use crate::Window;

pub fn open_app(app_info: &AppInfo, window: &Window) {
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

        window.close();
    }));
}

pub fn display_err(message: &str) {
    println!("Error! {}", message);

    exit(1);
}