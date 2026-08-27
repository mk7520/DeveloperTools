# Quick Reference Guide - Code Maestro

## 🚀 Quick Start

```bash
# 1. Setup
cp .env.example .env
cargo build
npm install

# 2. Development (3 terminals)
Terminal 1: cargo run              # Rust backend
Terminal 2: npm run dev            # React frontend
Terminal 3: cargo tauri dev        # Desktop app
```

## 📁 Key Files

### Backend
| File | Purpose |
|------|---------|
| `main.rs` | Entry point |
| `error.rs` | Error types |
| `core_*.rs` | Core functionality |
| `lsp_module.rs` | Language Server |
| `ai_module.rs` | AI engine |
| `database_module.rs` | Database |

### Frontend
| File | Purpose |
|------|---------|
| `App.jsx` | Main component |
| `Sidebar.jsx` | File explorer |
| `Terminal.jsx` | Terminal |
| `api.js` | Backend calls |
| `App.css` | Styling |

### Config
| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust deps |
| `package.json` | Node deps |
| `tauri.conf.json` | Desktop config |
| `.env.example` | Env template |

## 🔧 Common Commands

### Rust
```bash
cargo build           # Build project
cargo run            # Run application
cargo test           # Run tests
cargo fmt            # Format code
cargo clippy         # Lint code
cargo clean          # Clean build
```

### Frontend
```bash
npm run dev          # Start dev server
npm run build        # Build for production
npm run test         # Run tests
npm run lint         # Lint code
npm install          # Install dependencies
```

### Tauri
```bash
cargo tauri dev      # Run desktop app dev
cargo tauri build    # Build desktop package
cargo tauri test     # Test desktop app
```

## 📚 Documentation Map

| Document | Content |
|----------|---------|
| `README.md` | Project overview |
| `INSTALLATION.md` | Setup instructions |
| `ARCHITECTURE.md` | System design |
| `CONTRIBUTING.md` | Dev guidelines |
| `PROJECT_SUMMARY.md` | Complete summary |

## 🎯 Development Workflow

### 1. Create Feature Branch
```bash
git checkout -b feat/feature-name
```

### 2. Make Changes
- Modify code
- Add tests
- Update docs

### 3. Test
```bash
cargo test
npm run test
```

### 4. Format & Lint
```bash
cargo fmt
cargo clippy
npm run lint
```

### 5. Commit
```bash
git commit -m "feat(module): description"
```

### 6. Push & PR
```bash
git push origin feat/feature-name
```

## 🐛 Troubleshooting

### Rust Build Issues
```bash
cargo clean          # Clear cache
rustup update        # Update Rust
cargo tree           # Check dependencies
```

### Frontend Issues
```bash
rm -rf node_modules  # Clear node_modules
npm install          # Reinstall
npm cache clean --force
```

### Database Issues
```bash
rm data/maestro.db   # Reset database
# Application will recreate on startup
```

### Port Conflicts
```bash
# Change port in vite.config.js (line 5)
port: 5174           # or another port

# Change API port in backend
// Update in main.rs listen address
```

## 📊 Module Overview

### Core Modules
- **Document** - File management
- **Syntax** - Code analysis
- **Config** - Settings
- **LSP** - Language Server Protocol
- **AI** - Suggestion engine
- **Editor** - State management
- **Database** - Persistence

## 🔐 Environment Setup

```bash
# Required
MAESTRO_API_KEY=your-key
AI_API_KEY=your-ai-key

# Optional (defaults provided)
MAESTRO_DB_PATH=./data/maestro.db
EDITOR_THEME=dark
EDITOR_FONT_SIZE=14
```

## 💡 Tips

1. **Use proper logging**
   ```rust
   tracing::info!("Message");
   tracing::error!("Error");
   ```

2. **Write tests alongside code**
   ```rust
   #[test]
   fn test_feature() { }
   ```

3. **Keep components small**
   - One responsibility per component
   - Props for configuration
   - State for local data

4. **API calls with error handling**
   ```javascript
   try {
     const data = await api.call();
   } catch (error) {
     console.error('API error:', error);
   }
   ```

## 📈 Performance Tips

1. **Database**
   - Use indexes for frequent queries
   - Batch operations when possible
   - Clean old cache entries

2. **Frontend**
   - Use React.memo for expensive components
   - Debounce API calls
   - Cache responses

3. **Backend**
   - Use async/await for I/O
   - Cache compiled code
   - Profile with cargo-flamegraph

## 🚨 Before Committing

- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Tests pass (`cargo test`)
- [ ] Frontend builds (`npm run build`)
- [ ] Documentation updated
- [ ] No console errors/warnings

## 📞 Getting Help

1. Check `CONTRIBUTING.md`
2. Review `ARCHITECTURE.md`
3. Search existing issues
4. Check code comments
5. Read documentation

## 🎓 Learning Resources

### Rust
- https://doc.rust-lang.org/book/
- https://tokio.rs/
- https://tower-rs.github.io/tower-lsp/

### React
- https://react.dev/
- https://vitejs.dev/
- https://microsoft.github.io/monaco-editor/

### Tauri
- https://tauri.app/v1/guides/
- https://docs.rs/tauri/

---

**For detailed information, see the full documentation files.**

Last Updated: 2024
