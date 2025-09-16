mod error;
mod file_chooser;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use crate::file_chooser::FileDialogManager;
use crate::error::AppError;
use libadwaita as adw;

use adw::gtk::{
    ApplicationWindow, Box as GtkBox, Button, HeaderBar, Label, Orientation, PolicyType,
    ScrolledWindow, Frame, Align, Image,
};
use adw::glib;
use adw::gtk::prelude::{
    ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt, WidgetExt, FrameExt,
};

use adw::gtk::pango::WrapMode;

use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;

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

            let vbox = GtkBox::new(Orientation::Vertical, 16);
            vbox.set_margin_start(16);
            vbox.set_margin_end(16);
            vbox.set_margin_top(16);
            vbox.set_margin_bottom(16);

            let open_button = Button::from_icon_name("document-open-symbolic");
            open_button.set_tooltip_text(Some("Open an image file"));

            let clear_button = Button::from_icon_name("edit-clear-symbolic");
            clear_button.set_tooltip_text(Some("Clear metadata"));
            clear_button.set_visible(false);

            let meta_box = GtkBox::new(Orientation::Vertical, 12);

            let placeholder_frame = Frame::new(None);
            placeholder_frame.set_margin_start(24);
            placeholder_frame.set_margin_end(24);
            placeholder_frame.set_margin_top(24);
            placeholder_frame.set_margin_bottom(24);

            let placeholder_box = GtkBox::new(Orientation::Vertical, 12);
            placeholder_box.set_valign(Align::Center);
            placeholder_box.set_halign(Align::Center);

            let placeholder_icon = Image::from_icon_name("document-properties-symbolic");
            placeholder_icon.set_pixel_size(64);

            let placeholder_label = Label::new(Some(
                "Click the folder icon to open a file."
            ));
            placeholder_label.set_wrap(true);
            placeholder_label.set_wrap_mode(WrapMode::WordChar);
            placeholder_label.set_xalign(0.5);
            placeholder_label.set_markup("<span size='xx-large'><b>Welcome to Metadata Cleaner!</b></span>");

            placeholder_box.append(&placeholder_icon);
            placeholder_box.append(&placeholder_label);
            placeholder_frame.set_child(Some(&placeholder_box));
            meta_box.append(&placeholder_frame);

            let scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .vscrollbar_policy(PolicyType::Automatic)
                .child(&meta_box)
                .build();

            let header = HeaderBar::new();
            header.set_title_widget(Some(&Label::new(Some("Metadata Cleaner"))));
            header.pack_start(&open_button);
            header.pack_end(&clear_button);
            window.set_titlebar(Some(&header));

            open_button.connect_clicked({
                let meta_box = meta_box.clone();
                let clear_button = clear_button.clone();
                let placeholder_frame = placeholder_frame.clone();
                move |_| {
                    let meta_box_inner = meta_box.clone();
                    let clear_button_inner = clear_button.clone();
                    FileDialogManager::open_file(move |file_option| {
                        if let Some(file) = file_option {
                            while let Some(child) = meta_box_inner.first_child() {
                                meta_box_inner.remove(&child);
                            }

                            let raw_meta = MetadataReader::from_path(&file).unwrap_or_else(|e| {
                                AppError::error_exit(e);
                            });

                            let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
                            let widget = fancy_meta.to_widget();
                            meta_box_inner.append(&widget);

                            clear_button_inner.set_visible(true);
                        }
                    });
                }
            });

            clear_button.connect_clicked({
                let meta_box = meta_box.clone();
                let clear_button = clear_button.clone();
                let placeholder_frame = placeholder_frame.clone();
                move |_| {
                    while let Some(child) = meta_box.first_child() {
                        meta_box.remove(&child);
                    }

                    meta_box.append(&placeholder_frame);
                    clear_button.set_visible(false);
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
    let gui = GuiApp::new("xyz.takashialpha.MetadataCleaner");
    gui.run();
}

