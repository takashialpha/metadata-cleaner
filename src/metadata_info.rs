use libadwaita as adw;

use adw::gtk as gtk;
use adw::gtk::prelude::*;
use adw::gtk::{Box as GtkBox, Label};
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
    pub exif_tags: Vec<(String, String)>,
    pub iptc_tags: Vec<(String, String)>,
    pub xmp_tags: Vec<(String, String)>,
}

impl MetadataInfo {
    fn format_tags(raw_metadata: &rexiv2::Metadata, tags: &[String]) -> Vec<(String, String)> {
        tags.iter()
            .map(|tag| {
                let value = raw_metadata
                    .get_tag_interpreted_string(tag)
                    .unwrap_or_else(|_| "N/A".to_string());
                (tag.clone(), value)
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
            exif_tags: Self::format_tags(
                raw_metadata,
                &raw_metadata.get_exif_tags().unwrap_or_default(),
            ),
            iptc_tags: Self::format_tags(
                raw_metadata,
                &raw_metadata.get_iptc_tags().unwrap_or_default(),
            ),
            xmp_tags: Self::format_tags(
                raw_metadata,
                &raw_metadata.get_xmp_tags().unwrap_or_default(),
            ),
        }
    }

    pub fn to_widget(&self) -> gtk::ScrolledWindow {
        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled.set_min_content_height(500);
        scrolled.set_min_content_width(700);

        let outer = GtkBox::new(gtk::Orientation::Vertical, 16);
        outer.set_margin_top(16);
        outer.set_margin_bottom(16);
        outer.set_margin_start(16);
        outer.set_margin_end(16);

        let general_frame = gtk::Frame::new(Some("General Info"));
        let grid = gtk::Grid::new();
        grid.set_row_spacing(6);
        grid.set_column_spacing(12);
        grid.set_margin_top(8);
        grid.set_margin_bottom(8);
        grid.set_margin_start(8);
        grid.set_margin_end(8);

        let mut row = 0;
        let add_row = |grid: &gtk::Grid, r: &mut i32, key: &str, val: String| {
            let key_label = Label::new(None);
            key_label.set_markup(&format!("<b>{}</b>", key));
            key_label.set_xalign(0.0);

            let val_label = Label::new(Some(&val));
            val_label.set_xalign(0.0);
            val_label.set_wrap(true);
            val_label.set_wrap_mode(gtk::pango::WrapMode::WordChar);

            grid.attach(&key_label, 0, *r, 1, 1);
            grid.attach(&val_label, 1, *r, 1, 1);
            *r += 1;
        };

        add_row(
            &grid,
            &mut row,
            "Media Type",
            self.media_type
                .as_ref()
                .map(|v| format!("{:?}", v))
                .unwrap_or("N/A".to_string()),
        );
        add_row(
            &grid,
            &mut row,
            "Dimensions",
            format!(
                "{} x {}",
                self.pixel_width.unwrap_or(0),
                self.pixel_height.unwrap_or(0)
            ),
        );
        if let Some(val) = self.exposure_time {
            add_row(&grid, &mut row, "Exposure Time", val.to_string());
        }
        if let Some(val) = self.fnumber {
            add_row(&grid, &mut row, "F-Number", val.to_string());
        }
        if let Some(val) = self.focal_length {
            add_row(&grid, &mut row, "Focal Length", val.to_string());
        }
        if let Some(val) = self.iso_speed {
            add_row(&grid, &mut row, "ISO Speed", val.to_string());
        }
        if let Some(val) = &self.orientation {
            add_row(&grid, &mut row, "Orientation", format!("{:?}", val));
        }
        if let Some(val) = &self.gps_info {
            add_row(&grid, &mut row, "GPS Info", format!("{:?}", val));
        }

        general_frame.set_child(Some(&grid));
        outer.append(&general_frame);

        let add_tag_section = |outer: &GtkBox, title: &str, tags: &[(String, String)]| {
            if !tags.is_empty() {
                let frame = gtk::Frame::new(Some(title));
                frame.set_margin_top(8);

                let expander = gtk::Expander::new(Some(&format!("{} ({})", title, tags.len())));
                let listbox = gtk::ListBox::new();

                for (tag, value) in tags {
                    let row_label = Label::new(None);
                    row_label.set_markup(&format!("<tt>{}</tt> = {}", tag, value));
                    row_label.set_xalign(0.0);
                    row_label.set_wrap(true);
                    row_label.set_wrap_mode(gtk::pango::WrapMode::WordChar);

                    let row = gtk::ListBoxRow::new();
                    row.set_child(Some(&row_label));
                    listbox.append(&row);
                }

                expander.set_child(Some(&listbox));
                frame.set_child(Some(&expander));
                outer.append(&frame);
            }
        };

        add_tag_section(&outer, "EXIF Tags", &self.exif_tags);
        add_tag_section(&outer, "IPTC Tags", &self.iptc_tags);
        add_tag_section(&outer, "XMP Tags", &self.xmp_tags);

        scrolled.set_child(Some(&outer));
        scrolled
    }
}
