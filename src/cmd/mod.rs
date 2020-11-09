//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

pub mod config;
pub mod new;
pub mod source;
pub mod tag;
pub mod update;
pub mod upgrade;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Create a new substrate node template
    New {
        /// Package path
        #[structopt(name = "PATH")]
        path: PathBuf,
        /// If skip toolchain check
        #[structopt(short, long)]
        skip: bool,
    },
    /// List available tags
    Tag {
        /// The limit count of tags
        #[structopt(short, long, default_value = "10")]
        limit: usize,
        /// If update registry
        #[structopt(short, long)]
        update: bool,
    },
    /// Update registry
    Update,
    /// List Source
    Source {
        /// Show dependencies contain <query>
        #[structopt(short, long, default_value = "")]
        query: String,
        /// Registry tag
        #[structopt(short, long, default_value = "")]
        tag: String,
        /// If show versions
        #[structopt(short, long)]
        version: bool,
    },
    /// Upgrade substrate project
    Upgrade {
        /// Project path
        #[structopt(short, long, default_value = ".")]
        path: PathBuf,
        /// Registry tag
        #[structopt(short, long, default_value = "")]
        tag: String,
        /// If update registry
        #[structopt(short, long)]
        update: bool,
    },
    /// Show or edit the current config
    Config {
        /// Edit the config
        #[structopt(short, long)]
        edit: bool,
        /// Set registry
        #[structopt(
            short,
            long,
            default_value = "https://github.com/paritytech/substrate.git",
            name = "git-url"
        )]
        registry: String,
    },
}

/// Exec commands
pub fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::New { path, skip } => new::exec(path, skip)?,
        Opt::Config { edit, registry } => config::exec(edit, registry)?,
        Opt::Tag { limit, update } => tag::exec(limit, update)?,
        Opt::Update => update::exec()?,
        Opt::Upgrade { tag, path, update } => upgrade::exec(path, tag, update)?,
        Opt::Source {
            query,
            tag,
            version,
        } => source::exec(query, tag, version)?,
    }

    Ok(())
}
