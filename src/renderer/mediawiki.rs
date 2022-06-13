use super::models::GRender;
use crate::error::GdeError;
use crate::models::GdeResult;
use crate::utils;
use crate::{executor::ExecOption, utils::BUILD_PATH};
use r4d::{Processor, RadStorage, StorageOutput, StorageResult, WriteOption};
use reqwest::blocking::{multipart, Client};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub struct MWRenderer;

impl GRender for MWRenderer {
    fn rad_setup(&self, processor: &mut Processor) -> GdeResult<()> {
        processor.set_storage(Box::new(ImageList { images: Vec::new() }));
        Ok(())
    }

    /// MediaWiki's target is not a file but server loaded page
    fn render(&self, processor: &mut Processor, _: &ExecOption) -> GdeResult<Option<PathBuf>> {
        let image_list: ImageList = get_image_list(processor)?;
        let source_file = utils::middle_file_path()?;
        self.compress_file(&source_file)?;
        self.check_prerequisites()?;
        chomp_file(&source_file)?;

        let request = MediaWikiRequest::new(&source_file)?;
        request.exec(image_list)?;

        Ok(Some(source_file))
    }
}

impl MWRenderer {
    fn compress_file(&self, source_file: &Path) -> GdeResult<()> {
        utils::chomp_file(source_file)?;
        Ok(())
    }

    fn check_prerequisites(&self) -> GdeResult<()> {
        // Check if necessary config values are present.
        if std::env::var("MW_URL").is_err() {
            return Err(GdeError::ConfigError(String::from(
                "No \"MW_URL\" in env file",
            )));
        }
        if std::env::var("MW_ID").is_err() {
            return Err(GdeError::ConfigError(String::from(
                "No \"MW_ID\" in env file",
            )));
        }
        if std::env::var("MW_PWD").is_err() {
            return Err(GdeError::ConfigError(String::from(
                "No \"MW_PWD\" in env file",
            )));
        }
        if std::env::var("MW_TITLE").is_err() {
            return Err(GdeError::ConfigError(String::from(
                "No \"MW_TITLE\" in env file",
            )));
        }

        Ok(())
    }
}

pub struct PreviewRenderer;

impl GRender for PreviewRenderer {
    fn rad_setup(&self, processor: &mut Processor) -> GdeResult<()> {
        processor.set_storage(Box::new(ImageList { images: Vec::new() }));
        Ok(())
    }

    fn render(&self, processor: &mut Processor, _: &ExecOption) -> GdeResult<Option<PathBuf>> {
        let image_list: ImageList = get_image_list(processor)?;
        let source_file = utils::middle_file_path()?;
        chomp_file(&source_file)?;

        let new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(utils::BUILD_PATH.join("out.html"))?;

        processor.set_write_option(WriteOption::File(new_file));
        processor.process_file(&utils::renderer_path("mediawiki")?.join("preview.html"))?;

        // Copy images only after processing was succesful
        self.copy_images(&image_list)?;

        Ok(Some(source_file))
    }
}

impl PreviewRenderer {
    fn copy_images(&self, image_list: &ImageList) -> GdeResult<()> {
        for img in &image_list.images {
            std::fs::copy(img, &*BUILD_PATH.join(img))?;
        }
        Ok(())
    }
}

struct MediaWikiRequest<'a> {
    client: Client,
    url: String,
    bot_id: String,
    bot_pwd: String,
    page_title: String,
    target_file: &'a Path,
}

impl<'a> MediaWikiRequest<'a> {
    pub fn new(target_file: &'a Path) -> GdeResult<Self> {
        let client = Client::builder().cookie_store(true).build()?;

        Ok(Self {
            client,
            url: std::env::var("MW_URL").unwrap(),
            bot_id: std::env::var("MW_ID").unwrap(),
            bot_pwd: std::env::var("MW_PWD").unwrap(),
            page_title: std::env::var("MW_TITLE").unwrap(),
            target_file,
        })
    }

    fn exec(&self, image_list: ImageList) -> GdeResult<()> {
        let login_token = self.get_login_token()?;
        self.post_login(&login_token)?;
        let csrf_token = self.get_csrf_token()?;
        // This should come first
        if std::env::var("MW_UPLOAD").is_ok() {
            self.upload_images(&csrf_token, image_list)?;
        }
        self.edit_page(&csrf_token)?;
        // It is ok that evn var is not set
        Ok(())
    }

    fn get_login_token(&self) -> GdeResult<String> {
        let params = [
            ("action", "query"),
            ("meta", "tokens"),
            ("type", "login"),
            ("format", "json"),
        ];
        let response = self.client.post(&self.url).form(&params).send()?.text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["query"]["tokens"]["logintoken"] {
            Ok(content.to_owned())
        } else {
            Err(GdeError::RendererError(
                "Failed to get login token from mediawiki",
            ))
        }
    }

