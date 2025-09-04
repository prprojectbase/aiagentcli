use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub openrouter_api_key: String,
    pub model: String,
    pub work_dir: PathBuf,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub auto_save: bool,
    pub backup_enabled: bool,
    pub backup_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            openrouter_api_key: String::new(),
            model: "openai/gpt-4".to_string(),
            work_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            max_tokens: 4000,
            temperature: 0.7,
            timeout_seconds: 120,
            auto_save: true,
            backup_enabled: true,
            backup_dir: PathBuf::from(".ai_cli_backups"),
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let mut config: Config = toml::from_str(&content)
                .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
            
            // Resolve relative paths
            if !config.work_dir.is_absolute() {
                config.work_dir = std::env::current_dir()?.join(config.work_dir);
            }
            
            if !config.backup_dir.is_absolute() {
                config.backup_dir = std::env::current_dir()?.join(config.backup_dir);
            }
            
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
    
    pub async fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, content).await?;
        println!("✅ Configuration saved to: {}", config_path.display());
        Ok(())
    }
    
    pub async fn create_default_config() -> Result<Self> {
        let config = Self::default();
        config.save().await?;
        Ok(config)
    }
    
    pub async fn update_api_key(&mut self, api_key: String) -> Result<()> {
        self.openrouter_api_key = api_key;
        self.save().await?;
        Ok(())
    }
    
    pub async fn update_model(&mut self, model: String) -> Result<()> {
        self.model = model;
        self.save().await?;
        Ok(())
    }
    
    pub async fn update_work_dir(&mut self, work_dir: PathBuf) -> Result<()> {
        self.work_dir = work_dir;
        self.save().await?;
        Ok(())
    }
    
    pub fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Failed to get config directory"))?;
        
        Ok(config_dir.join("ai-cli-agent").join("config.toml"))
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.openrouter_api_key.is_empty() {
            return Err(anyhow!("OpenRouter API key is required"));
        }
        
        if self.model.is_empty() {
            return Err(anyhow!("Model is required"));
        }
        
        if self.max_tokens == 0 {
            return Err(anyhow!("Max tokens must be greater than 0"));
        }
        
        if self.temperature < 0.0 || self.temperature > 2.0 {
            return Err(anyhow!("Temperature must be between 0.0 and 2.0"));
        }
        
        if self.timeout_seconds == 0 {
            return Err(anyhow!("Timeout must be greater than 0"));
        }
        
        Ok(())
    }
    
    pub fn get_models_list() -> Vec<&'static str> {
        vec![
            "openai/gpt-4",
            "openai/gpt-4-turbo",
            "openai/gpt-3.5-turbo",
            "anthropic/claude-2",
            "anthropic/claude-instant-1",
            "google/palm-2-chat-bison",
            "google/palm-2-codechat-bison",
            "meta-llama/llama-2-70b-chat",
            "meta-llama/llama-2-13b-chat",
            "mistralai/mistral-7b-instruct",
            "mistralai/mixtral-8x7b-instruct",
        ]
    }
    
    pub async fn backup_file(&self, file_path: &PathBuf) -> Result<PathBuf> {
        if !self.backup_enabled {
            return Ok(file_path.clone());
        }
        
        if !self.backup_dir.exists() {
            fs::create_dir_all(&self.backup_dir).await?;
        }
        
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_{}_backup", file_name, timestamp);
        let backup_path = self.backup_dir.join(backup_name);
        
        fs::copy(file_path, &backup_path).await?;
        
        println!("📦 Backup created: {}", backup_path.display());
        Ok(backup_path)
    }
}