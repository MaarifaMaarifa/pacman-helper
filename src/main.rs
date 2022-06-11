use pac_helper::arch_packages::{Package, Dependency};
use pac_helper::package_database_reader::packages_reader;

use std::process;

fn main() {
    let packages = match packages_reader("/var/lib/pacman/local/") {
        Ok(packages) => packages,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    for package in &packages {
        println!("{}", package.name);
    }

    println!("Total packages installed = {}", packages.len());
}
