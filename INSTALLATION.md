# Installation Guide for Code Maestro

## Prerequisites

Before starting, ensure you have the following installed:

### Windows
```powershell
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
# Download from https://nodejs.org/

# Install Visual Studio Build Tools (required for some dependencies)
```

### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
brew install node

# Install Xcode Command Line Tools
xcode-select --install
```

### Linux
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
sudo apt-get install nodejs npm

# Install build tools
sudo apt-get install build-essential
```

## Project Setup

### 1. Create Project Structure
```bash
mkdir -p code-maestro
cd code-maestro

# Create necessary directories
mkdir -p src/modules/{core,lsp,ai,editor,database}
mkdir -p src-tauri/src
mkdir -p data
mkdir -p tests
```

### 2. Initialize Cargo Project
```bash
cargo init --name code-maestro
```

### 3. Copy Configuration Files
Copy `Cargo.toml`, `package.json`, and `tauri.conf.json` to the project root.

### 4. Copy Source Files
Copy all `.rs` files to the `src/` directory:
- `main.rs` - Entry point
- `error.rs` - Error types
- `core_module.rs` - Core functionality
- `lsp_module.rs` - Language Server
- `ai_module.rs` - AI engine
- `database_module.rs` - Database
- `editor_module.rs` - Editor integration

### 5. Install Dependencies
```bash
# Rust dependencies (automatic via Cargo.toml)
cargo build

# Node.js dependencies (for frontend)
npm install
```

## Building

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Tauri App Build
```bash
# Build desktop app
cargo tauri build

# Run in development
cargo tauri dev
```

## Running Tests
```bash
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Project Structure

```
code-maestro/
├── src/
│   ├── main.rs              # Entry point
│   ├── error.rs             # Error handling
│   ├── core_module.rs       # Core functionality
│   ├── lsp_module.rs        # Language Server
│   ├── ai_module.rs         # AI engine
│   ├── database_module.rs   # Database layer
│   └── editor_module.rs     # Editor integration
├── src-tauri/               # Tauri desktop app
├── tests/                   # Integration tests
├── Cargo.toml              # Rust dependencies
├── package.json            # Node.js dependencies
├── tauri.conf.json         # Tauri configuration
└── README.md               # Documentation
```

## Configuration

### Environment Variables
```bash
# API Key for AI engine
export MAESTRO_API_KEY=your-key-here

# Database path
export MAESTRO_DB_PATH=./data/maestro.db

# Log level
export RUST_LOG=debug
```

## Troubleshooting

### Build Issues
- Clear build cache: `cargo clean`
- Update Rust: `rustup update`
- Check dependencies: `cargo tree`

### Runtime Issues
- Enable logging: `RUST_LOG=debug cargo run`
- Check database: `sqlite3 ./data/maestro.db`
- Verify API key is set

## Next Steps

1. Complete the project structure by copying all source files
2. Run `cargo build` to compile
3. Run `cargo test` to verify everything works
4. Start implementing Phase 2 features

For more information, see `README.md` and documentation in `docs/`.
