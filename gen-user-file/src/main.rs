use std::fs;
use std::path::Path;
use std::collections::HashMap;

extern crate clap;
extern crate serde_json;
extern crate crypto_hash;

use clap::{Arg, App};
use crypto_hash::{Algorithm, hex_digest};


fn main() {
    let mut keys = HashMap::new();

    let m = App::new("gen-user-file")
        .version("0.0.1")
        .author("Karl Lam <karl.v.lam@gmail.com>")
        .arg(Arg::with_name("user_dir")
             .short("u")
             .long("userdir")
             .value_name("user_dir")
             .help("User directory")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("user_file")
             .short("o")
             .long("out")
             .value_name("user_file")
             .help("User file name")
             .takes_value(true)
             .required(true))
        .get_matches();


    let user_dir = m.value_of("user_dir").unwrap();
    let user_file = m.value_of("user_file").unwrap();

    match fs::read_dir(&user_dir) {
        Ok(r) => {
            //println!("{:#?}", r);
            for path in r {
                let user = String::from(path.unwrap().file_name().to_string_lossy());
                let content = fs::read_to_string(Path::new(&user_dir).join(&user)).unwrap();
                let v: Vec<&str> = content.split("\n").collect();
                //println!("{:#?}", &user );
                //println!("{:#?}", &v );
                let mut out: Vec<String> = Vec::new();
                for k in v {
                    match k.trim() {
                        "" => {},
                        _t => { out.push(String::from(_t)); },
                    }; 

                }

                keys.insert(user, out);
            }
        }, 
        Err(e) => { 
            println!("{:#?}", e);
            std::process::exit(1);
        },
    }

    let output_key = serde_json::to_string_pretty(&keys).unwrap();
    let digest = hex_digest(Algorithm::SHA256, &Vec::from(output_key.to_string()));
    let mut out = Vec::from(digest.to_string() + "\n");
    out.append(&mut Vec::from(output_key.to_string()));
    println!("{}", &digest);
    println!("{}", &output_key);
    match fs::write(user_file, out) {
        Ok(_) => {
            println!("User file created: {}", &user_file);
        },
        Err(e) => {
            println!("[ERROR] User file create failed: {:#?}", e);
            std::process::exit(1);
        }
    }
     
}
