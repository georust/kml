use quick_xml::Reader;

pub struct KmlReader {
    reader: Reader;
}

impl KmlReader {
    fn from_str(s: &str) -> Self {
        KmlReader {
            reader: Reader::from_str(s)
        }
    }
}
