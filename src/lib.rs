pub mod arch_package {

    use std::fs;

    #[derive(PartialEq, Eq, Clone)]
    pub struct Package {
        pub name: String,
        pub version: String,
        pub description: String,
        pub dependencies: Vec<String>,
        size: String,
    }

    impl Package {
        pub fn new() -> Package {
            Package {
                name: String::new(),
                version: String::new(),
                description: String::new(),
                dependencies: Vec::new(),
                size: String::new(), 
            }
        }

        pub fn init(&mut self, path: String) -> Result<(), &str> {

            let database_contents = match fs::read_to_string(path) {
                Ok(contents) => contents,
                Err(_) => return Err("Failed to read the package database"),
            };

            let mut take_name = false;
            let mut take_version = false;
            let mut take_discription = false;
            let mut take_dependancies = false;
            let mut take_size = false;

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
                    take_discription = true;
                    continue;
                }
                if line.contains("%DEPENDS") {
                    take_dependancies = true;
                    continue;
                }
                if line.contains("%SIZE%") {
                    take_size = true;
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

                if take_discription {
                    if line != "\n" {
                        self.description.push_str(line)
                    }
                    take_discription = false
                }

                if take_size {
                    self.size = line.to_owned();
                    take_size = false;
                }

                if take_dependancies {
                    if !line.trim().is_empty() {

                        // Removing the dependency version from the dependency string if it has one
                        let mut char_index = line.len();    // Initialize the character index to slice at, the char being ">" indicating version

                        if let Some(pos) = line.as_bytes().iter().position(|x| *x == b'>') {
                            char_index = pos;
                        }

                        self.dependencies.push(line[0..char_index].to_owned())
                    } else {
                        take_dependancies = false;
                    }
                }
            }
            Ok(())
        }
    }

    impl Package {
        pub fn has_dependencies(&self) -> bool {
            !self.dependencies.is_empty()
        }

        pub fn dependency_count(&self) -> usize {
            self.dependencies.len()
        }

        pub fn get_size(&self) -> Result<f32, &'static str> {
            let size_in_mb: f32 = match self.size.parse() {
                Ok(size) => size,
                Err(_) => return Err("Failed to obtain the package size"),
            };

            Ok(size_in_mb / 1_000_000 as f32)
        }

        pub fn has_dependency(&self, dependency: &str) -> bool {
            self.dependencies.contains(&dependency.to_owned())
        }
    }
}

pub mod package_database_reader {
    use super::arch_package::Package;
    use std::error::Error;
    use std::fs;
    use std::path::Path;

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

pub mod option_functions {
    use super::arch_package::Package;
    use std::collections::HashSet;

    pub fn get_packages_with_same_dependencies<'a>(package_name: &str, packages: &'a Vec<Package>) -> Option<HashSet<&'a str>> {
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
        
        // Returning the appropriate data
        if other_packages_found {
            Some(packages_with_same_dependencies)
        } else {
            None
        }
    }

    
    pub fn get_unique_dependencies<'a>(package_name: &str, packages: &'a Vec<Package>) -> Option<Vec<String>> {
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

        // Removing shared dependencies
        for package in packages {
            if package.name == package_name {
                continue;
            }

            for dep in &package.dependencies {
                if package_dependencies.contains(&dep) {
                    if let Some(pos) = packages_dependencies_copy.iter().position( |x| x == dep ) {
                        packages_dependencies_copy.remove(pos);
                    }
                    unique_dependencies_found = true;
                }
            }
        }

        if unique_dependencies_found && packages_dependencies_copy.len() > 0{
            Some(packages_dependencies_copy)
        } else {
            None
        }
    }
}
