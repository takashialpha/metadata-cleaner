mod error;
mod file_chooser;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use crate::error::AppError;
use crate::file_chooser::FileDialogManager;
use crate::metadata_writer::MetadataWriter;
use adw::glib;
use adw::gtk::pango::WrapMode;
use adw::gtk::{
    ApplicationWindow, Box as GtkBox, Button, Frame, HeaderBar, Image, Label, Orientation,
    PolicyType, ScrolledWindow, Stack,
};
use adw::prelude::*;
use libadwaita as adw;
use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;
use std::cell::RefCell;
use std::rc::Rc;

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
            clear_button.add_css_class("suggested-action");

            let stack = Stack::new();

            let placeholder_frame = Frame::new(None);
            placeholder_frame.set_margin_start(24);
            placeholder_frame.set_margin_end(24);
            placeholder_frame.set_margin_top(24);
            placeholder_frame.set_margin_bottom(24);

            let placeholder_box = GtkBox::new(Orientation::Vertical, 12);
            let placeholder_icon = Image::from_icon_name("text-x-generic-symbolic");
            placeholder_icon.set_pixel_size(64);
            let placeholder_label = Label::new(Some(
                "Welcome to Metadata Cleaner!\nClick the folder icon to open a file.",
            ));
            placeholder_label.set_wrap(true);
            placeholder_label.set_wrap_mode(WrapMode::WordChar);
            placeholder_label.set_xalign(0.5);
            placeholder_box.append(&placeholder_icon);
            placeholder_box.append(&placeholder_label);
            placeholder_frame.set_child(Some(&placeholder_box));

            stack.add_named(&placeholder_frame, Some("placeholder"));

            let scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .vscrollbar_policy(PolicyType::Automatic)
                .child(&stack)
                .build();

            let header = HeaderBar::new();
            header.set_title_widget(Some(&Label::new(Some("Metadata Cleaner"))));
            header.pack_start(&open_button);
            header.pack_end(&clear_button);
            window.set_titlebar(Some(&header));

            let current_file_path: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

            open_button.connect_clicked({
                let stack = stack.clone();
                let clear_button = clear_button.clone();
                let placeholder_frame = placeholder_frame.clone();
                let current_file_path = current_file_path.clone();

                move |_| {
                    let stack_inner = stack.clone();
                    let clear_button_inner = clear_button.clone();
                    let placeholder_clone = placeholder_frame.clone();
                    let current_file_path_inner = current_file_path.clone();

                    FileDialogManager::open_file(move |file_option| {
                        while let Some(child) = stack_inner.first_child() {
                            stack_inner.remove(&child);
                        }

                        if let Some(file) = file_option {
                            let raw_meta = MetadataReader::from_path(&file).unwrap_or_else(|e| {
                                AppError::error_exit(e);
                            });

                            *current_file_path_inner.borrow_mut() = Some(file.clone());

                            let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
                            let widget = fancy_meta.to_widget();
                            stack_inner.add_named(&widget, Some("metadata"));
                            stack_inner.set_visible_child(&widget);
                            clear_button_inner.set_visible(true);
                        } else {
                            stack_inner.add_named(&placeholder_clone, Some("placeholder"));
                            stack_inner.set_visible_child(&placeholder_clone);
                            clear_button_inner.set_visible(false);
                        }
                    });
                }
            });

            clear_button.connect_clicked({
                let stack = stack.clone();
                let clear_button = clear_button.clone();
                let current_file_path = current_file_path.clone();

                move |_| {
                    if let Some(file_path) = &*current_file_path.borrow() {
                        let raw_meta = MetadataReader::from_path(file_path).unwrap_or_else(|e| {
                            AppError::error_exit(e);
                        });
                        MetadataWriter::clear_file(&raw_meta.raw_metadata, file_path);
                    }

                    while let Some(child) = stack.first_child() {
                        stack.remove(&child);
                    }

                    let cleared_frame = Frame::new(None);
                    cleared_frame.set_margin_start(24);
                    cleared_frame.set_margin_end(24);
                    cleared_frame.set_margin_top(24);
                    cleared_frame.set_margin_bottom(24);

                    let cleared_box = GtkBox::new(Orientation::Vertical, 12);
                    let cleared_icon = Image::from_icon_name("dialog-information-symbolic");
                    cleared_icon.set_pixel_size(64);
                    let cleared_label = Label::new(Some(
                        "All metadata has been successfully cleared.\nYou can now open another file to inspect again.",
                    ));
                    cleared_label.set_wrap(true);
                    cleared_label.set_wrap_mode(WrapMode::WordChar);
                    cleared_label.set_xalign(0.5);
                    cleared_box.append(&cleared_icon);
                    cleared_box.append(&cleared_label);
                    cleared_frame.set_child(Some(&cleared_box));

                    stack.add_named(&cleared_frame, Some("cleared"));
                    stack.set_visible_child(&cleared_frame);
                    clear_button.set_visible(false);

                    current_file_path.borrow_mut().take();
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
