#!/bin/bash

# Example: Creating and modifying a Snake Game with AI CLI Agent
# This script demonstrates the multi-step capabilities of the AI CLI Agent

echo "🎮 AI CLI Agent - Snake Game Example"
echo "====================================="

# Check if the binary exists
if [ ! -f "../target/release/ai-cli-agent" ]; then
    echo "🔨 Building the AI CLI Agent..."
    cd ..
    cargo build --release
    cd examples
fi

# Set up environment
export OPENROUTER_API_KEY="${OPENROUTER_API_KEY:-your_api_key_here}"

if [ "$OPENROUTER_API_KEY" = "your_api_key_here" ]; then
    echo "❌ Please set your OpenRouter API key:"
    echo "   export OPENROUTER_API_KEY=your_actual_api_key"
    exit 1
fi

# Create a temporary directory for our example
EXAMPLE_DIR="./snake_game_example"
mkdir -p "$EXAMPLE_DIR"
cd "$EXAMPLE_DIR"

echo "📁 Working directory: $(pwd)"

# Step 1: Create a complete Snake Game
echo ""
echo "🎯 Step 1: Creating a complete Snake Game..."
echo "=========================================="

../target/release/ai-cli-agent execute \
    --prompt "You are an expert web developer. Create a complete, playable Snake game with modern styling." \
    "write a complete HTML snake game with CSS styling and JavaScript functionality. Include score tracking, game controls, and a modern design."

echo ""
echo "✅ Step 1 completed! Let's see what was created:"
ls -la

# Step 2: Read and examine the created files
echo ""
echo "📖 Step 2: Examining the created files..."
echo "========================================"

for file in *.html *.css *.js 2>/dev/null; do
    if [ -f "$file" ]; then
        echo ""
        echo "📄 Content of $file:"
        echo "===================="
        cat "$file"
        echo ""
        echo "===================="
    fi
done

# Step 3: Modify the game with gradient background
echo ""
echo "🎨 Step 3: Adding gradient background..."
echo "======================================"

# Find the HTML file
HTML_FILE=$(ls *.html 2>/dev/null | head -n 1)

if [ -n "$HTML_FILE" ]; then
    echo "📝 Found HTML file: $HTML_FILE"
    
    ../target/release/ai-cli-agent edit \
        --path "$HTML_FILE" \
        "change the background to a beautiful gradient color scheme that transitions from deep blue to purple, and make sure the game elements are still visible and look good on this background."
    
    echo ""
    echo "✅ Step 3 completed! Modified file content:"
    echo "========================================"
    cat "$HTML_FILE"
    echo ""
    echo "========================================"
else
    echo "❌ No HTML file found to edit"
fi

# Step 4: Add additional features
echo ""
echo "🚀 Step 4: Adding additional features..."
echo "======================================"

if [ -n "$HTML_FILE" ]; then
    ../target/release/ai-cli-agent edit \
        --path "$HTML_FILE" \
        "add a pause/resume feature to the game, include a high score system that persists using localStorage, and add sound effects for eating food and game over."
    
    echo ""
    echo "✅ Step 4 completed! Final game features added."
fi

# Step 5: Test the game in browser (if possible)
echo ""
echo "🌐 Step 5: Testing the game..."
echo "============================="

if command -v python3 &> /dev/null; then
    echo "🚀 Starting a local web server to test the game..."
    echo "Open your browser and go to: http://localhost:8000"
    echo "Press Ctrl+C to stop the server."
    echo ""
    
    python3 -m http.server 8000 &
    SERVER_PID=$!
    
    # Wait for a moment to let the server start
    sleep 2
    
    # Try to open the browser (optional)
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:8000/$HTML_FILE" 2>/dev/null
    elif command -v open &> /dev/null; then
        open "http://localhost:8000/$HTML_FILE" 2>/dev/null
    fi
    
    echo "🎮 Game is running! Press Enter to stop the server and continue..."
    read
    
    kill $SERVER_PID 2>/dev/null
    echo "✅ Server stopped."
else
    echo "ℹ️  To test the game, open the HTML file in your browser:"
    echo "   file://$(pwd)/$HTML_FILE"
fi

# Step 6: Summary
echo ""
echo "📊 Step 6: Summary of what was accomplished..."
echo "============================================"

echo "✅ Created a complete Snake Game with:"
echo "   - HTML structure"
echo "   - CSS styling"
echo "   - JavaScript game logic"
echo "   - Score tracking"
echo "   - Game controls"
echo "   - Modern design"

echo ""
echo "✅ Modified the game to include:"
echo "   - Beautiful gradient background"
echo "   - Pause/resume functionality"
echo "   - High score system with localStorage"
echo "   - Sound effects"

echo ""
echo "📁 All files created in: $(pwd)"
echo "🎮 You can play the game by opening $HTML_FILE in your browser"

# Cleanup option
echo ""
echo "🧹 Would you like to clean up the example files? (y/N)"
read -r cleanup_response

if [[ $cleanup_response =~ ^[Yy]$ ]]; then
    cd ..
    rm -rf "$EXAMPLE_DIR"
    echo "✅ Example files cleaned up."
else
    echo "📁 Example files kept in: $EXAMPLE_DIR"
fi

echo ""
echo "🎉 AI CLI Agent Snake Game Example completed!"
echo ""
echo "You can now use the AI CLI Agent for your own projects:"
echo "  ../target/release/ai-cli-agent execute \"your task description\""
echo "  ../target/release/ai-cli-agent interactive"