use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

extern crate clap;
extern crate serde_json;
extern crate crypto_hash;

use clap::{Arg, App};
use crypto_hash::{Algorithm, hex_digest};


fn main() {
    let m = App::new("gen-key-file")
        .version("0.0.1")
        .author("Karl Lam <karl.v.lam@gmail.com>")
        .arg(Arg::with_name("key_dir")
             .short("k")
             .long("keydir")
             .value_name("key_dir")
             .help("Key directory")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("key_file")
             .short("o")
             .long("out")
             .value_name("key_file")
             .help("Key file name")
             .takes_value(true)
             .required(true))
        .get_matches();

    let key_dir = m.value_of("key_dir").unwrap();
    let key_file = m.value_of("key_file").unwrap();


    let mut keys = BTreeMap::new();

    for path in fs::read_dir(&key_dir).unwrap() {
        let key = String::from(path.unwrap().file_name().to_string_lossy());
        let value = fs::read_to_string(Path::new(&key_dir).join(&key)).unwrap();
        keys.insert(key, value);
    }

    let output_key = serde_json::to_string_pretty(&keys).unwrap();

    let digest = hex_digest(Algorithm::SHA256, &Vec::from(output_key.to_string()));

    println!("{}", digest);
    println!("{}", &output_key);

    let mut out = Vec::from(digest + "\n");
    out.append(&mut Vec::from(output_key));

    match fs::write(&key_file, out) {
        Ok(_) => {
            println!("Key file created: {}", &key_file);
        },
        Err(e) => {
            println!("[ERROR] Key file create failed: {:#?}", e);
            std::process::exit(1);
        }
    }
    
}
