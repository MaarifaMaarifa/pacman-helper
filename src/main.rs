mod args;

use args::*;
use clap::Parser;
use pacman_helper::commandline_functions::*;

fn main() {
    let args = args::Cli::parse();

    // Populating system packages
    let packages = populate_packages();

    match args.command {
        Commands::GetUniqueDeps(unique_deps) => {
            if let Some(unique_deps) = get_unique_dependencies(&unique_deps.package, packages) {
                for dep in unique_deps {
                    println!("{}", dep)
                }
            }
        }
        Commands::GetPacsWithSameDeps(pacs_with_same_deps) => {
            if let Some(packages) =
                get_packages_with_same_dependencies(&pacs_with_same_deps.package, &packages)
            {
                for package in packages {
                    println!("{}", package)
                }
            }
        }
    }
}
