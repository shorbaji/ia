mod api;
mod credentials;
mod login;
mod logout;
mod runs;

use clap::{Parser, Subcommand, ValueEnum};

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
    /// Start a run.
    Run {
        #[command(subcommand)]
        target: RunTarget,
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

#[derive(Subcommand)]
enum RunTarget {
    /// Run a simeval episode (one policy in one simulator).
    Simeval {
        /// Compute backend to execute the run on.
        #[arg(long, value_enum)]
        backend: Backend,
        /// Simulator (gymnasium env id, e.g. HalfCheetah-v5).
        #[arg(long, default_value = "HalfCheetah-v5")]
        sim: String,
        /// Policy reference (hf://owner/repo).
        #[arg(long)]
        policy: String,
        /// Maximum environment steps per episode.
        #[arg(long, default_value_t = 100)]
        max_steps: u32,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Backend {
    /// Submit as an Anyscale Job.
    Anyscale,
    /// Submit as a KubeRay RayJob on the insaali GKE cluster.
    K8s,
}

impl Backend {
    fn as_str(self) -> &'static str {
        match self {
            Backend::Anyscale => "anyscale",
            Backend::K8s => "k8s",
        }
    }
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
            target:
                RunTarget::Simeval {
                    backend,
                    sim,
                    policy,
                    max_steps,
                },
        }) => runs::simeval(&sim, &policy, backend.as_str(), max_steps),
        Some(Commands::Status { run_id }) => runs::status(&run_id),
        Some(Commands::Logs { run_id }) => runs::logs(&run_id),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
