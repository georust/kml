# Changelog

## Unreleased

## [v0.8.1](https://github.com/georust/kml/releases/tag/v0.8.1)

- Fix parsing of `hotSpot` in `IconStyle` ([#53](https://github.com/georust/kml/pull/53)) from [@adrianhoppe](https://github.com/adrianhoppe)
- Disable unneeded aes-crypto feature for zip ([#52](https://github.com/georust/kml/pull/52)) from [@Dosenpfand](https://github.com/Dosenpfand)
- Clean up code examples ([#51](https://github.com/georust/kml/pull/51)) from [@Dosenpfand](https://github.com/Dosenpfand)
- Fix default implementation on enums ([#50](https://github.com/georust/kml/pull/50)) from [@luke-biel](https://github.com/luke-biel)))

## [v0.8.0](https://github.com/georust/kml/releases/tag/v0.8.0)

- Updated `quick-xml` and `zip` dependencies ([#48](https://github.com/georust/kml/pull/48)) from [@luke-biel](https://github.com/luke-biel)

## [v0.7.1](https://github.com/georust/kml/releases/tag/v0.7.1)

- Handle invalid UTF-8 characters in tag name and attributes ([#45](https://github.com/georust/kml/pull/45))

## [v0.7.0](https://github.com/georust/kml/releases/tag/v0.7.0)

- BREAKING: Updates `Style`, `StyleMap`, `BalloonStyle`, `IconStyle`, `Icon`, `LabelStyle`, `LineStyle`, `PolyStyle`, and `ListStyle` by adding public `attrs` property and changes type of `id` from `String` to `Option<String>` to more closely reflect the specification.
- Add `categories` field to `Cargo.toml`

## [v0.6.0](https://github.com/georust/kml/releases/tag/v0.6.0)

- Ignored deprecated code warnings for `geo_types::Coordinate` since we're supporting multiple versions of `geo-types` ([#40](https://github.com/georust/kml/pull/40))
- Fixed attributes not being written for `kml:Scale`, `kml:Orientation`, `kml:Point`, `kml:Location`, `kml:LineString`, `kml:LinearRing`, and `kml:Placemark`  ([#38](https://github.com/georust/kml/pull/38))
- Made license field SPDX compliant ([#36](https://github.com/georust/kml/pull/36))
- Added `kml:SchemaData`, `kml:SimpleData`, and `kml:SimpleArrayData` ([#35](https://github.com/georust/kml/pull/35)) from [@k-mack](https://github.com/k-mack)
- Fix clippy warnings from stable Rust 1.63.0 ([#31](https://github.com/georust/kml/pull/31)) from [@k-mack](https://github.com/k-mack)
- Added `kml:Alias` and `kml:ResourceMap` ([#29](https://github.com/georust/kml/pull/29)) from [@k-mack](https://github.com/k-mack)

## [v0.5.0](https://github.com/georust/kml/releases/tag/v0.5.0)

- Run tests without default features ([#26](https://github.com/georust/kml/pull/26))
- Adds `kml:Link` and `kml:Icon` ([#28](https://github.com/georust/kml/pull/28)) from [@k-mack](https://github.com/k-mack)
- Updates edition to 2021 ([#27](https://github.com/georust/kml/pull/27))

## [v0.4.3](https://github.com/georust/kml/releases/tag/v0.4.3)

- Fixes serialization order of some geometry elements to match the sequence in the [specification](http://schemas.opengis.net/kml/2.2.0/ogckml22.xsd) ([#25](https://github.com/georust/kml/pull/25)) from [@blipmusic](https://github.com/blipmusic)

## [v0.4.2](https://github.com/georust/kml/releases/tag/v0.4.2)

- Updates `quick-xml` to [v0.22](https://github.com/tafia/quick-xml/blob/master/Changelog.md#0220) which doesn't expose any breaking changes for this API
- Fix compilation issue when `zip` feature is disabled ([#22](https://github.com/georust/kml/pull/22)) from [@vilaureu](https://github.com/vilaureu)

## [v0.4.1](https://github.com/georust/kml/releases/tag/v0.4.1)

- Fix issue with sibling `kml:Folder` or `kml:Document` elements nesting ([#19](https://github.com/georust/kml/pull/19))

## [v0.4.0](https://github.com/georust/kml/releases/tag/v0.4.0)

- Clippy cleanup ([#3](https://github.com/georust/kml/pull/3))
- Add support for `kml:Location` ([#7](https://github.com/georust/kml/pull/7)) from [@Nadhum](https://github.com/Nadhum)
- Add support for `kml:Scale` ([#8](https://github.com/georust/kml/pull/8)) from [@Nadhum](https://github.com/Nadhum)
- Add support for `kml:Orientation` ([#8](https://github.com/georust/kml/pull/9)) from [@Nadhum](https://github.com/Nadhum)
- Require clippy and rustfmt in CI ([#14](https://github.com/georust/kml/pull/14))
- Add support for `hotSpot` element within `kml:IconStyle`, including a new `Units` enum and `Vec2` struct ([#13](https://github.com/georust/kml/pull/13)) from [@ardouglas](https://github.com/ardouglas)

## [v0.3.1](https://github.com/georust/kml/releases/tag/v0.3.1)

- Handle UTF-8 decoding issues without a panic, fixing [#1](https://github.com/georust/kml/issues/1)

## [v0.3.0](https://github.com/georust/kml/releases/tag/v0.3.0)

- Cleaned up method names (i.e. "parse*" to "read*")
- Added `KmlWriter::from_writer`
- Update license to MIT/Apache-2.0
- Transfer to georust

## [v0.2.0](https://github.com/georust/kml/releases/tag/v0.2.0)

- Initial functionality for reading, writing, and conversion
