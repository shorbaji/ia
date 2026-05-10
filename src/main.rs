mod login;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ia", version, about = "insaali CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Sign in via browser and save a token at ~/.config/insaali/credentials.
    Login,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => println!("hello, world!"),
        Some(Commands::Login) => match login::run() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("login failed: {e}");
                std::process::exit(1);
            }
        },
    }
}
