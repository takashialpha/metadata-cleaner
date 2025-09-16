mod error;
mod file_chooser;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use crate::file_chooser::FileDialogManager;
use error::AppError;
use libadwaita as adw;
use adw::gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, Button, ScrolledWindow, Label, Orientation, PolicyType, HeaderBar};
use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;
use gtk4::glib;


pub struct GuiApp {
    app: adw::Application,
}

impl GuiApp {
    pub fn new(application_id: &str) -> Self {
        let app = adw::Application::builder()
            .application_id(application_id)
            .build();

        Self { app }
    }

    fn setup(&self) {
        self.app.connect_activate(|app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(700)
                .default_height(500)
                .build();

            let vbox = GtkBox::new(Orientation::Vertical, 12);
            vbox.set_margin_top(12);
            vbox.set_margin_bottom(12);
            vbox.set_margin_start(12);
            vbox.set_margin_end(12);

            let button = Button::from_icon_name("document-open-symbolic");
            button.set_tooltip_text(Some("Open an image file"));

            let meta_box = GtkBox::new(Orientation::Vertical, 12);
            let scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .vscrollbar_policy(PolicyType::Automatic)
                .child(&meta_box)
                .build();

            let header = HeaderBar::new();
            header.set_title_widget(Some(&Label::new(Some("Metadata Cleaner"))));
            header.pack_end(&button);
            window.set_titlebar(Some(&header));

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
                            let label = Label::new(Some("No file selected"));
                            label.set_xalign(0.0);
                            meta_box.append(&label);
                        }
                    });
                }
            });

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

