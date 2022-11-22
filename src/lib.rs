pub mod arch_package {

    // Custom type for an arch package
    #[derive(Default)]
    pub struct Package {
        pub name: String,
        pub dependencies: Vec<String>,
        pub opt_dependencies: Vec<String>,
    }

    impl Package {
        pub fn new(database_content: String) -> Self {
            let mut package = Package::default();

            let mut lines_iter = database_content.lines();

            while let Some(current_line) = lines_iter.next() {
                let current_line = current_line.trim();

                if current_line == "%NAME%" {
                    if let Some(name) = lines_iter.next() {
                        package.name.push_str(name);
                    }
                }

                if current_line == "%DEPENDS%" {
                    while let Some(dependency) = lines_iter.next() {
                        let dependency = dependency.trim();

                        if dependency.is_empty() {
                            break;
                        } else {
                            package.dependencies.push(dependency.to_string());
                        }
                    }
                }

                if current_line == "%OPTDEPENDS%" {
                    while let Some(dependency) = lines_iter.next() {
                        let dependency = dependency.trim();

                        if dependency.is_empty() {
                            break;
                        } else if let Some(colon_index) = dependency.find(':') {
                            /* Getting rid of optional package description */
                            let dependency = &dependency[..colon_index];
                            package.opt_dependencies.push(dependency.to_string());
                        } else {
                            package.opt_dependencies.push(dependency.to_string());
                        }
                    }
                }
            }

            package
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
                let content = fs::read_to_string(file)?;
                let package = Package::new(content);
                packages.push(package);
            }
        }

        Ok(packages)
    }
}

//Module associated with functions that get called when passing different command line options
pub mod commandline_functions {
    use super::arch_package::Package;
    use crate::package_database_reader::packages_reader;
    use std::collections::HashSet;
    use std::process;

    const DEFAULT_DATABASE_PATH: &str = "/var/lib/pacman/local/";

    // A function to populate all the packages installed in the system
    pub fn populate_packages() -> Vec<Package> {
        let packages = match packages_reader(DEFAULT_DATABASE_PATH) {
            Ok(packages) => packages,
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };
        packages
    }

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
                if package_dependencies.contains(dep) {
                    packages_with_same_dependencies.insert(&package.name);
                    other_packages_found = true;
                }
            }
        }

        if other_packages_found {
            return Some(packages_with_same_dependencies);
        }

        None
    }

    // A function to get unique dependencies of a package
    pub fn get_unique_dependencies(
        package_name: &str,
        packages: Vec<Package>,
    ) -> Option<Vec<String>> {
        let mut package_dependencies: &Vec<String> = &Vec::new();
        let mut unique_dependencies_found = false;

        // Getting the package dependencies
        for package in &packages {
            if package.name == package_name {
                package_dependencies = &package.dependencies;
                break;
            }
        }

        let mut packages_dependencies_copy = package_dependencies.clone();

        // Removing shared dependencies from the cloned dependencies vector
        for package in &packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(dep) {
                    if let Some(pos) = packages_dependencies_copy.iter().position(|x| x == dep) {
                        packages_dependencies_copy.remove(pos);
                    }
                    unique_dependencies_found = true;
                }
            }
        }

        if unique_dependencies_found && !packages_dependencies_copy.is_empty() {
            return Some(packages_dependencies_copy);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::arch_package::Package;
    use super::commandline_functions::{
        get_packages_with_same_dependencies, get_unique_dependencies,
    };

    /// A test for creating a package instance with it's fields
    #[test]
    fn package_creation_test() {
        let package_database = "
%NAME%
linux-kernel

%DEPENDS%
coreutils
kmod
initramfs

%OPTDEPENDS%
wireless-regdb: to set the correct wireless channels of your country
linux-firmware: firmware images needed for some devices
";

        let package = Package::new(package_database.to_string());

        assert_eq!(package.name, "linux-kernel");
        assert_eq!(package.dependencies, vec!["coreutils", "kmod", "initramfs"]);
        assert_eq!(
            package.opt_dependencies,
            vec!["wireless-regdb", "linux-firmware"]
        );
    }

    fn get_packages() -> Vec<Package> {
        vec![
            Package {
                name: String::from("a"),
                dependencies: vec![
                    String::from("bottle"),
                    String::from("cup"),
                    String::from("plate"),
                ],
                opt_dependencies: vec![],
            },
            Package {
                name: String::from("b"),
                dependencies: vec![
                    String::from("bulb"),
                    String::from("wire"),
                    String::from("plate"),
                ],
                opt_dependencies: vec![],
            },
            Package {
                name: String::from("c"),
                dependencies: vec![
                    String::from("pluto"),
                    String::from("mars"),
                    String::from("cup"),
                ],
                opt_dependencies: vec![],
            },
        ]
    }

    /// A test for getting unique dependencies of a package out of others
    #[test]
    fn fetching_unique_dependencies_test() {
        assert_eq!(
            get_unique_dependencies("a", get_packages()),
            Some(vec!["bottle".to_owned()])
        );
    }

    /// A test for getting packages that share the same dependencies with the given package
    #[test]
    fn fetching_packages_with_same_deps_test() {
        let mut packages_with_same_deps = HashSet::new();
        packages_with_same_deps.insert("b");
        packages_with_same_deps.insert("c");

        assert_eq!(
            get_packages_with_same_dependencies("a", &get_packages()),
            Some(packages_with_same_deps)
        )
    }
}
