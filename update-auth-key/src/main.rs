use std::env;
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::process::Command;

extern crate clap;
extern crate chrono;
extern crate serde_json;
extern crate crypto_hash;
extern crate reqwest;

use clap::{Arg, App};
use chrono::prelude::*;
use chrono::DateTime;
use crypto_hash::{Algorithm, hex_digest};

type KeyMap = HashMap<String, String>;
type UserMap = HashMap<String, Vec<String>>;

macro_rules! log {
    ($($x:expr), *) => {
        let now: DateTime<Utc> = Utc::now().into();
        let time =  now.format("%FT%T.%3f").to_string();
        print!("{} ", &time);
        println!($($x), *);
    };
}

macro_rules! warn {
    ($($x:expr), *) => {
        let now: DateTime<Utc> = Utc::now().into();
        let time =  now.format("%FT%T.%3f").to_string();
        print!("{} [WARN] ", &time);
        println!($($x), *);
    };
}

macro_rules! error {
    ($($x:expr), *) => {
        let now: DateTime<Utc> = Utc::now().into();
        let time =  now.format("%FT%T.%3f").to_string();
        print!("{} [ERROR] ", &time);
        println!($($x), *);
    };
}

macro_rules! info {
    ($($x:expr), *) => {
        let now: DateTime<Utc> = Utc::now().into();
        let time =  now.format("%FT%T.%3f").to_string();
        print!("{} [INFO] ", &time);
        println!($($x), *);
    };
}


fn main()  {

    let m = App::new("updateauthkey")
        .version("0.0.1")
        .author("Karl Lam <karl.v.lam@gmail.com>")
        .arg(Arg::with_name("keyurl")
             .short("k")
             .long("keyurl")
             .value_name("key_file_url")
             .help("Key file URL")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("userurl")
             .short("u")
             .long("userurl")
             .value_name("user_file_url")
             .help("User file URL")
             .takes_value(true)
             .required(true))
        .get_matches();


    let key_url = m.value_of("keyurl").unwrap();
    let user_url = m.value_of("userurl").unwrap();

    let mut keys: KeyMap;
    let mut users: UserMap;

    match get_url(key_url) {
        Ok(Some(body)) => {
            match check_file_sum(body) {
                Some(key_json) => {
                    keys = serde_json::from_str(&key_json).unwrap();
                },
                None => { 
                    error!("Key checksum Error!!! {:#?}", &key_url);
                    std::process::exit(1);
                }
            }
        },
        _ => {
            error!("Get key_url Error!!! {:#?}", &key_url);
            std::process::exit(1);
        }
    };

    match get_url(user_url) {
        Ok(Some(body)) => {
            match check_file_sum(body) {
                Some(user_json) => {
                    users = serde_json::from_str(&user_json).unwrap();
                },
                None => { 
                    error!("User checksum Error!!! {:#?}", &user_url);
                    std::process::exit(1);
                }
            }
        },
        _ => {
            error!("Get user_url Error!!! {:#?}", &user_url);
            std::process::exit(1);
        }
    }

    update_authorized_keys(get_auth_keys(keys, users));

}

fn update_authorized_keys(key_map: HashMap<String, String>) {
    for user in key_map.keys() {
        //log!("{:#?}", &user);
        match get_home_dir(&user) {
            Some(home_dir) => {
                //log!("home_dir: {}", &home_dir);
                match key_map.get(user) {
                    Some(key_string) => {
                        //log!("key: {}", &key_string);
                        let key_file_path = Path::new(&home_dir).join(".ssh").join("authorized_keys");
                        let mut update_file = false;
                        match fs::read_to_string(&key_file_path) {
                            Err(e) => {
                                warn!("Key file does not exists! {:#?}", &key_file_path);
                                update_file = true;
                            },
                            Ok(ok) => {
                                //log!("Read OK! {:#?}", ok);
                                if ok.to_string() != key_string.to_string() {
                                    update_file = true;
                                }
                            },
                        }
                        if !update_file {
                            log!("Skipped unchanged key file: {:#?}", &key_file_path);
                        }else{
                            match fs::write(&key_file_path, Vec::from(key_string.to_string())) {
                                Err(e) => {
                                    error!("Write file {:#?} error: {:#?}", &key_file_path, e);
                                },
                                Ok(ok) => {
                                    info!("KEY_FILE_UPDATED: {:#?}", &key_file_path);
                                },
                            }
                        }
                    },
                    None => {
                    },
                }
            },
            None => {
                warn!("USER_OR_HOME_NOT_FOUND: {:#?}", &user);
            },
        }
    }
}

fn get_auth_keys(keys:KeyMap, users:UserMap) -> HashMap<String, String> {
    let mut user_map = HashMap::new();

    for u in users.keys() {
        let mut key_string = String::from("");
        for user in &users[u] {
            match keys.get(user) {
                Some(key_this) => {
                    key_string.push_str("\n");
                    key_string.push_str(key_this);
                },
                None => {
                    warn!{"USER_KEY_NOT_FOUND: USER => {:#?}, KEY => {:#?},", &u, &user}; 
                },
            }
            user_map.insert(u.to_string(), key_string.replace("\n\n", "\n"));

        }
    }

    user_map 
}

fn check_file_sum(mut file_text:String) -> Option<String>{
    let body = file_text.split_off(65);
    file_text.split_off(64);
    match file_text == hex_digest(Algorithm::SHA256, &Vec::from(body.to_string())) {
        true => { 
            Some(body)
        },
        false => { 
            None
        },
    }

}


fn get_url(url:&str) -> Result<Option<String>, Box<std::error::Error>> {
    let mut req = reqwest::get(url)?;
    match req.status().is_success() {
        true => {
            Ok(Some(req.text()?) )
        },
        false => Ok(None)
    }
}

fn get_home_dir(user:&String) -> Option<String>{
    //let mut r = Command::new("id").arg(user).output().unwrap();
    let r = Command::new("bash").arg("-c").arg(String::from("echo ~") + user).output().unwrap();
    let mut s = String::from_utf8_lossy(&r.stdout).to_string();
    s.truncate(s.len() - 1);
    let dir = Path::new(&s);
    //log!("{:#?}", &dir);
    match dir.is_dir() {
        true => Some(s),
        false =>  None,
    }
}
