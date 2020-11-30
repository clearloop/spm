//! Command Config
use crate::{
    registry::Registry,
    result::{Error, Result},
};
use etc::{Etc, FileSystem, Write};
use std::{path::PathBuf, process::Command};
use structopt::StructOpt;

/// Command Config
#[derive(StructOpt, Debug)]
pub enum Config {
    /// Sets config field
    Set {
        /// Substrate registry
        #[structopt(short)]
        registry: String,
    },
    /// Lists the current config
    List,
    /// Edits the current config
    Edit,
}

/// Exec `config` command
pub fn exec(r: Registry, config: Config) -> Result<()> {
    let cur_registry = PathBuf::from(&r.dir);
    let home = cur_registry
        .parent()
        .expect("Could not find home dir of sup")
        .parent()
        .expect("Could not find home dir of sup");

    match config {
        Config::List => {
            println!("{:#?}", &r.config);
        }
        Config::Edit => {
            Command::new("vi")
                .arg(
                    home.parent()
                        .expect("Could not find home dir of sup")
                        .join("config.toml")
                        .to_string_lossy()
                        .to_string(),
                )
                .status()?;
            return Ok(());
        }
        Config::Set { registry } => {
            if !registry.ends_with(".git") {
                return Err(Error::Sup(format!("Wrong git url: {}", registry)));
            }
            let mut config = r.config.clone();
            config.node.registry = registry;

            println!("{:?}", &config);
            Etc::from(&home)
                .open("config.toml")?
                .write(toml::to_string(&config)?)?;

            return Ok(());
        }
    }
    Ok(())
}
