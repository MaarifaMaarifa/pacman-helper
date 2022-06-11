pub mod arch_packages {

    use std::fs;

    #[derive(PartialEq, Eq)]
    pub struct Package {
        pub name: String,
        pub version: String,
        pub description: String,
        pub dependencies: Vec<String>,
        size: String,
        //pub is_initialized: bool,
    }

    impl Package {
        pub fn new() -> Package {
            Package {
                name: String::new(),
                version: String::new(),
                description: String::new(),
                dependencies: Vec::new(),
                size: String::new(), //is_initialized: false,
            }
        }

        pub fn init(&mut self, path: String) {
            //self.is_initialized = true;

            let database_contents = match fs::read_to_string(path) {
                Ok(contents) => contents,
                Err(_) => {
                    panic!("Failed to get the database path");
                }
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
                        self.dependencies.push(line.to_owned())
                    } else {
                        take_dependancies = false;
                    }
                }
            }
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

        pub fn has_dependancy(&self, dependency: &str) -> bool {
            self.dependencies.contains(&dependency.to_owned())
        }
    }

    pub struct Dependency<'a> {
        name: String,
        packages_depending: Vec<&'a Package>,
    }

    impl<'a> Dependency<'a> {
        pub fn new(name: String) -> Dependency<'a> {
            Dependency {
                name,
                packages_depending: Vec::new(),
            }
        }

        pub fn can_be_removed_with(&self, package: &Package) -> bool {
            if self.packages_depending.contains(&package) && package.dependency_count() == 1 {
                return true;
            }
            return false;
        }

        pub fn add_package(&mut self, package: &'a Package) {
            self.packages_depending.push(package);
        }

        pub fn packages_depending_count(&self) -> usize {
            self.packages_depending.len()
        }
    }
}

pub mod package_database_reader {
    use super::arch_packages::Package;
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
                package.init(file);

                packages.push(package);
            }
        }

        Ok(packages)
    }
}
