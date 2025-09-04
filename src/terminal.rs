use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use which::which;

pub struct TerminalManager {
    shell: String,
}

impl TerminalManager {
    pub fn new() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| {
            if cfg!(target_os = "windows") {
                "cmd.exe".to_string()
            } else {
                "/bin/bash".to_string()
            }
        });
        
        Self { shell }
    }
    
    pub async fn execute_command(&self, command: &str) -> Result<String> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = TokioCommand::new("cmd");
            cmd.args(&["/C", command]);
            cmd
        } else {
            let mut cmd = TokioCommand::new(&self.shell);
            cmd.args(&["-c", command]);
            cmd
        };
        
        cmd.current_dir(std::env::current_dir()?);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let mut child = cmd.spawn()?;
        
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        
        if let Some(mut stdout_pipe) = child.stdout.take() {
            stdout_pipe.read_to_end(&mut stdout).await?;
        }
        
        if let Some(mut stderr_pipe) = child.stderr.take() {
            stderr_pipe.read_to_end(&mut stderr).await?;
        }
        
        let status = child.wait().await?;
        
        let stdout_str = String::from_utf8_lossy(&stdout);
        let stderr_str = String::from_utf8_lossy(&stderr);
        
        if !status.success() {
            return Err(anyhow!(
                "Command failed with exit code: {}\nStdout: {}\nStderr: {}",
                status.code().unwrap_or(-1),
                stdout_str,
                stderr_str
            ));
        }
        
        let mut output = stdout_str.to_string();
        if !stderr_str.is_empty() {
            output.push_str("\n");
            output.push_str(&stderr_str);
        }
        
        Ok(output)
    }
    
    pub async fn execute_command_with_input(&self, command: &str, input: &str) -> Result<String> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = TokioCommand::new("cmd");
            cmd.args(&["/C", command]);
            cmd
        } else {
            let mut cmd = TokioCommand::new(&self.shell);
            cmd.args(&["-c", command]);
            cmd
        };
        
        cmd.current_dir(std::env::current_dir()?);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let mut child = cmd.spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes()).await?;
            stdin.flush().await?;
            drop(stdin);
        }
        
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        
        if let Some(mut stdout_pipe) = child.stdout.take() {
            stdout_pipe.read_to_end(&mut stdout).await?;
        }
        
        if let Some(mut stderr_pipe) = child.stderr.take() {
            stderr_pipe.read_to_end(&mut stderr).await?;
        }
        
        let status = child.wait().await?;
        
        let stdout_str = String::from_utf8_lossy(&stdout);
        let stderr_str = String::from_utf8_lossy(&stderr);
        
        if !status.success() {
            return Err(anyhow!(
                "Command failed with exit code: {}\nStdout: {}\nStderr: {}",
                status.code().unwrap_or(-1),
                stdout_str,
                stderr_str
            ));
        }
        
        let mut output = stdout_str.to_string();
        if !stderr_str.is_empty() {
            output.push_str("\n");
            output.push_str(&stderr_str);
        }
        
        Ok(output)
    }
    
    pub async fn run_script(&self, script_path: &Path) -> Result<String> {
        if !script_path.exists() {
            return Err(anyhow!("Script file not found: {}", script_path.display()));
        }
        
        let command = if cfg!(target_os = "windows") {
            format!("cmd /C \"{}\"", script_path.display())
        } else {
            format!("chmod +x \"{}\" && \"{}\"", script_path.display(), script_path.display())
        };
        
        self.execute_command(&command).await
    }
    
    pub fn command_exists(&self, command: &str) -> bool {
        which(command).is_ok()
    }
    
    pub async fn get_environment_variable(&self, name: &str) -> Result<String> {
        std::env::var(name).map_err(|e| anyhow!("Failed to get environment variable {}: {}", name, e))
    }
    
    pub async fn set_environment_variable(&self, name: &str, value: &str) -> Result<()> {
        std::env::set_var(name, value);
        Ok(())
    }
    
    pub async fn get_working_directory(&self) -> Result<String> {
        std::env::current_dir()
            .map(|path| path.display().to_string())
            .map_err(|e| anyhow!("Failed to get working directory: {}", e))
    }
    
    pub async fn change_directory(&self, path: &Path) -> Result<()> {
        std::env::set_current_dir(path)
            .map_err(|e| anyhow!("Failed to change directory: {}", e))
    }
    
    pub async fn create_process(&self, command: &str) -> Result<u32> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = TokioCommand::new("cmd");
            cmd.args(&["/C", command]);
            cmd
        } else {
            let mut cmd = TokioCommand::new(&self.shell);
            cmd.args(&["-c", command]);
            cmd
        };
        
        cmd.current_dir(std::env::current_dir()?);
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        cmd.stdin(Stdio::null());
        
        let child = cmd.spawn()?;
        Ok(child.id())
    }
    
    pub async fn kill_process(&self, pid: u32) -> Result<()> {
        let kill_command = if cfg!(target_os = "windows") {
            format!("taskkill /F /PID {}", pid)
        } else {
            format!("kill -9 {}", pid)
        };
        
        self.execute_command(&kill_command).await?;
        Ok(())
    }
    
    pub async fn get_process_list(&self) -> Result<String> {
        let command = if cfg!(target_os = "windows") {
            "tasklist".to_string()
        } else {
            "ps aux".to_string()
        };
        
        self.execute_command(&command).await
    }
}