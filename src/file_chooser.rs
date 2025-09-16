use libadwaita as adw;

use adw::gtk::gio::File;
use adw::gtk::glib::Error;
use adw::gtk::prelude::*;
use adw::gtk::Window;

use gtk4::FileDialog;

pub struct FileDialogManager;

impl FileDialogManager {
    pub fn open_file<F: 'static + Fn(Option<String>)>(callback: F) {
        let dialog = FileDialog::new();
        dialog.set_title("Select a File");
        dialog.set_modal(true);

        dialog.open(
            None::<&Window>,
            None::<&adw::gio::Cancellable>,
            move |result: Result<File, Error>| match result {
                Ok(file) => callback(Some(file.uri().to_string())),
                Err(_) => callback(None),
            },
        );
    }
}
