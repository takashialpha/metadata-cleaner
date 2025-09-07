mod cli;
mod error;
mod metadata_info;
mod metadata_reader;
mod metadata_writer;

use cli::Cli;
use error::AppError;
use metadata_info::MetadataInfo;
use metadata_reader::MetadataReader;
use metadata_writer::MetadataWriter;

fn main() {
    let file_path = Cli::prompt_input("Targetted file path: ");

    let raw_meta = MetadataReader::from_path(&file_path).unwrap_or_else(|e| {
        AppError::error_exit(e);
    });

    let fancy_meta = MetadataInfo::from(&raw_meta.raw_metadata);
    println!("{}", fancy_meta);

    let meta_erase_choice = Cli::prompt_input("Do you want to erase all metadata above? (y/n) ");
    match meta_erase_choice.as_str() {
        "y" | "Y" => MetadataWriter::clear_file(&raw_meta.raw_metadata, &file_path),
        _ => std::process::exit(0),
    };
}
