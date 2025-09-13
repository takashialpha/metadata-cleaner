mod error;
mod file_chooser;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use crate::file_chooser::FileDialogManager;
use error::AppError;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, Button, ScrolledWindow, glib};
use gtk4 as gtk;
use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;

pub struct GuiApp {
    app: gtk::Application,
}

impl GuiApp {
    pub fn new(application_id: &str) -> Self {
        let app = gtk::Application::builder()
            .application_id(application_id)
            .build();

        Self { app }
    }

    fn setup(&self) {
        self.app.connect_activate(|app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Metadata Cleaner")
                .default_width(600)
                .default_height(400)
                .build();

            let vbox = GtkBox::new(gtk::Orientation::Vertical, 5);

            let button = Button::with_label("Open File");

            let meta_box = GtkBox::new(gtk::Orientation::Vertical, 5);
            let scroll = ScrolledWindow::builder()
                .vexpand(true)
                .child(&meta_box)
                .build();

            button.connect_clicked({
                let meta_box = meta_box.clone();
                move |_| {
                    let meta_box = meta_box.clone();
                    FileDialogManager::open_file(move |file_option| {
                        while let Some(child) = meta_box.first_child() {
                            meta_box.remove(&child);
                        }

                        if let Some(file) = file_option {
                            let raw_meta = MetadataReader::from_path(&file).unwrap_or_else(|e| {
                                AppError::error_exit(e);
                            });

                            let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
                            let widget = fancy_meta.to_widget();
                            meta_box.append(&widget);
                        } else {
                            let label = gtk::Label::new(Some("No file selected"));
                            meta_box.append(&label);
                        }
                    });
                }
            });

            vbox.append(&button);
            vbox.append(&scroll);
            window.set_child(Some(&vbox));
            window.present();
        });
    }

    pub fn run(&self) -> glib::ExitCode {
        self.setup();
        self.app.run()
    }
}

fn main() {
    let gui = GuiApp::new("org.takashialpha.metadatacleaner");
    gui.run();
}
