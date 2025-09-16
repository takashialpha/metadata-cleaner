use gtk::prelude::*;
use gtk::{Box as GtkBox, Label};
use gtk4 as gtk;
use num_traits::ToPrimitive;

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
    fn format_tags_list(tags: &[String]) -> String {
        if tags.is_empty() {
            "N/A".to_string()
        } else {
            tags.join(", ")
        }
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
            exif_tags: raw_metadata
                .get_exif_tags()
                .unwrap_or_default()
                .into_iter()
                .map(|t| {
                    raw_metadata
                        .get_tag_interpreted_string(&t)
                        .unwrap_or_else(|_| "N/A".to_string())
                })
                .collect(),
            iptc_tags: raw_metadata
                .get_iptc_tags()
                .unwrap_or_default()
                .into_iter()
                .map(|t| {
                    raw_metadata
                        .get_tag_interpreted_string(&t)
                        .unwrap_or_else(|_| "N/A".to_string())
                })
                .collect(),
            xmp_tags: raw_metadata
                .get_xmp_tags()
                .unwrap_or_default()
                .into_iter()
                .map(|t| {
                    raw_metadata
                        .get_tag_interpreted_string(&t)
                        .unwrap_or_else(|_| "N/A".to_string())
                })
                .collect(),
        }
    }

    pub fn field_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::with_capacity(12);

        pairs.push((
            "Media Type".to_string(),
            self.media_type
                .as_ref()
                .map_or("N/A".to_string(), |v| format!("{:?}", v)),
        ));

        let width_str = self
            .pixel_width
            .map_or("N/A".to_string(), |w| w.to_string());
        let height_str = self
            .pixel_height
            .map_or("N/A".to_string(), |h| h.to_string());
        pairs.push((
            "Dimensions".to_string(),
            format!("{} x {}", width_str, height_str),
        ));

        pairs.push((
            "Exposure Time".to_string(),
            self.exposure_time
                .map_or("N/A".to_string(), |v| v.to_string()),
        ));
        pairs.push((
            "F-Number".to_string(),
            self.fnumber.map_or("N/A".to_string(), |v| v.to_string()),
        ));
        pairs.push((
            "Focal Length".to_string(),
            self.focal_length
                .map_or("N/A".to_string(), |v| v.to_string()),
        ));
        pairs.push((
            "ISO Speed".to_string(),
            self.iso_speed.map_or("N/A".to_string(), |v| v.to_string()),
        ));
        pairs.push((
            "Orientation".to_string(),
            self.orientation
                .as_ref()
                .map_or("N/A".to_string(), |v| format!("{:?}", v)),
        ));
        pairs.push((
            "GPS Info".to_string(),
            self.gps_info
                .as_ref()
                .map_or("N/A".to_string(), |v| format!("{:?}", v)),
        ));

        pairs.push((
            "EXIF Tags".to_string(),
            Self::format_tags_list(&self.exif_tags),
        ));
        pairs.push((
            "IPTC Tags".to_string(),
            Self::format_tags_list(&self.iptc_tags),
        ));
        pairs.push((
            "XMP Tags".to_string(),
            Self::format_tags_list(&self.xmp_tags),
        ));

        pairs
    }

    pub fn to_widget(&self) -> GtkBox {
        let vbox = GtkBox::new(gtk::Orientation::Vertical, 5);

        let add_label = |container: &GtkBox, key: &str, value: &str| {
            let text = format!("{}: {}", key, value);
            let label = Label::new(Some(&text));
            label.set_xalign(0.0);
            label.set_wrap(true);
            label.set_wrap_mode(gtk::pango::WrapMode::WordChar);
            container.append(&label);
        };

        for (k, v) in self.field_pairs() {
            match k.as_str() {
                "EXIF Tags" | "IPTC Tags" | "XMP Tags" => {
                    if v != "N/A" {
                        let tags: Vec<&str> = v.split(',').map(|s| s.trim()).collect();
                        let title = format!("{} ({})", k, tags.len());

                        let expander = gtk::Expander::new(Some(&title));
                        let inner = GtkBox::new(gtk::Orientation::Vertical, 2);

                        for tag_line in tags {
                            let label = Label::new(Some(tag_line));
                            label.set_xalign(0.0);
                            label.set_wrap(true);
                            label.set_wrap_mode(gtk::pango::WrapMode::WordChar);
                            inner.append(&label);
                        }

                        expander.set_child(Some(&inner));
                        vbox.append(&expander);
                    }
                }
                _ => add_label(&vbox, &k, &v),
            }
        }

        vbox
    }
}
