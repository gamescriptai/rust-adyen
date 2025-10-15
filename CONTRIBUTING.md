# Contributing to Adyen Rust Library

Thank you for your interest in contributing to the Adyen Rust library! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.82.0 or later
- Git
- A GitHub account

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/gamescriptai/rust-adyen.git
   cd rust-adyen
   ```

2. **Build the Project**
   ```bash
   cargo build --all-features --workspace
   ```

3. **Run Tests**
   ```bash
   cargo test --all-features --workspace
   ```

4. **Check Code Quality**
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features
   ```

## 📋 How to Contribute

### Reporting Issues

- Use the [GitHub Issues](https://github.com/gamescriptai/rust-adyen/issues) page
- Search existing issues first to avoid duplicates
- Include minimal reproduction code for bugs
- Use clear, descriptive titles

### Feature Requests

- Check the [roadmap section](./README.md#-roadmap) in the README first
- Open an issue with the "enhancement" label
- Describe the use case and expected behavior
- Consider implementation complexity and breaking changes

### Pull Requests

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Follow the coding standards below
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   cargo test --all-features --workspace
   cargo fmt
   cargo clippy --all-targets --all-features
   ```

4. **Commit Your Changes**
   ```bash
   git commit -m "feat: add new payment method support"
   ```
   Use [conventional commits](#commit-message-format) format.

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

## 💻 Coding Standards

### Rust Code Style

- **Formatting**: Use `cargo fmt` with the project's `rustfmt.toml`
- **Linting**: All `cargo clippy` warnings must be addressed
- **Documentation**: All public items must have doc comments with examples
- **Testing**: Minimum 95% test coverage for new code

### Modern Rust Patterns (Required)

- **Builder Pattern**: Use for complex request types with `.build()` validation
- **Type State Pattern**: Prevent invalid states at compile time
- **Error Handling**: Use `thiserror` for structured errors, never `anyhow` in public APIs
- **Async/Await**: All I/O operations must be async
- **Zero-Copy**: Support both `serde` and `rkyv` serialization

### API Design Guidelines

1. **Type Safety First**
   - Use newtype patterns for domain concepts
   - Prefer compile-time over runtime validation
   - Make invalid states unrepresentable

2. **Ergonomics**
   - Implement `From`/`Into` conversions between related types
   - Use method chaining for fluent APIs
   - Provide both borrowed and owned variants where appropriate

3. **Performance**
   - Use `Box<str>` instead of `String` for immutable strings
   - Use `SmallVec` for typically small collections
   - Mark functions `const` where possible

### Documentation Standards

- **Examples**: Every public API must have working examples
- **Error Documentation**: Document all possible error conditions
- **Module Documentation**: Include overview and usage patterns

Example:
```rust
/// Represents a monetary amount with currency.
///
/// This type stores amounts in minor units (e.g., cents for USD/EUR) to avoid
/// floating-point precision issues. All operations maintain precision and
/// currency safety.
///
/// # Examples
///
/// ```rust
/// use adyen_core::{Amount, Currency};
///
/// // Create from major units (e.g., dollars)
/// let amount = Amount::from_major_units(100, Currency::USD);
/// assert_eq!(amount.minor_units(), 10000); // 100 dollars = 10000 cents
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount { /* ... */ }
```

## 🧪 Testing Guidelines

### Test Categories

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test API interactions with mock responses
3. **Doc Tests**: Ensure documentation examples work
4. **Property Tests**: Use `proptest` for complex data validation

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_from_major_units() {
        let amount = Amount::from_major_units(100, Currency::USD);
        assert_eq!(amount.minor_units(), 10000);
        assert_eq!(amount.currency(), Currency::USD);
    }

    #[test]
    fn test_amount_validation() {
        assert!(Amount::new(Decimal::from(-1), Currency::USD).is_err());
    }
}
```

### Running Tests

```bash
# All tests
cargo test --all-features --workspace

# Specific crate
cargo test -p adyen-core

# With coverage
cargo llvm-cov --all-features --workspace

# Documentation tests
cargo test --doc
```

## 📖 Documentation

### Generating Documentation

```bash
# Generate docs
cargo doc --all-features --workspace --no-deps

# Open in browser
cargo doc --all-features --workspace --no-deps --open
```

### Writing Documentation

- Use proper Rust doc comment format (`///`)
- Include code examples that compile and run
- Link to related types with backticks: `` `Amount` ``
- Use sections: `# Examples`, `# Errors`, `# Panics`

## 🔄 Commit Message Format

We use [Conventional Commits](https://conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code formatting (no logic changes)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```bash
feat: add checkout payment methods API
fix: resolve currency conversion edge case
docs: update API examples in README
test: add integration tests for webhooks
refactor: improve error handling patterns
```

## 🏗️ Implementation Phases

Check the [roadmap section](./README.md#-roadmap) in the README for the current implementation roadmap. When contributing:

1. **Check Current Phase**: Ensure your contribution aligns with the current development phase
2. **Update Progress**: Mark completed tasks in the roadmap
3. **Follow Patterns**: Use established patterns from completed modules

### Priority Areas

1. **High Priority**: Checkout API, Payments API, Webhooks
2. **Medium Priority**: Management API, Platform APIs
3. **Low Priority**: Specialized APIs (Disputes, BIN Lookup)

## 🐛 Debugging

### Common Issues

1. **Compilation Errors**
   - Check feature flags are correctly set
   - Ensure all dependencies are compatible

2. **Test Failures**
   - Run tests individually to isolate issues
   - Check for timing issues in async tests

3. **Clippy Warnings**
   - Address all warnings before submitting
   - Use `#[allow(...)]` sparingly with justification

### Debug Tools

```bash
# Verbose compilation
cargo build --verbose

# Expanded macros
cargo expand

# Dependency tree
cargo tree

# Audit dependencies
cargo audit
```

## 🔒 Security

### Reporting Security Issues

- **DO NOT** file public issues for security vulnerabilities
- Email security issues to: [support@gamescript.ai]
- Include detailed reproduction steps

### Security Guidelines

- Never commit secrets or API keys
- Use secure credential handling patterns
- Validate all user inputs
- Follow HTTPS-only practices

## 📜 Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help maintain a welcoming environment
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## 🆘 Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: File a GitHub Issue
- **Development**: Join our development discussions
- **Documentation**: Check the [generated docs](https://docs.rs/adyen)

## 📋 Review Process

### Pull Request Reviews

1. **Automated Checks**: CI must pass (tests, formatting, linting)
2. **Code Review**: At least one maintainer review required
3. **Documentation**: Ensure docs are updated for public API changes
4. **Testing**: New functionality must include comprehensive tests

### Merge Requirements

- All CI checks passing
- Approved by at least one maintainer
- Documentation updated
- No merge conflicts
- Follows conventional commit format

Thank you for contributing to making Rust payment processing better! 🦀💳