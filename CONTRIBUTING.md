# Contributing to Halt.rs

Thank you for your interest in contributing to Halt.rs! This document provides guidelines and information for contributors.

## Code of Conduct

This project follows a code of conduct to ensure a welcoming environment for all contributors. By participating, you agree to:

- Be respectful and inclusive
- Focus on constructive feedback
- Accept responsibility for mistakes
- Show empathy towards other contributors
- Help create a positive community

## Getting Started

### Prerequisites

- Rust 1.70+ (for core development)
- Node.js 18+ (for MCP server and TypeScript bindings)
- Python 3.8+ (for Python bindings)
- Java 17+ (for Java bindings)
- Go 1.21+ (for Go bindings)

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/halt-rs/halt.git
   cd halt
   ```

2. Build all components:
   ```bash
   ./scripts/build/build.sh all
   ```

3. Run tests:
   ```bash
   ./scripts/build/build.sh test
   ```

4. Start development server:
   ```bash
   cargo run -- serve
   ```

## Development Workflow

### 1. Choose an Issue

- Check the [issue tracker](https://github.com/halt-rs/halt/issues) for open issues
- Look for issues labeled `good first issue` or `help wanted`
- Comment on the issue to indicate you're working on it

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes

- Follow the existing code style
- Write tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add new feature

- Description of changes
- Related issue: #123"
```

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `refactor:` for code refactoring
- `test:` for test additions
- `chore:` for maintenance

### 5. Create Pull Request

- Push your branch to GitHub
- Create a pull request with a clear description
- Reference any related issues
- Request review from maintainers

## Development Guidelines

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Run `clippy` for linting
- Write comprehensive unit tests
- Use meaningful variable and function names
- Add documentation comments for public APIs

### TypeScript/JavaScript Code

- Use TypeScript for type safety
- Follow the existing ESLint configuration
- Write Jest tests for all functionality
- Use meaningful variable and function names
- Add JSDoc comments for public APIs

### Python Code

- Follow PEP 8 style guidelines
- Use type hints
- Write comprehensive unit tests with pytest
- Add docstrings for all public functions
- Use meaningful variable and function names

### Java Code

- Follow standard Java naming conventions
- Use meaningful class and method names
- Write JUnit tests
- Add JavaDoc comments
- Handle exceptions appropriately

### Go Code

- Follow standard Go formatting (gofmt)
- Use meaningful package and function names
- Write comprehensive tests
- Add comments for exported functions
- Handle errors appropriately

## Testing

### Running Tests

```bash
# Run all tests
./scripts/build/build.sh test

# Run specific test suites
cargo test                    # Rust tests
cd mcp-server && npm test     # MCP server tests
cd bindings/python && pytest  # Python tests
```

### Writing Tests

- Write tests for all new functionality
- Include both positive and negative test cases
- Test edge cases and error conditions
- Ensure tests are fast and reliable
- Use descriptive test names

### Test Coverage

- Aim for high test coverage (>80%)
- Use code coverage tools to identify gaps
- Review coverage reports regularly

## Documentation

### Code Documentation

- Add documentation comments to all public APIs
- Explain parameters, return values, and behavior
- Include code examples where helpful
- Keep documentation up to date with code changes

### User Documentation

- Update README.md for significant changes
- Add examples for new features
- Update API documentation
- Keep installation and usage instructions current

## Performance

- Profile code for performance bottlenecks
- Optimize hot paths
- Use appropriate data structures
- Consider memory usage
- Write benchmarks for critical functionality

## Security

- Be aware of security implications
- Use secure coding practices
- Validate input data
- Handle sensitive information appropriately
- Report security issues privately

## Release Process

### Version Numbering

Halt.rs follows semantic versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] Update version numbers in all relevant files
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Create git tag
- [ ] Publish to package registries
- [ ] Update release notes

## Communication

- Use GitHub issues for bug reports and feature requests
- Use GitHub discussions for general questions
- Join our Discord/Slack for real-time discussion
- Be responsive to feedback and reviews

## Recognition

Contributors are recognized in:
- GitHub contributor statistics
- CHANGELOG.md for significant contributions
- Release notes
- Project documentation

## Questions?

If you have questions about contributing:
- Check existing issues and documentation
- Ask in GitHub discussions
- Contact maintainers directly

Thank you for contributing to Halt.rs! 🚀
