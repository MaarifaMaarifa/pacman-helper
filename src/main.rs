use pac_helper::package_database_reader::packages_reader;
use pac_helper::option_functions::get_unique_dependencies;

use std::process;

fn main() {
    let packages = match packages_reader("/var/lib/pacman/local/") {
        Ok(packages) => packages,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    
    // for package in &packages {
    //     match get_unique_dependencies(&package.name, &packages) {
    //         Some(_) => continue,
    //         None => println!("{}", package.name)
    //     }
    // }

    let pacs = get_unique_dependencies("libreoffice-still", &packages).unwrap();

    for pac in &pacs {
        println!("{}", pac);
    }
    println!("{}",pacs.len())
    
}
