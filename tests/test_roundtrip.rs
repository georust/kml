#[cfg(test)]
mod roundtrip_tests {
    use kml::Kml;
    use std::fs::File;
    use std::io::prelude::*;

    // Based on roundtrip tests in georust/geojson
    macro_rules! roundtrip_test {
        ($name:ident : $file_name:expr) => {
            #[test]
            fn $name() {
                let fixture_dir_path = "tests/fixtures/";
                let mut file_path = fixture_dir_path.to_owned();
                file_path.push_str($file_name.to_owned().as_str());

                test_round_trip(&file_path);
            }
        };
    }

    macro_rules! roundtrip_tests {
        ( $($name:ident: $file_name:expr,)* ) => {
            $(
                roundtrip_test!($name: $file_name);
             )*
        }
    }

    roundtrip_tests! {
        test_polygon: "polygon.kml",
        test_sample: "sample.kml",
        test_countries: "countries.kml",
    }

    // Confirms that parsing from KML and writing back doesn't drop any currently tracked data
    fn test_round_trip(file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        // Read and parse the KML from the file's contents
        let original_kml = file_contents.parse::<Kml>().expect("unable to parse");

        // Convert to a string and re-parse to make sure nothing we're watching was lost
        let kml_str = original_kml.to_string();

        let roundtrip_kml: Kml = kml_str.parse().unwrap();

        assert_eq!(original_kml, roundtrip_kml)
    }
}