    fn post_login(&self, login_token: &str) -> GdeResult<()> {
        let params = [
            ("action", "login"),
            ("lgname", self.bot_id.as_str()),
            ("lgpassword", self.bot_pwd.as_str()),
            ("lgtoken", login_token),
            ("format", "json"),
        ];
        self.client.post(&self.url).form(&params).send()?.text()?;

        // println!("Post response : {}", response);

        Ok(())
    }

    fn get_csrf_token(&self) -> GdeResult<String> {
        let params = [("action", "query"), ("meta", "tokens"), ("format", "json")];
        let response = self.client.post(&self.url).form(&params).send()?.text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["query"]["tokens"]["csrftoken"] {
            Ok(content.to_owned())
        } else {
            Err(GdeError::RendererError(
                "Failed to get login token from mediawiki",
            ))
        }
    }

    fn edit_page(&self, csrf_token: &str) -> GdeResult<()> {
        let content = std::fs::read_to_string(&self.target_file)?;
        let params = [
            ("action", "edit"),
            ("title", &self.page_title),
            ("text", &content),
            ("token", csrf_token),
            ("format", "json"),
        ];

        let response = self.client.post(&self.url).form(&params).send()?.text()?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        if let serde_json::Value::String(content) = &json["edit"]["result"] {
            println!("Edit page : {}", content)
        } else {
            eprintln!("Failed to push page");
            eprintln!(
                r#"Error
{}"#,
                json
            );
        }

        Ok(())
    }

    // Should receive Jsonvalue[error][code]
    fn upload_error_fallback(&self, error: &serde_json::Value, target: &Path) -> GdeResult<()> {
        match error.as_str().unwrap() {
            "fileexists-no-change" => {
                println!("No upload for duplicate files {}", target.display())
            }
            _ => eprintln!("Failed to upload image"),
        }
        Ok(())
    }

    fn upload_images(&self, csrf_token: &str, image_list: ImageList) -> GdeResult<()> {
        for line in image_list.images.iter() {
            let path_buf = PathBuf::from(line);

            if !path_buf.exists() {
                eprintln!(
                    "Given image file  \"{}\" doesn't exit and cannot be sent.",
                    path_buf.display()
                );
                return Err(GdeError::RendererError(
                    "Failed to send images to mediawiki server",
                ));
            }

            let form_params = [
                ("action".to_owned(), "upload".to_owned()),
                ("ignorewarnings".to_owned(), "1".to_owned()),
                (
                    "filename".to_owned(),
                    path_buf.file_name().unwrap().to_str().unwrap().to_string(),
                ),
                ("format".to_owned(), "json".to_owned()),
            ];
            let file_part = multipart::Form::new()
                .text("token", csrf_token.to_owned())
                .file("file", &path_buf)?;

            let request = self
                .client
                .post(&self.url)
                .header(reqwest::header::CONTENT_DISPOSITION, line.to_owned())
                .query(&form_params)
                .multipart(file_part)
                .build()?;

            let response = self.client.execute(request)?.text()?;

            let json: serde_json::Value = serde_json::from_str(&response)?;
            if let serde_json::Value::String(content) = &json["upload"]["result"] {
                println!("Upload image \"{}\" : {}", path_buf.display(), content)
            } else {
                self.upload_error_fallback(&json["error"]["code"], &path_buf)?;
            }
        }

        Ok(())
    }
} // End impl

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImageList {
    pub images: Vec<String>,
}

impl RadStorage for ImageList {
    fn update(&mut self, args: &[String]) -> StorageResult<()> {
        self.images.push(args[0].to_owned());
        Ok(())
    }

    fn extract(&mut self, _: bool) -> StorageResult<Option<StorageOutput>> {
        Ok(Some(StorageOutput::Text(self.images.join("\n"))))
    }
}

impl ImageList {
    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>) -> GdeResult<Self> {
        Ok(bincode::deserialize::<Self>(&bytes).expect("F"))
    }

    pub fn from_text(text: String) -> Self {
        let images = text.lines().map(|s| s.to_owned()).collect::<Vec<String>>();

        Self { images }
    }
}

// Utils method
fn chomp_file(path: impl AsRef<Path>) -> GdeResult<()> {
    utils::chomp_file(path.as_ref())?;
    Ok(())
}
fn get_image_list(processor: &mut Processor) -> GdeResult<ImageList> {
    let output = processor.extract_storage(false).unwrap().unwrap();
    if let StorageOutput::Text(texts) = output {
        Ok(ImageList::from_text(texts))
    } else {
        Err(GdeError::RendererError(
            "Image list cannot be constructed from StorageOutput::Text",
        ))
    }
}
