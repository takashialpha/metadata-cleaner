# TODO List

- [x] add a field in a struct for each metadata
- [ ] detect all metadata
- [x] add more formats support
- [ ] modularize and fix the code for some metadata
- [ ] add more files support
- [ ] add a gtk gui
- [ ] add workflow; to do packaging using flatpak

## metadata docs

### EXIF
- `Exif.Image.Make` → Camera manufacturer
- `Exif.Image.Model` → Camera model
- `Exif.Photo.DateTimeOriginal` → Original capture date/time
- `Exif.Photo.FNumber` → Aperture
- `Exif.Photo.ExposureTime` → Shutter speed
- `Exif.Photo.ISOSpeedRatings` → ISO value
- `Exif.GPSInfo.GPSLatitude` → GPS latitude
- `Exif.GPSInfo.GPSLongitude` → GPS longitude

### IPTC
- `Iptc.Application2.ObjectName` → Image title
- `Iptc.Application2.Keywords` → Keywords
- `Iptc.Application2.Caption` → Caption/description
- `Iptc.Application2.Byline` → Photographer/author
- `Iptc.Application2.City` → City
- `Iptc.Application2.CountryName` → Country

### XMP
- `Xmp.dc.title` → Title
- `Xmp.dc.creator` → Author
- `Xmp.dc.subject` → Keywords
- `Xmp.photoshop.DateCreated` → Creation date
- `Xmp.photoshop.City` → City
- `Xmp.photoshop.Country` → Country
