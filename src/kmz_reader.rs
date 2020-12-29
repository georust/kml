use std::fmt::Debug;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::str::FromStr;

use num_traits::Float;
use zip::ZipArchive;

use crate::errors::Error;
use crate::reader::KmlReader;

#[cfg_attr(docsrs, doc(cfg(feature = "zip")))]
impl<T> KmlReader<Cursor<Vec<u8>>, T>
where
    T: Float + FromStr + Default + Debug,
{
    pub fn from_kmz_file<P: AsRef<Path>>(path: P) -> Result<KmlReader<Cursor<Vec<u8>>, T>, Error> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;

        // Should parse the first file with a KML extension
        for i in 0..archive.len() {
            let mut kml_file = archive.by_index(i).map_err(|_| Error::InvalidInput)?;
            if !kml_file.name().to_ascii_lowercase().ends_with(".kml") {
                continue;
            }
            let mut buf = Vec::with_capacity(kml_file.size() as usize);
            std::io::copy(&mut kml_file, &mut buf)?;
            return Ok(KmlReader::from_reader(Cursor::new(buf)));
        }

        Err(Error::InvalidInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Kml;

    #[test]
    fn test_parse_kmz() {
        let kmz_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("polygon.kmz");
        let mut kml_reader = KmlReader::<_, f64>::from_kmz_file(kmz_path).unwrap();
        let kml = kml_reader.parse().unwrap();

        assert!(matches!(kml, Kml::Polygon(_)))
    }
}
