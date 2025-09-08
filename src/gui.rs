use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

pub struct GuiApp {
    app: Application,
}

impl GuiApp {
    pub fn new(application_id: &str) -> Self {
        let app = Application::builder()
            .application_id(application_id)
            .build();

        Self { app }
    }

    fn setup(&self) {
        self.app.connect_activate(|app| {
            let window = ApplicationWindow::builder()
                .application(app)
                // .default_width(800)
                // .default_height(600)
                // design conventions to be defined.
                .title("Metadata Cleaner")
                .build();

            window.present();
        });
    }

    pub fn run(&self) -> glib::ExitCode {
        self.setup();
        self.app.run()
    }
}

