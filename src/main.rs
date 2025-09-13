mod error;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;
mod file_chooser;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use crate::file_chooser::FileDialogManager;
use error::AppError;
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
                .build();

            let button = Button::with_label("Open File");
            button.connect_clicked(move |_| {
                FileDialogManager::open_file(|file_option| {
                    if let Some(file) = file_option {
                        let raw_meta = MetadataReader::from_path(&file).unwrap_or_else(|e| {
                            AppError::error_exit(e);
                        });

                        let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
                        println!("{}", fancy_meta);
                    } else {
                        println!("No file selected");
                    }
                });
            });

            window.set_child(Some(&button));
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

