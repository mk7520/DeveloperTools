# Contributing to Code Maestro

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Git

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/code-maestro.git
cd code-maestro

# Install dependencies
cargo build
npm install

# Set up environment
cp .env.example .env
```

## Development Guidelines

### Code Style

#### Rust
- Follow Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write tests for new features

```bash
cargo fmt
cargo clippy
cargo test
```

#### JavaScript/React
- Use ESLint for linting
- Use Prettier for formatting
- Follow React best practices
- Write component tests

```bash
npm run lint
npm run format
npm run test
```

### Commit Messages

Format: `type(scope): description`

Examples:
- `feat(ai): add code completion suggestions`
- `fix(editor): resolve syntax highlighting issue`
- `docs(readme): update installation steps`
- `refactor(db): optimize query performance`
- `test(lsp): add language server tests`

Types:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `refactor` - Code refactoring
- `test` - Adding tests
- `chore` - Maintenance

### Branch Naming

Format: `type/description`

Examples:
- `feat/code-completion`
- `fix/editor-bug`
- `docs/api-guide`

## Making Changes

### 1. Create Feature Branch
```bash
git checkout -b feat/my-feature
```

### 2. Make Changes
- Keep commits atomic
- Add tests for new features
- Update documentation

### 3. Test Locally
```bash
# Backend tests
cargo test

# Frontend tests
npm run test

# Full integration test
cargo tauri dev
```

### 4. Commit Changes
```bash
git add .
git commit -m "feat(module): description"
```

### 5. Push and Create PR
```bash
git push origin feat/my-feature
```

## Testing

### Unit Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Frontend Tests
```bash
npm run test
npm run test:watch
```

## Documentation

### Code Comments
```rust
/// Brief description
/// 
/// Longer explanation if needed
/// 
/// # Example
/// ```
/// let result = function();
/// ```
pub fn function() {
    // Implementation
}
```

### Markdown Documentation
- Keep README.md updated
- Document new features in ARCHITECTURE.md
- Add usage examples
- Include code snippets

## Pull Request Process

1. **Before Submitting**
   - Run `cargo fmt`
   - Run `cargo clippy`
   - Run `cargo test`
   - Update documentation
   - Add tests for new features

2. **PR Description**
   - Describe changes clearly
   - Reference related issues (#123)
   - Include before/after examples
   - List any breaking changes

3. **Review Process**
   - Address review comments
   - Re-test after changes
   - Maintain conversation in PR

4. **Merging**
   - Squash commits if requested
   - Delete feature branch
   - Close related issues

## Common Tasks

### Adding a New Feature

1. **Create feature branch**
   ```bash
   git checkout -b feat/new-feature
   ```

2. **Implement feature**
   - Add code to appropriate module
   - Write tests
   - Update documentation

3. **Test thoroughly**
   ```bash
   cargo test
   cargo clippy
   npm run test
   ```

4. **Create PR**
   - Describe changes
   - Reference issues
   - Request reviewers

### Fixing a Bug

1. **Create bug branch**
   ```bash
   git checkout -b fix/bug-name
   ```

2. **Write failing test**
   ```rust
   #[test]
   fn test_bug() {
       // Test that reproduces bug
   }
   ```

3. **Fix the bug**
   - Implement fix
   - Ensure test passes

4. **Verify fix**
   ```bash
   cargo test
   ```

### Adding Documentation

1. **Update relevant files**
   - README.md
   - ARCHITECTURE.md
   - API documentation
   - Code comments

2. **Check formatting**
   - Use markdown formatting
   - Include code examples
   - Add tables if needed

3. **Review and submit**
   - Proofread content
   - Create PR
   - Address feedback

## Building for Release

```bash
# Create release build
cargo build --release

# Create desktop package
cargo tauri build

# Create distribution
npm run build
```

## Getting Help

- Check existing issues and PRs
- Read documentation
- Ask questions in discussions
- Review similar implementations

## Code of Conduct

- Be respectful and inclusive
- Focus on the code, not the person
- Help others learn and grow
- Report issues to maintainers

## License

All contributions are licensed under MIT License.

---

Thank you for contributing to Code Maestro! 🚀
