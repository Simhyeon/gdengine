use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::utils;
use reqwest::blocking::{Client, multipart};

pub const IMAGE_LIST : &str = "image_list.txt";

/// MediaWiki's target is not a file but server loaded page
pub(crate) fn render(test: bool) -> Result<Option<PathBuf>, GdeError> {
    check_prerequisites()?;
    let source_file = utils::middle_file_path()?;

    let request = MediaWikiRequest::new(&source_file)?;

    if test { request.login()?; } 
    else { request.exec()?; }

    Ok(Some(source_file))
}

fn check_prerequisites() -> Result<(), GdeError> {
    // Check if necessary config values are present.
    if let Err(_) = std::env::var("MW_URL") { return Err(GdeError::ConfigError(String::from("No \"MW_URL\" in env file"))); }
    if let Err(_) = std::env::var("MW_ID") { return Err(GdeError::ConfigError(String::from("No \"MW_ID\" in env file"))); }
    if let Err(_) = std::env::var("MW_PWD") { return Err(GdeError::ConfigError(String::from("No \"MW_PWD\" in env file"))); }
    if let Err(_) = std::env::var("MW_TITLE") { return Err(GdeError::ConfigError(String::from("No \"MW_TITLE\" in env file"))); }

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
    pub fn new(target_file: &'a Path) -> Result<Self, GdeError> {
        let client = Client::builder().cookie_store(true).build()?;

        Ok(Self {
            client,
            url        : std::env::var("MW_URL").unwrap(),
            bot_id     : std::env::var("MW_ID").unwrap(),
            bot_pwd    : std::env::var("MW_PWD").unwrap(),
            page_title : std::env::var("MW_TITLE").unwrap(),
            target_file,
        })
    }

    fn exec(&self) -> Result<(), GdeError> {
        let login_token = self.get_login_token()?;
        self.post_login(&login_token)?;
        let csrf_token = self.get_csrf_token()?;
        self.edit_page(&csrf_token)?;
        // It is ok that evn var is not set
        if let Ok(value) = std::env::var("MW_UPLOAD") {
            // If true then upload images
            if let Ok(true) = value.to_lowercase().parse::<bool>() {
                self.upload_images(&csrf_token)?;
            }
        }
        Ok(())
    }

    fn login(&self) -> Result<(), GdeError> {
        self.get_login_token()?;
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
        } else {
            eprintln!("Failed to push page");
            eprintln!(r#"Error
{}"#, json);
        }

        Ok(())
    }

    fn upload_images(&self, csrf_token: &str) -> Result<(), GdeError> {

        let images = String::from_utf8(std::fs::read(Path::new(IMAGE_LIST))?).expect("");

        for line in images.lines().into_iter() {
            let path_buf = PathBuf::from(line);
            let form_params = [
                ("action".to_owned(), "upload".to_owned()),
                ("ignorewarnings".to_owned(), "1".to_owned()),
                ("filename".to_owned(),path_buf.file_name().unwrap().to_str().unwrap().to_string()),
                ("format".to_owned(), "json".to_owned())
            ];
            let file_part = multipart::Form::new()
                .text("token", csrf_token.to_owned())
                .file("file", &path_buf)?;

            let request = self.client.post(&self.url)
                .header(reqwest::header::CONTENT_DISPOSITION, line.to_owned())
                .query(&form_params)
                .multipart(file_part)
                .build()?;

            // For DEBUG purpose
            // _print_ln!("REQ : {}", request.url());

            let response = self.client.execute(request)?.text()?;

            let json: serde_json::Value = serde_json::from_str(&response)?;
            if let serde_json::Value::String(content) = &json["upload"]["result"] {
                println!("Edit page : {}", content)
            } else {
                eprintln!("Failed to upload images");
            eprintln!(r#"Error
{}"#, json);
            }
        }

        Ok(())
    }
}
