pub mod arch_package {
    // Section names in arch package description
    const NAME_SECTION: &str = "%NAME%";
    const DEPENDENCIES_SECTION: &str = "%DEPENDS%";
    const OPTIONAL_DEPENDENCIES_SECTION: &str = "%OPTDEPENDS%";

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

                if current_line == NAME_SECTION {
                    if let Some(name) = lines_iter.next() {
                        package.name.push_str(name);
                        continue;
                    }
                }

                if current_line == DEPENDENCIES_SECTION {
                    for dependency in lines_iter.by_ref() {
                        let dependency = dependency.trim();

                        if dependency.is_empty() {
                            break;
                        } else {
                            package.dependencies.push(dependency.to_string());
                        }
                    }
                    continue;
                }

                if current_line == OPTIONAL_DEPENDENCIES_SECTION {
                    for dependency in lines_iter.by_ref() {
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
                    continue;
                }
            }

            package
        }
    }
}

// Module associated with reading the local package databases
pub mod package_database_reader {
    use super::arch_package::Package;
    use std::fs;
    use std::path::Path;

    use anyhow::Result;

    // Method to read the packages, returning a Vector of packages with their fields populated
    pub fn packages_reader(databases_path: &str) -> Result<Vec<Package>> {
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

    use anyhow::{Context, Result};

    const DEFAULT_DATABASE_PATH: &str = "/var/lib/pacman/local/";

    // A function to populate all the packages installed in the system
    pub fn populate_packages() -> Result<Vec<Package>> {
        let packages =
            packages_reader(DEFAULT_DATABASE_PATH).context("Failed to read packages database")?;
        Ok(packages)
    }

    // A function to get packages that share the same dependencies with the package name passed
    pub fn get_packages_with_same_dependencies<'a>(
        package_name: &str,
        packages: &'a Vec<Package>,
    ) -> Option<HashSet<&'a str>> {
        let mut package_dependencies: &Vec<String> = &Vec::new();
        let mut packages_with_same_dependencies: HashSet<&str> = HashSet::new();

        // Getting the package dependencies
        if let Some(index) = packages
            .iter()
            .position(|package| package.name == package_name)
        {
            package_dependencies = &packages.get(index).unwrap().dependencies; // unwrap will never panic as it is guaranteed by the if statement
        }

        // Not bothering if the package has no dependencies
        if package_dependencies.is_empty() {
            return None;
        };

        // Getting the packages with similar dependencies
        for package in packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(dep) {
                    packages_with_same_dependencies.insert(&package.name);
                }
            }
        }

        if !packages_with_same_dependencies.is_empty() {
            return Some(packages_with_same_dependencies);
        }

        None
    }

    // A function to get unique dependencies of a package
    pub fn get_unique_dependencies<'a>(
        package_name: &str,
        packages: &'a Vec<Package>,
    ) -> Option<Vec<&'a String>> {
        let package_dependencies: &Vec<String>;
        let mut unique_dependencies_found = false;

        // Getting the package dependencies
        if let Some(index) = packages
            .iter()
            .position(|package| package.name == package_name)
        {
            package_dependencies = &packages.get(index).unwrap().dependencies; // unwrap will never panic as it is guaranteed by the if statement
        } else {
            return None;
        };

        let mut packages_dependencies_copy: Vec<&String> = package_dependencies.iter().collect();

        // Removing shared dependencies from the cloned dependencies vector
        for package in packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(dep) {
                    if let Some(pos) = packages_dependencies_copy.iter().position(|x| *x == dep) {
                        packages_dependencies_copy.swap_remove(pos);
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
                    String::from("spoon"),
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
        let packages = get_packages();

        let sample = ["bottle".to_owned(), "spoon".to_owned()];
        let result = get_unique_dependencies("a", &packages).unwrap();

        let sample_set: HashSet<_> = sample.iter().collect();
        let mut result_set: HashSet<&String> = HashSet::new();

        result.iter().for_each(|x| _ = result_set.insert(x));

        let result_length = sample_set
            .symmetric_difference(&result_set)
            .into_iter()
            .collect::<Vec<_>>()
            .len();

        assert_eq!(result_length, 0);
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
