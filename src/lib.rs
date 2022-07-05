pub mod arch_package {

    use std::fs;

    // Custom type for an arch package
    pub struct Package {
        pub name: String,
        pub version: String,
        pub description: String,
        pub dependencies: Vec<String>,
    }

    impl Package {
        pub fn new() -> Package {
            Package {
                name: String::new(),
                version: String::new(),
                description: String::new(),
                dependencies: Vec::new(),
            }
        }

        // Method to populate package fields, by reading their respective local databases
        pub fn init(&mut self, path: String) -> Result<(), &str> {
            let database_contents = match fs::read_to_string(path) {
                Ok(contents) => contents,
                Err(_) => return Err("Failed to read the package database"),
            };

            let mut take_name = false;
            let mut take_version = false;
            let mut take_description = false;
            let mut take_dependencies = false;

            for line in database_contents.lines() {
                if line.contains("%NAME%") {
                    take_name = true;
                    continue;
                }
                if line.contains("%VERSION%") {
                    take_version = true;
                    continue;
                }
                if line.contains("%DESC%") {
                    take_description = true;
                    continue;
                }
                if line.contains("%DEPENDS") {
                    take_dependencies = true;
                    continue;
                }

                if take_name {
                    self.name = line.to_owned();
                    take_name = false;
                }

                if take_version {
                    self.version = line.to_owned();
                    take_version = false;
                }

                if take_description {
                    if line != "\n" {
                        self.description.push_str(line)
                    }
                    take_description = false
                }

                if take_dependencies {
                    if !line.trim().is_empty() {
                        // Removing the dependency version from the dependency string if it has one
                        let mut char_index = line.len(); // Initialize the character index to slice at, the char being ">" indicating version

                        if let Some(pos) = line.as_bytes().iter().position(|x| *x == b'>') {
                            char_index = pos;
                        } else if let Some(pos) = line.as_bytes().iter().position(|x| *x == b'=') {
                            char_index = pos;
                        }

                        self.dependencies.push(line[0..char_index].to_owned())
                    } else {
                        take_dependencies = false;
                    }
                }
            }
            Ok(())
        }
    }
}

// Module associated with reading the local package databases
pub mod package_database_reader {
    use super::arch_package::Package;
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    // Method to read the packages, returning a Vector of packages with their fields populated
    pub fn packages_reader(databases_path: &str) -> Result<Vec<Package>, Box<dyn Error>> {
        let paths = fs::read_dir(databases_path)?;

        let mut folders: Vec<String> = Vec::new();

        for path in paths {
            folders.push(format!(
                "{}/{}",
                path?.path().to_str().unwrap().to_owned(),
                "desc"
            ))
        }

        let mut packages: Vec<Package> = Vec::new();

        for file in folders {
            if Path::new(&file).exists() {
                let mut package = Package::new();
                package.init(file)?;

                packages.push(package);
            }
        }

        Ok(packages)
    }
}

//Module associated with functions that get called when passing different command line options
pub mod option_functions {
    use super::arch_package::Package;
    use std::collections::HashSet;

    // A function to get packages that share the same dependencies with the package name passed
    pub fn get_packages_with_same_dependencies<'a>(
        package_name: &str,
        packages: &'a Vec<Package>,
    ) -> Option<HashSet<&'a str>> {
        let mut package_dependencies: &Vec<String> = &Vec::new();
        let mut packages_with_same_dependencies: HashSet<&str> = HashSet::new();
        let mut other_packages_found = false;

        // Getting the package dependencies
        for package in packages {
            if package.name == package_name {
                package_dependencies = &package.dependencies;
                break;
            }
        }

        // Getting the packages with similar dependencies
        for package in packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(&dep) {
                    packages_with_same_dependencies.insert(&package.name);
                    other_packages_found = true;
                }
            }
        }

        if other_packages_found {
            Some(packages_with_same_dependencies)
        } else {
            None
        }
    }

    // A function to get unique dependencies of a package
    pub fn get_unique_dependencies(
        package_name: &str,
        packages: &Vec<Package>,
    ) -> Option<Vec<String>> {
        let mut package_dependencies: &Vec<String> = &Vec::new();
        let mut unique_dependencies_found = false;

        // Getting the package dependencies
        for package in packages {
            if package.name == package_name {
                package_dependencies = &package.dependencies;
                break;
            }
        }

        let mut packages_dependencies_copy = package_dependencies.clone();

        // Removing shared dependencies from the cloned dependencies vector
        for package in packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(&dep) {
                    if let Some(pos) = packages_dependencies_copy.iter().position(|x| x == dep) {
                        packages_dependencies_copy.remove(pos);
                    }
                    unique_dependencies_found = true;
                }
            }
        }

        if unique_dependencies_found && packages_dependencies_copy.len() > 0 {
            Some(packages_dependencies_copy)
        } else {
            None
        }
    }
}

// A module for handling commandline options
pub mod commandline {
    use super::arch_package::Package;
    use super::option_functions::*;

    use std::process;

    pub fn run(args: Vec<String>, packages: &Vec<Package>) -> Result<(), &'static str> {
        let commandline_options = ["--get-unique-deps", "--get-pacs-with-same-deps"];

        if args.len() != 3 {
            return Err("Error: Invalid number of arguments, 2 arguments required");
        }

        if !commandline_options.contains(&args[1].as_ref()) {
            eprintln!("Error, Invalid option {}", args[1]);
            process::exit(1);
        }

        let option = &args[1];
        let package = &args[2];

        if option == "--get-unique-deps" {
            match get_unique_dependencies(package, packages) {
                Some(vec) => {
                    for item in vec {
                        println!("{}", item)
                    }
                }
                None => return Err("No unique dependencies found"),
            }
        }

        if option == "--get-pacs-with-same-deps" {
            match get_packages_with_same_dependencies(package, packages) {
                Some(vec) => {
                    for item in vec {
                        println!("{}", item);
                    }
                }
                None => return Err("No packages with same dependencies found"),
            }
        }

        Ok(())
    }
}
