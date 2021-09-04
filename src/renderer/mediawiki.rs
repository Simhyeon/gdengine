use std::path::PathBuf;
use crate::error::GdeError;
use crate::config::Config;
use crate::utils;

/// MediaWiki's target is not a file but server loaded page
pub(crate) fn render(config: &Config) -> Result<Option<PathBuf>, GdeError> {
    // Check if necessary config values are present.
    if let None = config.get_env("bot_id") { return Err(GdeError::ConfigError(String::from("No bot_id in config file"))); }
    if let None = config.get_env("bot_pwd") { return Err(GdeError::ConfigError(String::from("No bot_pwd in config file"))); }
    if let None = config.get_env("page_title") { return Err(GdeError::ConfigError(String::from("No page_title in config file"))); }

    let source_file = utils::middle_file_path()?;

    utils::command("node", vec![
        &utils::renderer_path("mediawiki")?.join("index.js"),
        &source_file,
        &*utils::CONFIG_PATH
    ])?;

    Ok(Some(source_file))
}
