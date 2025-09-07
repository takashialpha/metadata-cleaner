use crate::error::AppError;

pub struct MetadataReader {
    pub raw_metadata: rexiv2::Metadata,
}

impl MetadataReader {
    pub fn from_path(file_path: &str) -> Result<MetadataReader, AppError> {
        Ok(MetadataReader {
            raw_metadata: Self::load_metadata(file_path)?,
        })
    }

    fn load_metadata(file_path: &str) -> Result<rexiv2::Metadata, AppError> {
        rexiv2::Metadata::new_from_path(file_path).map_err(AppError::FailedReadingMetadata)
    }
}

