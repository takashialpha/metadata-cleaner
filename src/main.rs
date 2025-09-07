use num_traits::ToPrimitive;
use std::fmt;
use std::io::{self, Write};

enum AppError {
    FailedReadingMetadata(rexiv2::Rexiv2Error),
    FailedWritingMetadata(rexiv2::Rexiv2Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FailedReadingMetadata(err) => write!(f, "Failed reading metadata: {err}"),
            AppError::FailedWritingMetadata(err) => write!(f, "Failed writing metadata: {err}"),
        }
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)        
    }
}
impl std::error::Error for AppError {}

impl AppError {
    pub fn error_exit(error: AppError) -> ! {
        eprintln!("{}", error);
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
    // thumbnail: Option<Vec<u8>>,
    // preview_images: *type,
    // **disabled code**
}

impl MetadataInfo {
    fn format_tags(raw_metadata: &rexiv2::Metadata, tags: &[String]) -> Vec<String> {
        tags.iter()
            .map(|tag| {
                let value = raw_metadata
                    .get_tag_interpreted_string(tag)
                    .unwrap_or_else(|_| "N/A".to_string());
                format!("{tag} = {value}")
            })
            .collect()
    }

    fn media_type(raw_metadata: &rexiv2::Metadata) -> Option<rexiv2::MediaType> {
        raw_metadata.get_media_type().ok()
    }

    fn pixel_width(raw_metadata: &rexiv2::Metadata) -> Option<i32> {
        Some(raw_metadata.get_pixel_width())
    }

    fn pixel_height(raw_metadata: &rexiv2::Metadata) -> Option<i32> {
        Some(raw_metadata.get_pixel_height())
    }

    fn exposure_time(raw_metadata: &rexiv2::Metadata) -> Option<f64> {
        raw_metadata
            .get_exposure_time()
            .and_then(|ratio| ratio.to_f64())
    }

    fn fnumber(raw_metadata: &rexiv2::Metadata) -> Option<f64> {
        raw_metadata.get_fnumber()
    }

    fn focal_length(raw_metadata: &rexiv2::Metadata) -> Option<f64> {
        raw_metadata.get_focal_length()
    }

    fn iso_speed(raw_metadata: &rexiv2::Metadata) -> Option<i32> {
        raw_metadata.get_iso_speed()
    }

    fn gps_info(raw_metadata: &rexiv2::Metadata) -> Option<rexiv2::GpsInfo> {
        raw_metadata.get_gps_info()
    }

    fn orientation(raw_metadata: &rexiv2::Metadata) -> Option<rexiv2::Orientation> {
        Some(raw_metadata.get_orientation())
    }

    fn exif_tags(raw_metadata: &rexiv2::Metadata) -> Vec<String> {
        raw_metadata.get_exif_tags().unwrap_or_default()
    }

    fn iptc_tags(raw_metadata: &rexiv2::Metadata) -> Vec<String> {
        raw_metadata.get_iptc_tags().unwrap_or_default()
    }

    fn xmp_tags(raw_metadata: &rexiv2::Metadata) -> Vec<String> {
        raw_metadata.get_xmp_tags().unwrap_or_default()
    }

    pub fn from(raw_metadata: &rexiv2::Metadata) -> MetadataInfo {
        MetadataInfo {
            media_type: Self::media_type(raw_metadata),
            pixel_width: Self::pixel_width(raw_metadata),
            pixel_height: Self::pixel_height(raw_metadata),
            exposure_time: Self::exposure_time(raw_metadata),
            fnumber: Self::fnumber(raw_metadata),
            focal_length: Self::focal_length(raw_metadata),
            iso_speed: Self::iso_speed(raw_metadata),
            gps_info: Self::gps_info(raw_metadata),
            orientation: Self::orientation(raw_metadata),
            exif_tags: Self::format_tags(raw_metadata, &Self::exif_tags(raw_metadata)),
            iptc_tags: Self::format_tags(raw_metadata, &Self::iptc_tags(raw_metadata)),
            xmp_tags: Self::format_tags(raw_metadata, &Self::xmp_tags(raw_metadata)),
            // thumbnail: Self::thumbnail(&raw_metadata),
            // preview_images: Self::preview_images(&raw_metadata),
            // **disabled code**

            // thumbnails are panicking in rexiv2 crate.
            // method for preview images not built yet. wait until gtk ui is built.
        }
    }
}

impl fmt::Display for MetadataInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_opt<T: ToString>(val: Option<T>) -> String {
            val.map_or("N/A".to_string(), |v| v.to_string())
        }

        writeln!(
            f,
            "Media Type: {}",
            self.media_type
                .as_ref()
                .map_or("N/A".to_string(), |v| format!("{:?}", v))
        )?;
        writeln!(
            f,
            "Dimensions: {} x {}",
            self.pixel_width.unwrap_or(0),
            self.pixel_height.unwrap_or(0)
        )?;
        writeln!(f, "Exposure Time: {}", fmt_opt(self.exposure_time))?;
        writeln!(f, "F-number: {}", fmt_opt(self.fnumber))?;
        writeln!(f, "Focal Length: {}", fmt_opt(self.focal_length))?;
        writeln!(f, "ISO Speed: {}", fmt_opt(self.iso_speed))?;
        writeln!(f, "Orientation: {:?}", self.orientation)?;
        writeln!(f, "GPS Info: {:?}", self.gps_info)?;
        writeln!(f, "EXIF Tags: [{}]", self.exif_tags.join(", "))?;
        writeln!(f, "IPTC Tags: [{}]", self.iptc_tags.join(", "))?;
        writeln!(f, "XMP Tags: [{}]", self.xmp_tags.join(", "))?;
        Ok(())
    }
}

struct MetadataReader {
    raw_metadata: rexiv2::Metadata,
}

impl MetadataReader {
    pub fn from_path(file_path: &String) -> Result<MetadataReader, AppError> {
        Ok(MetadataReader {
            raw_metadata: Self::load_metadata(file_path)?,
        })
    }

    fn load_metadata(file_path: &String) -> Result<rexiv2::Metadata, AppError> {
        match rexiv2::Metadata::new_from_path(file_path) {
            Ok(raw_metadata) => Ok(raw_metadata),
            Err(e) => Err(AppError::FailedReadingMetadata(e)),
        }
    }
}

struct MetadataWriter;

impl MetadataWriter {
    pub fn clear_file(raw_metadata: &rexiv2::Metadata, file_path: &String) {
        raw_metadata.clear();
        if let Err(e) = raw_metadata.save_to_file(file_path) {
            AppError::error_exit(AppError::FailedWritingMetadata(e));
        }
    }
}

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
