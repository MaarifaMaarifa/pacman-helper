use clap::{Parser, Subcommand};

/// pacman-helper
#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// get dependencies that are unique to the given package
    GetUniqueDeps(UniqueDeps),

    /// get packages that share the same dependencies with the given package
    GetPacsWithSameDeps(PacsWithSameDeps),
}

#[derive(Debug, Parser)]
pub struct UniqueDeps {
    /// package name
    pub package: String,
}

#[derive(Debug, Parser)]
pub struct PacsWithSameDeps {
    /// package name
    pub package: String,
}
