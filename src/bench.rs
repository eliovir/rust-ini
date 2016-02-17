//#![feature(test)]

extern crate test;
use test::Bencher;

extern crate inifile;
use inifile::IniFile;

#[bench]
fn bench_inifile(b: &mut Bencher) {
	b.iter(|| {
		let mut ini = IniFile::new();
		ini.read("data/config.ini");
	});
}
