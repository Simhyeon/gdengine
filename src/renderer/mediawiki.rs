use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::utils;
use rad::{Processor,AuthType};
use reqwest::blocking::{Client, multipart};

pub const IMAGE_LIST : &str = "image_list.txt";

pub(crate) fn rad_setup(processor : &mut Processor) {
    processor.from_string(
        r#"$ifenv(MW_UPLOAD,$fileout(true,image_list.txt,))"#
    ).expect("Failed to setup mediawiki");
}

/// MediaWiki's target is not a file but server loaded page
pub(crate) fn render() -> Result<Option<PathBuf>, GdeError> {
    check_prerequisites()?;
    let source_file = utils::middle_file_path()?;
    preprocess(&source_file)?;

    let request = MediaWikiRequest::new(&source_file)?;
    request.exec()?; 

    Ok(Some(source_file))
}

pub(crate) fn render_preview() -> Result<Option<PathBuf>, GdeError> {
    let source_file = utils::middle_file_path()?;
    preprocess(&source_file)?;

    // Create preview html file
    Processor::new()
        .greedy(true)
        .write_to_file(Some(utils::BUILD_PATH.join("out.html")))?
        .unix_new_line(true)
        .allow(Some(vec!(AuthType::FIN, AuthType::ENV)))
        .from_file(&utils::renderer_path("mediawiki")?.join("preview.html"))?;

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

fn preprocess(path: &Path) -> Result<(), GdeError> {
    utils::chomp_file(path)?;
    Ok(())
}

pub(crate) fn clear_files(test: bool, preserve: bool) -> Result<(), GdeError> {
    let image_list = std::env::current_dir()?.join(IMAGE_LIST);
    // Test reseve image list file
    if image_list.exists() {
        if test || preserve {
            std::fs::rename(
                image_list,
                &*utils::CACHE_PATH.join(IMAGE_LIST)
            )?;
        } else {
            std::fs::remove_file(image_list)?;
        }
    }
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
        if let Ok(_) = std::env::var("MW_UPLOAD") {
            self.upload_images(&csrf_token)?;
        }
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

    // Should receive Jsonvalue[error][code]
    fn upload_error_fallback(&self, error: &serde_json::Value, target: &Path) -> Result<(), GdeError> {
        match error.as_str().unwrap() {
            "fileexists-no-change" => {
                println!("No upload for duplicate files {}", target.display())
            }
            _ => eprintln!("Failed to upload image"),
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

            let response = self.client.execute(request)?.text()?;

            let json: serde_json::Value = serde_json::from_str(&response)?;
            if let serde_json::Value::String(content) = &json["upload"]["result"] {
                println!("Edit page : {}", content)
            } else {
                self.upload_error_fallback(&json["error"]["code"], &path_buf)?;
            }
        }

        Ok(())
    }
}
