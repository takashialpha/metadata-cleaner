use std::io::{self, Write};

enum AppError {
    FailedReadingMetadata(rexiv2::Rexiv2Error),
}

impl AppError {
    fn error_msg(error: AppError) -> String {
        match error {
            AppError::FailedReadingMetadata(err) => format!("{}", err),
        }
    }

    pub fn error_exit(error: AppError) -> ! {
        println!("{}", Self::error_msg(error));
        std::process::exit(1);
    }
}

struct Cli;

impl Cli {
    pub fn prompt_input(prompt: &str) -> String {
        loop {
            print!("{prompt}");
            match io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to flush stdout: {}", e);
                    continue;
                }
            }

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to read input: {}", e);
                    continue;
                }
            }

            let input = input.trim();
            match input.is_empty() {
                false => return input.to_string(),
                true => eprintln!("Input cannot be empty."),
            }
        }
    }
}

struct MetadataInfo {
    media_type: Option<rexiv2::MediaType>,
    pixel_width: Option<i32>,
    pixel_height: Option<i32>,
    exposure_time: Option<f64>,
    fnumber: Option<f64>,
    focal_length: Option<f64>,
    iso_speed: Option<i32>,
    gps_info: Option<rexiv2::GpsInfo>,
    orientation: Option<rexiv2::Orientation>,
    exif_tags: Vec<String>,
    iptc_tags: Vec<String>,
    xmp_tags: Vec<String>,
    thumbnail: Option<Vec<u8>>,
    preview_images: Vec<Vec<u8>>,
}

impl MetadataInfo {
    fn media_type(raw_metadata: &rexiv2::Metadata) -> Option<rexiv2::MediaType> {
        rexiv2::Metadata::get_media_type(raw_metadata).ok()
    }

    pub fn from(raw_metadata: rexiv2::Metadata) -> MetadataInfo {
        MetadataInfo {
            media_type: Self::media_type(&raw_metadata),
        }
    }
}

struct MetadataReader {
    raw_metadata: rexiv2::Metadata,
}

impl MetadataReader {
    pub fn from_path(file_path: String) -> Result<MetadataReader, AppError> {
        Ok(MetadataReader {
            raw_metadata: Self::load_metadata(file_path)?,
        })
    }

    fn load_metadata(file_path: String) -> Result<rexiv2::Metadata, AppError> {
        match rexiv2::Metadata::new_from_path(&file_path) {
            Ok(raw_metadata) => Ok(raw_metadata),
            Err(e) => Err(AppError::FailedReadingMetadata(e)),
        }
    }
}

fn main() {
    let file_path = Cli::prompt_input("Targetted file path: ");

    let raw_meta = MetadataReader::from_path(file_path).unwrap_or_else(|e| {
        AppError::error_exit(e);
    });

    let fancy_meta = MetadataInfo::from(raw_meta.raw_metadata);
}
