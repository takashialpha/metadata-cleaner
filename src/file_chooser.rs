use gtk4::gio::File;
use gtk4::glib::Error;
use gtk4::prelude::*;
use gtk4::{FileDialog, Window};

pub struct FileDialogManager;

impl FileDialogManager {
    pub fn open_file<F: 'static + Fn(Option<String>)>(callback: F) {
        let dialog = FileDialog::new();
        dialog.set_title("Select a File");
        dialog.set_modal(true);

        dialog.open(
            None::<&Window>,                 // parent window
            None::<&gtk4::gio::Cancellable>, // cancellable
            move |result: Result<File, Error>| match result {
                Ok(file) => callback(Some(file.uri().to_string())),
                Err(_) => callback(None),
            },
        );
    }
}
