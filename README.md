# AI CLI Agent

A powerful AI-powered command-line interface agent that integrates with OpenRouter API to perform automated software development tasks. Built with Rust for performance and reliability.

## Features

- 🤖 **AI-Powered**: Leverages OpenRouter API to access various AI models
- 📁 **File Operations**: Read, write, edit, and delete files and directories
- 💻 **Terminal Integration**: Execute shell commands and manage processes
- 🎯 **Multi-Step Tasks**: Execute complex development workflows
- 🔄 **Interactive Mode**: Chat with the AI for iterative development
- ⚡ **Fast & Efficient**: Built with Rust for optimal performance
- 🔧 **Configurable**: Customizable settings and model selection

## Installation

### Prerequisites

- Rust (latest stable version)
- OpenRouter API key
- Git

### Setup

1. Clone the repository:
```bash
git clone https://github.com/your-username/ai-cli-agent.git
cd ai-cli-agent
```

2. Set up your OpenRouter API key:
```bash
export OPENROUTER_API_KEY="your_api_key_here"
```

3. Build the project:
```bash
cargo build --release
```

4. Run the test script to verify everything works:
```bash
chmod +x test_ai_agent.sh
./test_ai_agent.sh
```

## Usage

### Basic Commands

#### Execute a Task
```bash
cargo run -- execute "write a hello world program in python"
```

#### File Operations
```bash
# Read a file
cargo run -- read path/to/file.txt

# Write a file
cargo run -- write path/to/file.txt "Hello, World!"

# Edit a file
cargo run -- edit path/to/file.txt "change hello to hi"

# Delete a file
cargo run -- delete path/to/file.txt

# List directory
cargo run -- list path/to/directory
```

#### Terminal Commands
```bash
# Run a command
cargo run -- run "ls -la"
```

#### Interactive Mode
```bash
cargo run -- interactive
```

### Advanced Usage

#### Custom Model Selection
```bash
cargo run -- execute --model "anthropic/claude-2" "write a rust program"
```

#### Custom Working Directory
```bash
cargo run -- execute --work-dir "/path/to/project" "analyze this codebase"
```

#### Custom System Prompt
```bash
cargo run -- execute --prompt "You are an expert Rust developer" "write a web server"
```

## Configuration

The AI CLI Agent uses a configuration file located at `~/.config/ai-cli-agent/config.toml`. You can customize various settings:

```toml
openrouter_api_key = "your_api_key_here"
model = "openai/gpt-4"
work_dir = "/path/to/your/project"
max_tokens = 4000
temperature = 0.7
timeout_seconds = 120
auto_save = true
backup_enabled = true
backup_dir = ".ai_cli_backups"
```

### Available Models

The agent supports various models through OpenRouter:

- `openai/gpt-4`
- `openai/gpt-4-turbo`
- `openai/gpt-3.5-turbo`
- `anthropic/claude-2`
- `anthropic/claude-instant-1`
- `google/palm-2-chat-bison`
- `google/palm-2-codechat-bison`
- `meta-llama/llama-2-70b-chat`
- `meta-llama/llama-2-13b-chat`
- `mistralai/mistral-7b-instruct`
- `mistralai/mixtral-8x7b-instruct`

## Examples

### Example 1: Create a Snake Game

```bash
cargo run -- execute "write a complete HTML snake game with CSS and JavaScript"
```

### Example 2: Modify the Game

```bash
cargo run -- edit snake.html "change the background to a gradient from blue to purple"
```

### Example 3: Build a Rust Project

```bash
cargo run -- execute "create a new Rust project with a simple web server using tokio"
```

### Example 4: Code Analysis

```bash
cargo run -- execute "analyze this codebase and suggest improvements"
```

## API Response Format

The AI agent understands structured responses in the following format:

```
WRITE_FILE: filename.html
<!DOCTYPE html>
<html>
<head>
    <title>My Page</title>
</head>
<body>
    <h1>Hello World</h1>
</body>
</html>

RUN_COMMAND: npm install
npm install

EDIT_FILE: style.css
Change the background color to blue
```

## Development

### Project Structure

```
ai-cli-agent/
├── src/
│   ├── main.rs          # Main entry point
│   ├── ai.rs            # OpenRouter API integration
│   ├── file_ops.rs      # File operations
│   ├── terminal.rs      # Terminal operations
│   ├── config.rs        # Configuration management
│   └── utils.rs         # Utility functions
├── Cargo.toml           # Rust dependencies
├── test_ai_agent.sh     # Test script
└── README.md           # This file
```

### Adding New Features

1. Add new dependencies to `Cargo.toml`
2. Implement the feature in the appropriate module
3. Add CLI commands in `main.rs`
4. Update the configuration if needed
5. Add tests

### Testing

Run the test script:
```bash
./test_ai_agent.sh
```

Run unit tests:
```bash
cargo test
```

## Troubleshooting

### Common Issues

1. **API Key Not Found**
   - Make sure you've set the `OPENROUTER_API_KEY` environment variable
   - Check your configuration file

2. **Permission Denied**
   - Ensure you have write permissions in the target directory
   - Check file system permissions

3. **Network Issues**
   - Verify your internet connection
   - Check if OpenRouter API is accessible

4. **Build Errors**
   - Ensure you have the latest Rust version
   - Run `cargo clean` and try building again

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug cargo run -- execute "your task"
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support, please open an issue on GitHub or contact the maintainers.

## Acknowledgments

- OpenRouter for providing AI model access
- Rust community for excellent tools and libraries
- All contributors who have helped improve this project