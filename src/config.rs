//! Local configuration

/// Local configuration
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {}

/// Parse local configuration
pub(crate) fn parse() -> anyhow::Result<Config> {
    let binary_name = env!("CARGO_PKG_NAME");
    let xdg_dirs = xdg::BaseDirectories::with_prefix(binary_name)?;
    let config_filepath = xdg_dirs
        .find_config_file("config.toml")
        .ok_or_else(|| anyhow::anyhow!("Missing config file"))?;
    let toml_data = std::fs::read_to_string(config_filepath)?;
    let config = toml::from_str(&toml_data)?;
    log::debug!("{config:#?}");
    Ok(config)
}
