#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! yaml-rust = "0.4"
//! linked-hash-map = "0.5.1"
//! ```

// This is not intended for production… rewrite it it in a saner way, in a language that fits better into the ecosystem.
// https://github.com/ImmobilienScout24/yamlreader may be a good alternative, but uses different merge semantics.

use std::env;
use std::fs::File;
use std::io::prelude::*;
extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
extern crate linked_hash_map;
use linked_hash_map::Entry as LHMEntry;

fn main() {
    let docg = env::args().skip(1).flat_map(|filename| {
		let mut f = File::open(&filename).expect(&(filename + " could not be opened"));
		let mut contents = String::new();
		f.read_to_string(&mut contents).expect("something went wrong reading the YAML file");
		let docs = YamlLoader::load_from_str(&contents).unwrap();
		return docs;
	});

	let merge = docg.fold(Yaml::Null, merge_yaml);
	
	let mut out_str = String::new();
	{
		let mut emitter = YamlEmitter::new(&mut out_str);
		emitter.dump(&merge).unwrap();
	}
	println!("{}", out_str);
}

fn merge_yaml(merge: Yaml, doc: Yaml) -> Yaml {
	match doc {
		Yaml::Array(a) => match merge {
			Yaml::Array(ao) => {
				Yaml::Array(ao.into_iter().chain(a).collect())
			},
			_ => Yaml::Array(a)
		},
		Yaml::Hash(h) => match merge {
			Yaml::Hash(mut ho) => {
				for (k,v) in h.into_iter() { match ho.entry(k) {
					LHMEntry::Vacant(mut e) => { e.insert(v); },
					LHMEntry::Occupied(mut e) => {
						let m = merge_yaml(e.insert(Yaml::BadValue), v); // Creating a vacant bucket by using e.remove() is sadly not possible, so swap with a dummy value
						if m == Yaml::Null {
							e.remove();
						} else {
							e.insert(m);
						}
					}
				}}
				Yaml::Hash(ho)
			},
			_ => Yaml::Hash(h)
		},
		_ => doc
	}
}
