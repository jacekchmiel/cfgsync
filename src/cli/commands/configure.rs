use clap::Args;

use crate::config::Config;

use super::CfgSyncCommand;

#[derive(Args, Debug)]
pub struct Configure {
    /// Cfgsync configuration key
    key: String,
    /// Cfgsync configuration value
    value: String,
}

impl CfgSyncCommand for Configure {
    fn run(&self, config: &crate::config::Config) -> crate::error::Result<()> {
        let key = serde_yaml::Value::String(self.key.clone());
        let value = serde_yaml::Value::String(self.value.clone());

        let mut config_map = serde_yaml::to_value(config)?
            .as_mapping()
            .expect("Config didn't serialize to Mapping")
            .to_owned();
        if !config_map.contains_key(&key) {
            bail!(
                "Invalid configuration key. Available keys: {}",
                available_keys().join(", ")
            )
        }
        config_map.insert(key, value);

        let new_config: Config = serde_yaml::from_value(serde_yaml::Value::Mapping(config_map))?;
        new_config.store()
    }
}

fn available_keys() -> Vec<String> {
    serde_yaml::to_value(Config::default())
        .expect("Cannot serialize default config")
        .as_mapping()
        .expect("Config didn't serialize to Mapping")
        .into_iter()
        .map(|i| i.0)
        .filter_map(|v| v.as_str().map(Into::into))
        .collect()
}
