# IniFile

[![Build Status](https://travis-ci.org/eliovir/rust-ini.png)](https://travis-ci.org/eliovir/rust-ini)


This crate provides a handler for INI files.

TODO : make it works with the `Encoder` and `Decoder` traits in Rust's `serialize` crate.

## Usage


```rust
extern crate inifile;

use inifile::IniFile;

fn main() {
	let mut config = IniFile::new();
	config.read("data/config.ini");
	let found = config.get_f64("Floats", "float01");
	assert_eq!(found, 0.1);
}

```

## Version policy

This library follows [semver](http://semver.org), with a notable, but temporary
exception, as Rust is currently still in flux:

Changes to maintain compatibility with the current development state of Rust are
considered patches, as long as the resulting API stays faithful to the previous
API and doesn't add or loose any features.

## License

MIT, see `LICENSE`

