use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use tokio::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
}

pub fn change_extension(filename: &str, new_extension: &str) -> String {
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap_or_default();
    let stem_str = stem.to_string_lossy();
    
    if new_extension.starts_with('.') {
        format!("{}.{}", stem_str, &new_extension[1..])
    } else {
        format!("{}.{}", stem_str, new_extension)
    }
}

pub async fn create_temp_file(content: &str, extension: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let filename = format!("ai_cli_{}_{}", generate_uuid(), extension);
    let file_path = temp_dir.join(filename);
    
    fs::write(&file_path, content).await?;
    Ok(file_path)
}

pub async fn read_file_lines(path: &Path, max_lines: Option<usize>) -> Result<Vec<String>> {
    let content = fs::read_to_string(path).await?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    
    if let Some(max) = max_lines {
        Ok(lines.into_iter().take(max).collect())
    } else {
        Ok(lines)
    }
}

pub async fn append_to_file(path: &Path, content: &str) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    
    use tokio::io::AsyncWriteExt;
    file.write_all(content.as_bytes()).await?;
    file.write_all(b"\n").await?;
    
    Ok(())
}

pub async fn prepend_to_file(path: &Path, content: &str) -> Result<()> {
    let existing_content = fs::read_to_string(path).await?;
    let new_content = format!("{}\n{}", content, existing_content);
    fs::write(path, new_content).await?;
    Ok(())
}

pub async fn insert_into_file(path: &Path, line_number: usize, content: &str) -> Result<()> {
    let lines = read_file_lines(path, None).await?;
    
    if line_number > lines.len() {
        return Err(anyhow!("Line number {} exceeds file length {}", line_number, lines.len()));
    }
    
    let mut new_lines = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == line_number {
            new_lines.push(content.to_string());
        }
        new_lines.push(line.clone());
    }
    
    let new_content = new_lines.join("\n");
    fs::write(path, new_content).await?;
    Ok(())
}

pub async fn replace_line_in_file(path: &Path, line_number: usize, new_content: &str) -> Result<()> {
    let lines = read_file_lines(path, None).await?;
    
    if line_number >= lines.len() {
        return Err(anyhow!("Line number {} exceeds file length {}", line_number, lines.len()));
    }
    
    let mut new_lines = lines.clone();
    new_lines[line_number] = new_content.to_string();
    
    let new_content = new_lines.join("\n");
    fs::write(path, new_content).await?;
    Ok(())
}

pub async fn delete_line_from_file(path: &Path, line_number: usize) -> Result<()> {
    let lines = read_file_lines(path, None).await?;
    
    if line_number >= lines.len() {
        return Err(anyhow!("Line number {} exceeds file length {}", line_number, lines.len()));
    }
    
    let new_lines: Vec<String> = lines
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i != &line_number)
        .map(|(_, line)| line)
        .collect();
    
    let new_content = new_lines.join("\n");
    fs::write(path, new_content).await?;
    Ok(())
}

pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    
    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours % 24, minutes % 60, seconds % 60)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

pub async fn count_files_in_directory(path: &Path) -> Result<usize> {
    let mut count = 0;
    let mut entries = fs::read_dir(path).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            count += 1;
        } else if metadata.is_dir() {
            count += count_files_in_directory(&entry.path()).await?;
        }
    }
    
    Ok(count)
}

pub async fn get_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    let mut entries = fs::read_dir(path).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += get_directory_size(&entry.path()).await?;
        }
    }
    
    Ok(total_size)
}

pub fn is_text_file(filename: &str) -> bool {
    let text_extensions = [
        "txt", "md", "json", "xml", "html", "htm", "css", "js", "ts", "py", "rs", "java", "c", "cpp",
        "h", "hpp", "sh", "bash", "zsh", "fish", "ps1", "bat", "cmd", "yml", "yaml", "toml", "ini",
        "cfg", "conf", "log", "sql", "php", "rb", "go", "swift", "kt", "scala", "r", "m", "h",
        "cpp", "cc", "cxx", "c++", "hpp", "hxx", "h++", "cs", "fs", "fsx", "vb", "pl", "pm", "t",
        "pod", "lua", "coffee", "litcoffee", "ts", "tsx", "jsx", "vue", "svelte", "astro", "elm",
        "dart", "ex", "exs", "erl", "hrl", "clj", "cljs", "cljc", "edn", "f", "fsi", "fs", "fsx",
        "ml", "mli", "nim", "cr", "php", "php3", "php4", "php5", "phtml", "jl", "d", "di", "nim",
        "nimble", "v", "vsh", "sv", "svh", "vhdl", "vhd", "tcl", "tk", "rkt", "rktl", "rktd",
        "scrbl", "scm", "ss", "zig", "zon", "vala", "vapi", "pyx", "pxd", "pxi", "nim", "nimble",
    ];
    
    let extension = get_file_extension(filename).unwrap_or("");
    text_extensions.contains(&extension)
}

pub async fn create_directory_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub async fn copy_directory_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Err(anyhow!("Source directory does not exist: {}", src.display()));
    }
    
    if !src.is_dir() {
        return Err(anyhow!("Source is not a directory: {}", src.display()));
    }
    
    create_directory_if_not_exists(dst).await?;
    
    let mut entries = fs::read_dir(src).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if entry.file_type().await?.is_dir() {
            copy_directory_recursive(&src_path, &dst_path).await?;
        } else {
            fs::copy(&src_path, &dst_path).await?;
        }
    }
    
    Ok(())
}

pub async fn ensure_file_exists(path: &Path, default_content: &str) -> Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_directory_if_not_exists(parent).await?;
        }
        fs::write(path, default_content).await?;
    }
    Ok(())
}