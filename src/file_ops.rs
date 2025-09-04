use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use tokio::fs;
use std::env;
use walkdir::WalkDir;

use crate::ai::OpenRouterClient;

pub struct FileManager {
    work_dir: PathBuf,
}

impl FileManager {
    pub fn new(work_dir: &Path) -> Self {
        Self {
            work_dir: work_dir.to_path_buf(),
        }
    }
    
    pub fn get_current_dir(&self) -> PathBuf {
        env::current_dir().unwrap_or_else(|_| self.work_dir.clone())
    }
    
    pub async fn read_file(&self, path: &Path) -> Result<String> {
        let full_path = self.resolve_path(path);
        
        if !full_path.exists() {
            return Err(anyhow!("File not found: {}", full_path.display()));
        }
        
        let content = fs::read_to_string(&full_path).await?;
        Ok(content)
    }
    
    pub async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        let full_path = self.resolve_path(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::write(&full_path, content).await?;
        println!("✅ File written: {}", full_path.display());
        Ok(())
    }
    
    pub async fn edit_file(&self, ai_client: &OpenRouterClient, path: &Path, instructions: &str) -> Result<()> {
        let full_path = self.resolve_path(path);
        
        if !full_path.exists() {
            return Err(anyhow!("File not found: {}", full_path.display()));
        }
        
        let current_content = self.read_file(path).await?;
        let new_content = ai_client.edit_code(&current_content, instructions).await?;
        
        self.write_file(path, &new_content).await?;
        Ok(())
    }
    
    pub async fn delete_path(&self, path: &Path) -> Result<()> {
        let full_path = self.resolve_path(path);
        
        if !full_path.exists() {
            return Err(anyhow!("Path not found: {}", full_path.display()));
        }
        
        if full_path.is_dir() {
            fs::remove_dir_all(&full_path).await?;
        } else {
            fs::remove_file(&full_path).await?;
        }
        
        println!("✅ Deleted: {}", full_path.display());
        Ok(())
    }
    
    pub async fn list_directory(&self, path: &Path) -> Result<Vec<String>> {
        let full_path = self.resolve_path(path);
        
        if !full_path.exists() {
            return Err(anyhow!("Directory not found: {}", full_path.display()));
        }
        
        if !full_path.is_dir() {
            return Err(anyhow!("Path is not a directory: {}", full_path.display()));
        }
        
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(&full_path).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            
            let metadata = entry.metadata().await?;
            let file_type = if metadata.is_dir() {
                "DIR"
            } else if metadata.is_file() {
                "FILE"
            } else {
                "OTHER"
            };
            
            entries.push(format!("{} [{}]", name, file_type));
        }
        
        entries.sort();
        Ok(entries)
    }
    
    pub async fn copy_file(&self, src: &Path, dst: &Path) -> Result<()> {
        let src_path = self.resolve_path(src);
        let dst_path = self.resolve_path(dst);
        
        if !src_path.exists() {
            return Err(anyhow!("Source file not found: {}", src_path.display()));
        }
        
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::copy(&src_path, &dst_path).await?;
        println!("✅ Copied: {} -> {}", src_path.display(), dst_path.display());
        Ok(())
    }
    
    pub async fn move_file(&self, src: &Path, dst: &Path) -> Result<()> {
        let src_path = self.resolve_path(src);
        let dst_path = self.resolve_path(dst);
        
        if !src_path.exists() {
            return Err(anyhow!("Source file not found: {}", src_path.display()));
        }
        
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::rename(&src_path, &dst_path).await?;
        println!("✅ Moved: {} -> {}", src_path.display(), dst_path.display());
        Ok(())
    }
    
    pub async fn create_directory(&self, path: &Path) -> Result<()> {
        let full_path = self.resolve_path(path);
        fs::create_dir_all(&full_path).await?;
        println!("✅ Directory created: {}", full_path.display());
        Ok(())
    }
    
    pub async fn file_exists(&self, path: &Path) -> bool {
        let full_path = self.resolve_path(path);
        full_path.exists()
    }
    
    pub async fn get_file_info(&self, path: &Path) -> Result<String> {
        let full_path = self.resolve_path(path);
        
        if !full_path.exists() {
            return Err(anyhow!("File not found: {}", full_path.display()));
        }
        
        let metadata = fs::metadata(&full_path).await?;
        let file_type = if metadata.is_dir() {
            "Directory"
        } else if metadata.is_file() {
            "File"
        } else {
            "Other"
        };
        
        let size = metadata.len();
        let modified = metadata.modified()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|_| "Unknown".to_string());
        
        Ok(format!(
            "Path: {}\nType: {}\nSize: {} bytes\nModified: {}",
            full_path.display(),
            file_type,
            size,
            modified
        ))
    }
    
    pub async fn search_files(&self, pattern: &str, directory: &Path) -> Result<Vec<String>> {
        let full_path = self.resolve_path(directory);
        
        if !full_path.exists() || !full_path.is_dir() {
            return Err(anyhow!("Directory not found: {}", full_path.display()));
        }
        
        let mut matches = Vec::new();
        
        for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.to_lowercase().contains(&pattern.to_lowercase()) {
                    matches.push(path.display().to_string());
                }
            }
        }
        
        Ok(matches)
    }
    
    fn resolve_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.work_dir.join(path)
        }
    }
}