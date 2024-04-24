use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum GenerateCommands {
    Invoice {
        name: Option<String>,
    },
}

