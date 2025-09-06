use std::io::{self, Write};

struct App {
    target_file_path: String,
}

impl App {
    pub fn get_user_input(prompt: &str) -> String {
        print!("{prompt}");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    
        input.trim().to_string()
    }
}

struct MetadataLogic {
    metadata: Box<rexiv2::Metadata>
}

impl MetadataLogic {
    pub fn new() -> MetadataLogic {
        MetadataLogic {
            metadata: Box::new(Self::get_raw_metadata())
        }
    }
    
    fn get_file_type() -> {}

    fn get_raw_metadata() -> rexiv2::Metadata {

        let app = App {
            target_file_path: App::get_user_input("Targetted file to get metadata: ")
        };

        match rexiv2::Metadata::new_from_path(&app.target_file_path) {
            Ok(metadata) => metadata,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn get_all_metadata_info(&self) -> String {
        format!("File Type: {:?}\nSize: {}x{}\n", self.metadata.get_media_type(), self.metadata.get_pixel_width(), self.metadata.get_pixel_height())
    }
}

fn main() {
    let meta = MetadataLogic::new();
    println!("{}", meta.get_all_metadata_info());
}
