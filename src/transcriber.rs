use reqwest::Client;
use reqwest::multipart::Form;
use std::path::Path;

pub struct Transcriber;

impl Transcriber {

    const OPENAI_URL: &'static str = "https://api.openai.com/v1/audio/transcriptions";
    const OPENAI_MODEL: &'static str = "whisper-1";
    const OPENAI_AUTH_TOKEN: &'static str = "Bearer sk-***";

    pub fn new() -> Self {
        Self {}
    }

    /// Submits an audio file to OpenAI for transcribing. Returns result as `String`.
    pub async fn transcribe_file(&self, file_path: &str) -> Result<String, String> {
        if !Path::new(file_path).exists() {
            return Err("File not found!".to_string());
        }

        let form = self.build_form(file_path).await;
        let response = Client::new()
            .post(Self::OPENAI_URL)
            .header("Authorization", Self::OPENAI_AUTH_TOKEN)
            .multipart(form)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or("Failed to read response text".to_string());
                    Ok(text)
                } else {
                    Err(format!("Request failed with status: {}", resp.status()))
                }
            }
            Err(err) => Err(format!("Request error: {}", err)),
        }
    }

    async fn build_form(&self, path: &str) -> Form {
        Form::new()
            .text("model", Self::OPENAI_MODEL)
            .file("file", path)
            .await.unwrap()
    }
}