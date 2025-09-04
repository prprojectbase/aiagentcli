#!/bin/bash

# Test script for AI CLI Agent
# This script demonstrates the AI agent's ability to:
# 1. Write HTML code for a snake game
# 2. Edit the file to change background to gradient color

echo "🤖 AI CLI Agent Test Script"
echo "============================"

# Check if we have the required dependencies
echo "📋 Checking dependencies..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first."
    exit 1
fi

if [ -z "$OPENROUTER_API_KEY" ]; then
    echo "❌ OPENROUTER_API_KEY environment variable is not set."
    echo "Please set it with: export OPENROUTER_API_KEY=your_api_key"
    exit 1
fi

echo "✅ Dependencies check passed"

# Build the project
echo "🔨 Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Create test directory
echo "📁 Creating test directory..."
TEST_DIR="./ai_agent_test"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "✅ Test directory created: $TEST_DIR"

# Test 1: Write HTML snake game
echo ""
echo "🎮 Test 1: Writing HTML snake game..."
echo "===================================="

../target/release/ai-cli-agent execute --api-key "$OPENROUTER_API_KEY" "write a code in html for a snake game"

if [ $? -eq 0 ]; then
    echo "✅ Snake game creation test passed"
    
    # Check if files were created
    if ls *.html 1> /dev/null 2>&1; then
        echo "📄 HTML files created:"
        ls -la *.html
    fi
else
    echo "❌ Snake game creation test failed"
fi

# Test 2: Edit the snake game to add gradient background
echo ""
echo "🎨 Test 2: Adding gradient background..."
echo "========================================"

# Find the HTML file
HTML_FILE=$(ls *.html | head -n 1)

if [ -n "$HTML_FILE" ]; then
    echo "📝 Found HTML file: $HTML_FILE"
    
    ../target/release/ai-cli-agent edit --api-key "$OPENROUTER_API_KEY" --path "$HTML_FILE" "change the background of the game to a gradient color"
    
    if [ $? -eq 0 ]; then
        echo "✅ Gradient background edit test passed"
        
        # Show the modified file
        echo "📄 Modified file content:"
        echo "========================"
        cat "$HTML_FILE"
        echo ""
        echo "========================"
    else
        echo "❌ Gradient background edit test failed"
    fi
else
    echo "❌ No HTML file found to edit"
fi

# Test 3: List directory contents
echo ""
echo "📋 Test 3: Listing directory contents..."
echo "======================================"

../target/release/ai-cli-agent list --api-key "$OPENROUTER_API_KEY"

echo "✅ Directory listing test passed"

# Test 4: Interactive mode (optional)
echo ""
echo "🔄 Test 4: Interactive mode test..."
echo "=================================="
echo "Note: This test requires manual interaction."
echo "You can test interactive mode by running:"
echo "  ../target/release/ai-cli-agent interactive --api-key \"$OPENROUTER_API_KEY\""
echo ""

# Cleanup
echo "🧹 Cleaning up..."
cd ..
rm -rf "$TEST_DIR"

echo "✅ Test completed"
echo ""
echo "🎉 AI CLI Agent test script finished!"
echo ""
echo "You can now use the AI CLI agent with:"
echo "  cargo run -- execute \"your task description\""
echo "  cargo run -- interactive"
echo ""
echo "Don't forget to set your OPENROUTER_API_KEY environment variable!"