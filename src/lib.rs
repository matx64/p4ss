use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Get(GetCommand),
    Set(SetCommand),
    Del(DeleteCommand),
}

#[derive(Debug, Args)]
pub struct GetCommand {
    #[arg()]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct SetCommand {
    #[arg()]
    pub name: String,

    #[arg()]
    pub login: String,

    #[arg()]
    pub password: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[arg()]
    pub name: String,
}

#[derive(Debug)]
pub struct VaultItem {
    pub login: String,
    pub password: String,
}

pub fn get_item(name: String) {
    let path = format!("./vault/{name}");

    if !Path::new(&path).is_file() {
        println!("Item doesn't exist.");
        return;
    }

    let contents = fs::read_to_string(&path).unwrap();

    let mut i: u8 = 0;
    for line in contents.lines() {
        if i == 0 {
            println!("\n[login]:\t {}", line);
            i += 1;
        } else {
            println!("[password]:\t {}\n", line);
            break;
        }
    }
}

pub fn set_item(name: String, login: String, password: String) {
    if !Path::new("./vault").is_dir() {
        fs::create_dir("./vault").unwrap();
    }

    let contents = format!("{login}\n{password}");

    let mut file = File::create("./vault/".to_owned() + &name).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("{} credentials set.", name.to_uppercase());
}

pub fn delete_item(name: String) {
    let path = format!("./vault/{name}");

    if !Path::new(path.as_str()).is_file() {
        println!("Item doesn't exist.");
        return;
    }

    fs::remove_file(&path).unwrap();

    println!("{} deleted.", name.to_uppercase());
}
