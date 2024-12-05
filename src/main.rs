mod day1;
mod day2;
mod day3;
fn main() -> Result<(), anyhow::Error> {
    let config = Conf::get_conf()?;
    day3::run(config.path_to_input)?;
    Ok(())
}


// config module
use config::Config;
use std::{env::VarError, path::PathBuf};
struct Conf{
    path_to_input : PathBuf,
}

impl Conf{
    fn get_conf() -> Result<Self, config::ConfigError>{
        let config_path = get_env_path().map_err(|_| config::ConfigError::NotFound("env file not found".to_owned()))?;
        let config = Config::builder()
            .add_source(config::File::from(config_path).required(true))
            .build()?;
        let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").map_err(|_|{
            config::ConfigError::NotFound("manifest not found".to_owned())
        })?);
        path.push("puzzle_input");
        path.push(config.get::<String>("source_file")?);
        Ok(Self{
            path_to_input : path
        })
    }
}

fn get_env_path() -> Result<PathBuf, VarError>{
    let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("env");
    path.push("env.toml");
    Ok(path)
}