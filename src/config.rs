use serde::Deserialize;

pub(crate) const DEFAULT_CONFIG_PATH: &str = "config.yaml";

pub(crate) fn read_config(path: &std::path::Path) -> crate::Result<Config> {
    Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) check_interval_sec: u64,
    pub(crate) runners: Vec<Runner>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum Runner {
    Google {
        hostname: String,
        username: String,
        password: String,
    },
}
