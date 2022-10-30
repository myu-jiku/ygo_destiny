mod imp;

use std::fs;
use std::sync::Mutex;

use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::logic::ext_data::vercheck;
use crate::logic::utils::http;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl Window {
    pub fn new<P: glib::IsA<adw::Application>>(app: &P) -> Self {
        let window: Window = glib::Object::new(&[("application", app)]);

        return window;
    }

    fn get_leaflet(&self) -> &gtk::TemplateChild<adw::Leaflet> {
        return &self.imp().leaflet;
    }

    fn get_toast_overlay(&self) -> &gtk::TemplateChild<adw::ToastOverlay> {
        return &self.imp().toast_overlay;
    }

    fn get_update_version(&self) -> &Mutex<String> {
        return &self.imp().update_version;
    }

    pub fn show_update_notification(&self) {
        // *self.get_update_version().lock().unwrap() = http::update_version();
        let update_version = http::update_version();
        if update_version.is_some() {
            *self.get_update_version().lock().unwrap() = update_version.unwrap();

            let toast_overlay = self.get_toast_overlay();

            let toast = adw::Toast::new("New update available.");
            toast.set_button_label(Some("Update"));
            toast.set_action_name(Some("win.update_data"));

            toast_overlay.add_toast(&toast);
        }
    }

    pub fn update(&self) {
        let update_status = http::update();

        match update_status {
            http::UpdateStatus::Complete => fs::write(
                &*vercheck::EXT_PATH,
                &*self.get_update_version().lock().unwrap(),
            )
            .unwrap(),
            x => println!("{:#?}", x),
        }
    }
}
