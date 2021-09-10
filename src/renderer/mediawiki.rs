use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::config::Config;
use crate::utils;
use reqwest::blocking::Client;

/// MediaWiki's target is not a file but server loaded page
pub(crate) fn render(config: &Config) -> Result<Option<PathBuf>, GdeError> {
    check_prerequisites(config)?;
    let source_file = utils::middle_file_path()?;

    MediaWikiRequest::new(config.get_env_string("mediawiki_url").unwrap(),config, &source_file)?.exec()?;

    Ok(Some(source_file))
}

fn check_prerequisites(config: &Config) -> Result<(), GdeError> {
    // Check if necessary config values are present.
    if let None = config.get_env_string("mediawiki_url") { return Err(GdeError::ConfigError(String::from("No mediawiki_url in config file"))); }
    if let None = config.get_env_string("bot_id") { return Err(GdeError::ConfigError(String::from("No bot_id in config file"))); }
    if let None = config.get_env_string("bot_pwd") { return Err(GdeError::ConfigError(String::from("No bot_pwd in config file"))); }
    if let None = config.get_env_string("page_title") { return Err(GdeError::ConfigError(String::from("No page_title in config file"))); }

    Ok(())
}

struct MediaWikiRequest<'a> {
    client : Client,
    url : String,
    bot_id : String,
    bot_pwd : String,
    page_title : String,
    target_file : &'a Path,
}
impl<'a> MediaWikiRequest<'a> {
    pub fn new(url : String, config: &Config, target_file: &'a Path) -> Result<Self, GdeError> {
        let client = Client::builder().cookie_store(true).build()?;

        Ok(Self {
            client,
            url,
            bot_id : config.get_env_string("bot_id").unwrap(),
            bot_pwd : config.get_env_string("bot_pwd").unwrap(),
            page_title : config.get_env_string("page_title").unwrap(),
            target_file,
        })
    }

    pub fn exec(&self) -> Result<(), GdeError> {
        let login_token = self.get_login_token()?;
        self.post_login(&login_token)?;
        let csrf_token = self.get_csrf_token()?;
        self.edit_page(&csrf_token)?;
        Ok(())
    }

    fn get_login_token(&self) -> Result<String, GdeError> {
        let params = [("action", "query"), ("meta", "tokens"), ("type", "login"), ("format", "json") ];
        let response = self.client.post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["query"]["tokens"]["logintoken"] {
            Ok(content.to_owned())
        } else {
            Err(GdeError::RendererError("Failed to get login token from mediawiki"))
        }
    }

    fn post_login(&self, login_token : &str) -> Result<(), GdeError> {
        let params = [ ("action", "login"), ("lgname", self.bot_id.as_str()), ("lgpassword", self.bot_pwd.as_str()), ("lgtoken", login_token), ("format", "json") ];
        self.client.post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        // println!("Post response : {}", response);

        Ok(())
    }

    fn get_csrf_token(&self) -> Result<String, GdeError> {
        let params = [
            ("action","query"),
            ("meta","tokens"),
            ("format", "json") 
        ]; 
        let response = self.client.post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["query"]["tokens"]["csrftoken"] {
            Ok(content.to_owned())
        } else {
            Err(GdeError::RendererError("Failed to get login token from mediawiki"))
        }
    }

    fn edit_page(&self, csrf_token: &str) -> Result<(), GdeError> {
        let content = std::fs::read_to_string(&self.target_file)?;
        let params = [ 
            ("action", "edit"),
            ("title", &self.page_title),
            ("text", &content),
            ("token", csrf_token),
            ("format", "json")
        ];

        let response = self.client.post(&self.url)
            .form(&params)
            .send()?
            .text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["edit"]["result"] {
            println!("Edit page : {}", content)
        }

        Ok(())
    }
}

