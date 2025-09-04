use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::fs;
use std::env;

mod ai;
mod file_ops;
mod terminal;
mod config;
mod utils;

use ai::OpenRouterClient;
use file_ops::FileManager;
use terminal::TerminalManager;
use config::Config;

#[derive(Parser)]
#[command(name = "ai-cli-agent")]
#[command(about = "AI-powered CLI agent for software development")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, help = "Custom AI prompt")]
    prompt: Option<String>,
    
    #[arg(short, long, help = "OpenRouter API key")]
    api_key: Option<String>,
    
    #[arg(short, long, help = "Model to use (e.g., openai/gpt-4)")]
    model: Option<String>,
    
    #[arg(short, long, help = "Working directory")]
    work_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a multi-step AI task
    Execute {
        #[arg(help = "Task description")]
        task: String,
    },
    /// Read file content
    Read {
        #[arg(help = "File path")]
        path: PathBuf,
    },
    /// Write content to file
    Write {
        #[arg(help = "File path")]
        path: PathBuf,
        #[arg(help = "Content to write")]
        content: String,
    },
    /// Edit file content
    Edit {
        #[arg(help = "File path")]
        path: PathBuf,
        #[arg(help = "Edit instructions")]
        instructions: String,
    },
    /// Delete file or directory
    Delete {
        #[arg(help = "File or directory path")]
        path: PathBuf,
    },
    /// List directory contents
    List {
        #[arg(help = "Directory path")]
        path: Option<PathBuf>,
    },
    /// Execute terminal command
    Run {
        #[arg(help = "Command to execute")]
        command: String,
    },
    /// Interactive mode
    Interactive,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load configuration
    let mut config = Config::load().await?;
    
    // Override config with command line arguments
    if let Some(api_key) = cli.api_key {
        config.openrouter_api_key = api_key;
    }
    if let Some(model) = cli.model {
        config.model = model;
    }
    if let Some(work_dir) = cli.work_dir {
        config.work_dir = work_dir;
    }
    
    // Validate configuration
    if config.openrouter_api_key.is_empty() {
        return Err(anyhow!("OpenRouter API key is required. Set it in config or use --api-key"));
    }
    
    // Initialize managers
    let ai_client = OpenRouterClient::new(&config.openrouter_api_key, &config.model);
    let file_manager = FileManager::new(&config.work_dir);
    let terminal_manager = TerminalManager::new();
    
    // Execute command
    match cli.command {
        Commands::Execute { task } => {
            let prompt = cli.prompt.unwrap_or_else(|| 
                format!("You are an AI software development assistant. Execute the following task: {}", task)
            );
            execute_task(&ai_client, &file_manager, &terminal_manager, &prompt, &task).await?;
        }
        Commands::Read { path } => {
            let content = file_manager.read_file(&path).await?;
            println!("{}", content);
        }
        Commands::Write { path, content } => {
            file_manager.write_file(&path, &content).await?;
            println!("File written successfully: {}", path.display());
        }
        Commands::Edit { path, instructions } => {
            file_manager.edit_file(&ai_client, &path, &instructions).await?;
            println!("File edited successfully: {}", path.display());
        }
        Commands::Delete { path } => {
            file_manager.delete_path(&path).await?;
            println!("Deleted successfully: {}", path.display());
        }
        Commands::List { path } => {
            let path = path.unwrap_or_else(|| PathBuf::from("."));
            let contents = file_manager.list_directory(&path).await?;
            for item in contents {
                println!("{}", item);
            }
        }
        Commands::Run { command } => {
            let output = terminal_manager.execute_command(&command).await?;
            println!("{}", output);
        }
        Commands::Interactive => {
            interactive_mode(&ai_client, &file_manager, &terminal_manager).await?;
        }
    }
    
    Ok(())
}

async fn execute_task(
    ai_client: &OpenRouterClient,
    file_manager: &FileManager,
    terminal_manager: &TerminalManager,
    system_prompt: &str,
    task: &str,
) -> Result<()> {
    println!("🤖 AI CLI Agent - Executing task: {}", task);
    println!("=====================================");
    
    let context = format!(
        "Current working directory: {}\n\nTask: {}",
        file_manager.get_current_dir().display(),
        task
    );
    
    let response = ai_client.send_message(system_prompt, &context).await?;
    
    // Parse and execute the AI's response
    let actions = parse_ai_response(&response);
    
    for action in actions {
        match action {
            AIAction::WriteFile { path, content } => {
                println!("📝 Writing file: {}", path);
                file_manager.write_file(&PathBuf::from(path), &content).await?;
            }
            AIAction::EditFile { path, instructions } => {
                println!("✏️ Editing file: {}", path);
                file_manager.edit_file(ai_client, &PathBuf::from(path), &instructions).await?;
            }
            AIAction::RunCommand { command } => {
                println!("⚡ Running command: {}", command);
                let output = terminal_manager.execute_command(&command).await?;
                println!("Output: {}", output);
            }
            AIAction::ReadFile { path } => {
                println!("📖 Reading file: {}", path);
                let content = file_manager.read_file(&PathBuf::from(path)).await?;
                println!("Content: {}", content);
            }
            AIAction::DeleteFile { path } => {
                println!("🗑️ Deleting file: {}", path);
                file_manager.delete_path(&PathBuf::from(path)).await?;
            }
            AIAction::CreateDirectory { path } => {
                println!("📁 Creating directory: {}", path);
                fs::create_dir_all(&path).await?;
            }
            AIAction::ListDirectory { path } => {
                println!("📋 Listing directory: {}", path);
                let contents = file_manager.list_directory(&PathBuf::from(path)).await?;
                for item in contents {
                    println!("  {}", item);
                }
            }
        }
    }
    
    println!("✅ Task completed successfully!");
    Ok(())
}

