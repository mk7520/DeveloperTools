#!/bin/bash
# Setup script for Code Maestro project
# Run this to initialize the complete project structure

set -e

echo "Creating Code Maestro project structure..."

# Create directories
mkdir -p src/modules/{core,lsp,ai,editor,database}
mkdir -p src-tauri/src/{components,pages,utils}
mkdir -p tests/unit
mkdir -p data
mkdir -p .github/workflows
mkdir -p docs

echo "Directories created successfully!"
echo ""
echo "Next steps:"
echo "1. Run: cargo init --name code-maestro"
echo "2. Copy files from this project"
echo "3. Run: cargo build --release"
echo ""
