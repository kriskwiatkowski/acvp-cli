# Contributing

Thank you for your interest in contributing to acvp-cli!

## Ways to Contribute

- Report bugs
- Suggest features
- Improve documentation
- Submit code changes
- Write tests
- Review pull requests

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Test thoroughly
6. Submit a pull request

## Development Setup

```bash
git clone https://git.amongbytes.com/AmongBytes/acvp-cli
cd acvp-cli
cargo build
cargo test
```

## Code Style

### Formatting

Use `rustfmt`:

```bash
cargo fmt
```

### Linting

Use `clippy`:

```bash
cargo clippy -- -D warnings
```

### Before Committing

Run local CI:

```bash
./.gitea/local-ci.sh
```

## Pull Request Process

1. **Create Issue First** - Discuss changes before coding
2. **Branch Naming** - Use descriptive names (`feature/add-sha3`, `fix/clippy-warnings`)
3. **Commit Messages** - Clear and descriptive
4. **Tests** - Add tests for new features
5. **Documentation** - Update docs for user-facing changes
6. **CI** - Ensure all checks pass

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

Example:

```
feat(subprocess): Add support for ML-KEM-1024

Implements test vector processing for ML-KEM-1024 parameter set.
Includes unit tests and integration tests.

Closes #123
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_config_parsing
```

### Run with Output

```bash
cargo test -- --nocapture
```

### Add New Tests

Place tests in appropriate module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Documentation

### Code Documentation

Use doc comments:

```rust
/// Processes test vectors for hash algorithms.
///
/// # Arguments
///
/// * `subprocess` - The modulewrapper subprocess
/// * `vector_set` - JSON test vectors
///
/// # Returns
///
/// JSON response or error
///
/// # Example
///
/// ```
/// let response = process_hash(&mut subprocess, &vectors)?;
/// ```
pub fn process_hash(
    subprocess: &mut Subprocess,
    vector_set: &Value
) -> Result<Value> {
    // ...
}
```

### Generate Docs

```bash
cargo doc --open
```

## Adding New Algorithms

1. **Add Handler** in `src/subprocess/primitives.rs`:

```rust
pub fn process_new_algo(
    subprocess: &mut Subprocess,
    vector_set: &Value
) -> Result<Value> {
    // Implementation
}
```

1. **Register in Router** in `src/subprocess/mod.rs`:

```rust
match algorithm {
    "NEW-ALGO" => process_new_algo(self, vector_set),
    // ...
}
```

1. **Add Tests**:

```rust
#[test]
fn test_new_algo() {
    // Test implementation
}
```

1. **Update Documentation**:
   - Add to supported algorithms list
   - Add usage examples

## Code Review Guidelines

### As an Author

- Keep changes focused
- Write clear descriptions
- Respond to feedback promptly
- Update based on reviews

### As a Reviewer

- Be constructive
- Focus on code quality
- Test the changes
- Approve when ready

## Issue Reporting

### Bug Reports

Include:

- Description
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, Rust version)
- Logs/errors

Template:

```markdown
**Bug Description**
Clear description of the bug.

**To Reproduce**
1. Run command: `acvp-cli ...`
2. Observe error

**Expected**
What should happen.

**Actual**
What actually happens.

**Environment**
- OS: Ubuntu 22.04
- Rust: 1.70.0
- acvp-cli: 1.6.7

**Logs**
```

Error messages here

```
```

### Feature Requests

Include:

- Use case
- Proposed solution
- Alternatives considered
- Additional context

## Questions and Help

- Open a GitHub issue with "Question:" prefix
- Check existing issues and documentation first
- Provide context and examples

## License

By contributing, you agree that your contributions will be licensed under the ISC License.

## Code of Conduct

Be respectful and inclusive. We aim for a welcoming environment for all contributors.
