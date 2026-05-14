mod api;
mod credentials;
mod login;
mod logout;
mod runs;

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
    /// Sign out by removing the saved token.
    Logout,
    /// Start a simulation run.
    Run {
        /// Simulator (e.g. mujoco).
        #[arg(long)]
        sim: String,
        /// Policy reference (e.g. hf://owner/model).
        #[arg(long)]
        policy: String,
        /// Compute backend.
        #[arg(long, default_value = "insaali")]
        compute_backend: String,
    },
    /// Show status of a run.
    Status {
        /// Run id.
        run_id: String,
    },
    /// Print accumulated logs for a run.
    Logs {
        /// Run id.
        run_id: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        None => {
            println!("hello, world!");
            Ok(())
        }
        Some(Commands::Login) => login::run(),
        Some(Commands::Logout) => logout::run(),
        Some(Commands::Run {
            sim,
            policy,
            compute_backend,
        }) => runs::run(&sim, &policy, &compute_backend),
        Some(Commands::Status { run_id }) => runs::status(&run_id),
        Some(Commands::Logs { run_id }) => runs::logs(&run_id),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
