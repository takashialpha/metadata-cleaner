use crate::error::AppError;

pub struct MetadataWriter;

impl MetadataWriter {
    pub fn clear_file(raw_metadata: &rexiv2::Metadata, file_path: &str) {
        raw_metadata.clear();
        if let Err(e) = raw_metadata.save_to_file(file_path) {
            AppError::error_exit(AppError::FailedWritingMetadata(e));
        }
    }
}
