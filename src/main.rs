mod args;

use args::*;
use clap::Parser;
use pacman_helper::commandline_functions::*;

fn main() {
    let args = args::Cli::parse();

    match args.command {
        Commands::GetUniqueDeps(unique_deps) => {
            get_unique_dependencies(&unique_deps.package);
        }
        Commands::GetPacsWithSameDeps(pacs_with_same_deps) => {
            get_packages_with_same_dependencies(&pacs_with_same_deps.package);
        }
    }
}
