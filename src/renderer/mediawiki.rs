use std::process::Command;
use crate::error::GdeError;
use crate::config::Config;
use crate::utils;

pub(crate) fn render(config: &Config) -> Result<(), GdeError> {
    // Check if necessary config values are present.
    if let None = config.get_value("bot_id") { return Err(GdeError::ConfigError(String::from("No bot_id in config file"))); }
    if let None = config.get_value("bot_pwd") { return Err(GdeError::ConfigError(String::from("No bot_pwd in config file"))); }
    if let None = config.get_value("page_title") { return Err(GdeError::ConfigError(String::from("No page_title in config file"))); }

    let source_file = utils::middle_file_path()?;

    // Execute binary to send reques to wiki server
    let output = Command::new("node")
        // Other aguments
        .arg(utils::renderer_path("mediawiki")?.join("index.js"))
        .arg(source_file)
        .arg(&*utils::CONFIG_PATH)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
