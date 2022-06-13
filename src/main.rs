use pac_helper::commandline::run;
use pac_helper::package_database_reader::packages_reader;

use std::env;
use std::process;

fn main() {
    let packages = match packages_reader("/var/lib/pacman/local/") {
        Ok(packages) => packages,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let args: Vec<String> = env::args().into_iter().collect();

    if let Err(err) = run(args, &packages) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
