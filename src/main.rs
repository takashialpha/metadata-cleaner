mod error;
mod gui;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use error::AppError;
use gui::GuiApp;
use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;
use metadata_writer::MetadataWriter;

fn main() {
    let gui = GuiApp::new("org.takashialpha.metadatacleaner");
    let exit_code = gui.run();
    // std::process::exit(exit_code.into());

    let file_path = "/home/takashi/Imagens/a3.jpg";

    let raw_meta = MetadataReader::from_path(&file_path).unwrap_or_else(|e| {
        AppError::error_exit(e);
    });

    let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
    println!("{}", fancy_meta);
}
