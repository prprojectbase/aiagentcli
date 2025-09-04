#!/usr/bin/env python3
"""
Demo script showing how the AI CLI Agent would work.
This simulates the behavior of the Rust-based AI CLI Agent.
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Any

class AICLIAgentDemo:
    def __init__(self, api_key: str = "demo_key"):
        self.api_key = api_key
        self.work_dir = Path.cwd()
        self.backup_dir = Path(".ai_cli_backups")
        self.backup_dir.mkdir(exist_ok=True)
        
    def simulate_ai_response(self, prompt: str, context: str = "") -> str:
        """Simulate AI response based on the prompt"""
        if "snake game" in prompt.lower() and "html" in prompt.lower():
            return self._generate_snake_game_html()
        elif "gradient background" in prompt.lower():
            return self._generate_gradient_background_edit()
        else:
            return f"AI Response for: {prompt}"
    
    def _generate_snake_game_html(self) -> str:
        """Generate HTML snake game code"""
        return '''WRITE_FILE: snake_game.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Snake Game</title>
    <style>
        body {
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background-color: #1a1a1a;
            font-family: Arial, sans-serif;
        }
        .game-container {
            text-align: center;
        }
        canvas {
            border: 2px solid #fff;
            background-color: #000;
        }
        .score {
            color: #fff;
            font-size: 24px;
            margin: 20px 0;
        }
        .controls {
            color: #fff;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <div class="game-container">
        <h1 style="color: #fff;">Snake Game</h1>
        <div class="score">Score: <span id="score">0</span></div>
        <canvas id="gameCanvas" width="400" height="400"></canvas>
        <div class="controls">
            <p>Use arrow keys to control the snake</p>
            <button onclick="startGame()">Start Game</button>
            <button onclick="pauseGame()">Pause</button>
        </div>
    </div>

    <script>
        const canvas = document.getElementById('gameCanvas');
        const ctx = canvas.getContext('2d');
        const scoreElement = document.getElementById('score');

        let snake = [{x: 200, y: 200}];
        let direction = {x: 0, y: 0};
        let food = {x: 0, y: 0};
        let score = 0;
        let gameRunning = false;
        let gamePaused = false;

        function generateFood() {
            food.x = Math.floor(Math.random() * 20) * 20;
            food.y = Math.floor(Math.random() * 20) * 20;
        }

        function drawGame() {
            // Clear canvas
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            // Draw snake
            ctx.fillStyle = '#0f0';
            snake.forEach(segment => {
                ctx.fillRect(segment.x, segment.y, 18, 18);
            });

            // Draw food
            ctx.fillStyle = '#f00';
            ctx.fillRect(food.x, food.y, 18, 18);
        }

        function updateGame() {
            if (!gameRunning || gamePaused) return;

            // Move snake head
            const head = {x: snake[0].x + direction.x, y: snake[0].y + direction.y};

            // Check wall collision
            if (head.x < 0 || head.x >= canvas.width || head.y < 0 || head.y >= canvas.height) {
                gameOver();
                return;
            }

            // Check self collision
            if (snake.some(segment => segment.x === head.x && segment.y === head.y)) {
                gameOver();
                return;
            }

            snake.unshift(head);

            // Check food collision
            if (head.x === food.x && head.y === food.y) {
                score += 10;
                scoreElement.textContent = score;
                generateFood();
            } else {
                snake.pop();
            }

            drawGame();
        }

        function gameOver() {
            gameRunning = false;
            alert(`Game Over! Score: ${score}`);
        }

        function startGame() {
            snake = [{x: 200, y: 200}];
            direction = {x: 20, y: 0};
            score = 0;
            scoreElement.textContent = score;
            gameRunning = true;
            gamePaused = false;
            generateFood();
            drawGame();
        }

        function pauseGame() {
            gamePaused = !gamePaused;
        }

        // Keyboard controls
        document.addEventListener('keydown', (e) => {
            if (!gameRunning) return;

            switch(e.key) {
                case 'ArrowUp':
                    if (direction.y === 0) {
                        direction = {x: 0, y: -20};
                    }
                    break;
                case 'ArrowDown':
                    if (direction.y === 0) {
                        direction = {x: 0, y: 20};
                    }
                    break;
                case 'ArrowLeft':
                    if (direction.x === 0) {
                        direction = {x: -20, y: 0};
                    }
                    break;
                case 'ArrowRight':
                    if (direction.x === 0) {
                        direction = {x: 20, y: 0};
                    }
                    break;
            }
        });

        // Game loop
        setInterval(updateGame, 150);

        // Initial draw
        drawGame();
    </script>
</body>
</html>'''

    def _generate_gradient_background_edit(self) -> str:
        """Generate edit instructions for gradient background"""
        return '''EDIT_FILE: snake_game.html
Replace the background-color in the body CSS with a gradient from deep blue to purple:

body {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    margin: 0;
    background: linear-gradient(135deg, #1e3c72 0%, #2a5298 50%, #7e22ce 100%);
    font-family: Arial, sans-serif;
}'''

    def execute_task(self, task: str) -> bool:
        """Execute a task using simulated AI responses"""
        print(f"🤖 Executing task: {task}")
        print("=" * 50)
        
        # Get AI response
        ai_response = self.simulate_ai_response(task)
        
        # Parse and execute actions
        actions = self._parse_ai_response(ai_response)
        
        for action in actions:
            success = self._execute_action(action)
            if not success:
                print(f"❌ Failed to execute action: {action}")
                return False
        
        print("✅ Task completed successfully!")
        return True
    
    def _parse_ai_response(self, response: str) -> List[Dict[str, Any]]:
        """Parse AI response into actions"""
        actions = []
        lines = response.split('\n')
        
        current_action = None
        current_content = []
        
        for line in lines:
            line = line.strip()
            if line.startswith('WRITE_FILE:'):
                if current_action:
                    current_action['content'] = '\n'.join(current_content)
                    actions.append(current_action)
                current_action = {
                    'type': 'write',
                    'path': line[len('WRITE_FILE:'):].strip(),
                    'content': ''
                }
                current_content = []
            elif line.startswith('EDIT_FILE:'):
                if current_action:
                    current_action['content'] = '\n'.join(current_content)
                    actions.append(current_action)
                current_action = {
                    'type': 'edit',
                    'path': line[len('EDIT_FILE:'):].strip(),
                    'content': ''
                }
                current_content = []
            elif line.startswith('RUN_COMMAND:'):
                if current_action:
                    current_action['content'] = '\n'.join(current_content)
                    actions.append(current_action)
                current_action = {
                    'type': 'run',
                    'command': line[len('RUN_COMMAND:'):].strip(),
                    'content': ''
                }
                current_content = []
            elif line and current_action:
                current_content.append(line)
        
        if current_action:
            current_action['content'] = '\n'.join(current_content)
            actions.append(current_action)
        
        return actions
    
    def _execute_action(self, action: Dict[str, Any]) -> bool:
        """Execute a single action"""
        try:
            if action['type'] == 'write':
                return self._write_file(action['path'], action['content'])
            elif action['type'] == 'edit':
                return self._edit_file(action['path'], action['content'])
            elif action['type'] == 'run':
                return self._run_command(action['command'])
            else:
                print(f"❌ Unknown action type: {action['type']}")
                return False
        except Exception as e:
            print(f"❌ Error executing action: {e}")
            return False
    
    def _write_file(self, path: str, content: str) -> bool:
        """Write content to a file"""
        file_path = Path(path)
        
        # Create backup if file exists
        if file_path.exists():
            backup_path = self.backup_dir / f"{file_path.name}_{self._get_timestamp()}_backup"
            backup_path.write_text(file_path.read_text())
            print(f"📦 Backup created: {backup_path}")
        
        # Create parent directories if needed
        file_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Write file
        file_path.write_text(content)
        print(f"📝 File written: {file_path}")
        return True
    
    def _edit_file(self, path: str, instructions: str) -> bool:
        """Edit a file based on instructions"""
        file_path = Path(path)
        
        if not file_path.exists():
            print(f"❌ File not found: {file_path}")
            return False
        
        # Create backup
        backup_path = self.backup_dir / f"{file_path.name}_{self._get_timestamp()}_backup"
        backup_path.write_text(file_path.read_text())
        print(f"📦 Backup created: {backup_path}")
        
        # For demo purposes, we'll simulate the edit
        current_content = file_path.read_text()
        
        # Simple simulation of gradient background edit
        if "gradient" in instructions.lower():
            new_content = current_content.replace(
                "background-color: #1a1a1a;",
                "background: linear-gradient(135deg, #1e3c72 0%, #2a5298 50%, #7e22ce 100%);"
            )
        else:
            new_content = current_content  # No change for demo
        
        file_path.write_text(new_content)
        print(f"✏️ File edited: {file_path}")
        return True
    
    def _run_command(self, command: str) -> bool:
        """Run a shell command"""
        try:
            result = subprocess.run(command, shell=True, capture_output=True, text=True)
            if result.returncode == 0:
                print(f"⚡ Command executed: {command}")
                if result.stdout:
                    print(f"Output: {result.stdout}")
                return True
            else:
                print(f"❌ Command failed: {command}")
                print(f"Error: {result.stderr}")
                return False
        except Exception as e:
            print(f"❌ Error running command: {e}")
            return False
    
    def _get_timestamp(self) -> str:
        """Get current timestamp"""
        import datetime
        return datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    
    def list_directory(self, path: str = ".") -> List[str]:
        """List directory contents"""
        dir_path = Path(path)
        if not dir_path.exists():
            return []
        
        items = []
        for item in dir_path.iterdir():
            item_type = "DIR" if item.is_dir() else "FILE"
            items.append(f"{item.name} [{item_type}]")
        
        return sorted(items)
    
    def read_file(self, path: str) -> str:
        """Read file content"""
        file_path = Path(path)
        if not file_path.exists():
            return f"File not found: {file_path}"
        
        return file_path.read_text()

def main():
    """Main demo function"""
    print("🤖 AI CLI Agent Demo")
    print("=" * 50)
    
    # Initialize the demo agent
    agent = AICLIAgentDemo()
    
    # Test 1: Create Snake Game
    print("\n🎮 Test 1: Creating Snake Game")
    print("-" * 30)
    success1 = agent.execute_task("write a code in html for a snake game")
    
    if success1:
        print("\n📁 Files created:")
        for item in agent.list_directory():
            print(f"  {item}")
    
    # Test 2: Edit Snake Game
    print("\n🎨 Test 2: Adding Gradient Background")
    print("-" * 40)
    success2 = agent.execute_task("change the background of the game to a gradient color")
    
    # Show final result
    if success1 and success2:
        print("\n📄 Final snake_game.html content:")
        print("-" * 40)
        content = agent.read_file("snake_game.html")
        print(content)
    
    print("\n🎉 Demo completed!")
    print("\nTo run the actual Rust-based AI CLI Agent:")
    print("1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
    print("2. Set OPENROUTER_API_KEY environment variable")
    print("3. Run: cargo run -- execute 'your task'")

if __name__ == "__main__":
    main()