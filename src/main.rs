use std::io::{self, Write};

struct Cli {
    file_path: String,
}

impl Cli {
    pub fn prompt_input(prompt: &str) -> String {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }
}

struct MetadataReader {
    metadata: rexiv2::Metadata,
}

impl MetadataReader {
    pub fn new() -> Result<MetadataReader, rexiv2::Rexiv2Error> {
        let metadata = Self::load_metadata()?;
        Ok(MetadataReader { metadata })
    }

    fn load_metadata() -> Result<rexiv2::Metadata, rexiv2::Rexiv2Error> {
        let cli = Cli {
            file_path: Cli::prompt_input("Target file to get metadata: "),
        };

        rexiv2::Metadata::new_from_path(&cli.file_path)
    }

    pub fn info_summary(&self) -> String {
        format!(
            "File Type: {:?}\nSize: {}x{}\n",
            self.metadata.get_media_type(),
            self.metadata.get_pixel_width(),
            self.metadata.get_pixel_height()
        )
    }
}

fn main() {
    let meta = MetadataReader::new().unwrap_or_else(|e| {
        eprintln!("Failed to read metadata: {}", e);
        std::process::exit(1);
    });

    println!("{}", meta.info_summary());
}
