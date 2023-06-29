use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Get(GetCommand),
    Set(SetCommand),
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

#[derive(Debug)]
pub struct VaultItem {
    pub login: String,
    pub password: String,
}

pub fn get_item(_name: String) {
    todo!()
}

pub fn set_item(_name: String, _login: String, _password: String) {
    todo!()
}
