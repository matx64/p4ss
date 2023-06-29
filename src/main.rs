use clap::Parser;
use p4ss::{delete_item, get_item, set_item, Command};

#[derive(Parser, Debug)]
#[command(name = "p4ss")]
#[command(about = "A simple terminal password manager.")]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Get(args) => {
            get_item(args.name);
        }
        Command::Set(args) => {
            set_item(args.name, args.login, args.password);
        }
        Command::Del(args) => {
            delete_item(args.name);
        }
    }
}
