use num_traits::ToPrimitive;
use std::fmt;

pub struct MetadataInfo {
    pub media_type: Option<rexiv2::MediaType>,
    pub pixel_width: Option<i32>,
    pub pixel_height: Option<i32>,
    pub exposure_time: Option<f64>,
    pub fnumber: Option<f64>,
    pub focal_length: Option<f64>,
    pub iso_speed: Option<i32>,
    pub gps_info: Option<rexiv2::GpsInfo>,
    pub orientation: Option<rexiv2::Orientation>,
    pub exif_tags: Vec<String>,
    pub iptc_tags: Vec<String>,
    pub xmp_tags: Vec<String>,
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

    pub fn from(raw_metadata: &rexiv2::Metadata) -> MetadataInfo {
        MetadataInfo {
            media_type: raw_metadata.get_media_type().ok(),
            pixel_width: Some(raw_metadata.get_pixel_width()),
            pixel_height: Some(raw_metadata.get_pixel_height()),
            exposure_time: raw_metadata.get_exposure_time().and_then(|r| r.to_f64()),
            fnumber: raw_metadata.get_fnumber(),
            focal_length: raw_metadata.get_focal_length(),
            iso_speed: raw_metadata.get_iso_speed(),
            gps_info: raw_metadata.get_gps_info(),
            orientation: Some(raw_metadata.get_orientation()),
            exif_tags: Self::format_tags(raw_metadata, &raw_metadata.get_exif_tags().unwrap_or_default()),
            iptc_tags: Self::format_tags(raw_metadata, &raw_metadata.get_iptc_tags().unwrap_or_default()),
            xmp_tags: Self::format_tags(raw_metadata, &raw_metadata.get_xmp_tags().unwrap_or_default()),
        }
    }
}

impl fmt::Display for MetadataInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_opt<T: ToString>(val: Option<T>) -> String {
            val.map_or("N/A".to_string(), |v| v.to_string())
        }

        writeln!(f, "Media Type: {}", self.media_type.as_ref().map_or("N/A".to_string(), |v| format!("{:?}", v)))?;
        writeln!(f, "Dimensions: {} x {}", self.pixel_width.unwrap_or(0), self.pixel_height.unwrap_or(0))?;
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

