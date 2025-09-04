use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
    temperature: f32,
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<OpenRouterChoice>,
    usage: Option<OpenRouterUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponseMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub struct OpenRouterClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenRouterClient {
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
    
    pub async fn send_message(&self, system_prompt: &str, user_message: &str) -> Result<String> {
        let request = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![
                OpenRouterMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenRouterMessage {
                    role: "user".to_string(),
                    content: user_message.to_string(),
                },
            ],
            temperature: 0.7,
            max_tokens: Some(4000),
            stream: false,
        };
        
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/ai-cli-agent")
            .header("X-Title", "AI CLI Agent")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("API request failed: {} - {}", status, error_text));
        }
        
        let openrouter_response: OpenRouterResponse = response.json().await?;
        
        if let Some(choice) = openrouter_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!("No response choices received"))
        }
    }
    
    pub async fn send_message_with_context(
        &self,
        system_prompt: &str,
        user_message: &str,
        context: &str,
    ) -> Result<String> {
        let full_message = format!("{}\n\nContext:\n{}", user_message, context);
        self.send_message(system_prompt, &full_message).await
    }
    
    pub async fn generate_code(&self, prompt: &str, language: &str) -> Result<String> {
        let system_prompt = format!(
            "You are an expert {} programmer. Generate clean, efficient, and well-commented code. \
            Provide only the code without any explanations unless specifically requested.",
            language
        );
        
        let enhanced_prompt = format!(
            "Generate {} code for: {}\n\nPlease format your response as:\n\
            WRITE_FILE: filename.{}\n\n[code here]",
            language, prompt, language.to_lowercase()
        );
        
        self.send_message(&system_prompt, &enhanced_prompt).await
    }
    
    pub async fn edit_code(&self, file_content: &str, edit_instructions: &str) -> Result<String> {
        let system_prompt = "You are an expert programmer. Edit the provided code according to the instructions. \
            Maintain the existing structure and style unless the instructions specify otherwise. \
            Provide the complete edited code.";
        
        let prompt = format!(
            "Edit the following code:\n\n{}\n\nInstructions: {}\n\nPlease provide the complete edited code.",
            file_content, edit_instructions
        );
        
        self.send_message(system_prompt, &prompt).await
    }
}