async fn interactive_mode(
    ai_client: &OpenRouterClient,
    file_manager: &FileManager,
    terminal_manager: &TerminalManager,
) -> Result<()> {
    use dialoguer::Input;
    
    println!("🤖 AI CLI Agent - Interactive Mode");
    println!("Type 'exit' to quit");
    println!("=================================");
    
    loop {
        let task: String = Input::new()
            .with_prompt("Task")
            .interact_text()?;
        
        if task.to_lowercase() == "exit" {
            break;
        }
        
        if let Err(e) = execute_task(
            ai_client,
            file_manager,
            terminal_manager,
            "You are an AI software development assistant. Execute the following task:",
            &task,
        ).await {
            eprintln!("❌ Error: {}", e);
        }
    }
    
    Ok(())
}

#[derive(Debug)]
enum AIAction {
    WriteFile { path: String, content: String },
    EditFile { path: String, instructions: String },
    RunCommand { command: String },
    ReadFile { path: String },
    DeleteFile { path: String },
    CreateDirectory { path: String },
    ListDirectory { path: String },
}

fn parse_ai_response(response: &str) -> Vec<AIAction> {
    let mut actions = Vec::new();
    
    // Simple parsing - in a real implementation, this would be more sophisticated
    let lines: Vec<&str> = response.lines().collect();
    let mut current_action = String::new();
    let mut current_type = None;
    
    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("WRITE_FILE:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("write");
            current_action = trimmed["WRITE_FILE:".len()..].trim().to_string();
        } else if trimmed.starts_with("EDIT_FILE:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("edit");
            current_action = trimmed["EDIT_FILE:".len()..].trim().to_string();
        } else if trimmed.starts_with("RUN_COMMAND:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("run");
            current_action = trimmed["RUN_COMMAND:".len()..].trim().to_string();
        } else if trimmed.starts_with("READ_FILE:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("read");
            current_action = trimmed["READ_FILE:".len()..].trim().to_string();
        } else if trimmed.starts_with("DELETE_FILE:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("delete");
            current_action = trimmed["DELETE_FILE:".len()..].trim().to_string();
        } else if trimmed.starts_with("CREATE_DIRECTORY:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("create_dir");
            current_action = trimmed["CREATE_DIRECTORY:".len()..].trim().to_string();
        } else if trimmed.starts_with("LIST_DIRECTORY:") {
            if let Some(action) = finalize_action(&current_action, &current_type) {
                actions.push(action);
            }
            current_type = Some("list_dir");
            current_action = trimmed["LIST_DIRECTORY:".len()..].trim().to_string();
        } else if !trimmed.is_empty() {
            current_action.push('\n');
            current_action.push_str(trimmed);
        }
    }
    
    if let Some(action) = finalize_action(&current_action, &current_type) {
        actions.push(action);
    }
    
    actions
}

fn finalize_action(action: &str, action_type: &Option<&str>) -> Option<AIAction> {
    let action_type = action_type?;
    if action.is_empty() {
        return None;
    }
    
    let parts: Vec<&str> = action.splitn(2, '\n').collect();
    let first_line = parts[0].trim();
    let content = if parts.len() > 1 { parts[1].trim() } else { "" };
    
    match action_type {
        "write" => Some(AIAction::WriteFile {
            path: first_line.to_string(),
            content: content.to_string(),
        }),
        "edit" => Some(AIAction::EditFile {
            path: first_line.to_string(),
            instructions: content.to_string(),
        }),
        "run" => Some(AIAction::RunCommand {
            command: action.to_string(),
        }),
        "read" => Some(AIAction::ReadFile {
            path: first_line.to_string(),
        }),
        "delete" => Some(AIAction::DeleteFile {
            path: first_line.to_string(),
        }),
        "create_dir" => Some(AIAction::CreateDirectory {
            path: first_line.to_string(),
        }),
        "list_dir" => Some(AIAction::ListDirectory {
            path: first_line.to_string(),
        }),
        _ => None,
    }
}