# acvp-cli Documentation

Welcome to the acvp-cli documentation!

This documentation is built with [mdBook](https://rust-lang.github.io/mdBook/).

## Viewing the Documentation

### Online

Visit [https://yourdomain.com/acvp-cli-docs](https://yourdomain.com/acvp-cli-docs) (when deployed)

### Local Development

1. **Install mdBook**:
   ```bash
   cargo install mdbook
   ```

2. **Serve locally**:
   ```bash
   cd docs
   mdbook serve --open
   ```

   This will:
   - Build the book
   - Start a local server
   - Open in your browser
   - Auto-reload on changes

3. **Build static HTML**:
   ```bash
   mdbook build
   ```

   Output in `docs/book/`

## Structure

```
docs/
├── book.toml              # Configuration
├── src/                   # Source files
│   ├── SUMMARY.md        # Table of contents
│   ├── README.md         # Introduction
│   ├── user-guide/       # User documentation
│   ├── reference/        # Technical reference
│   ├── development/      # Developer guides
│   └── appendix/         # Additional info
└── book/                  # Built output (generated)
```

## Contributing to Docs

### Adding a New Page

1. Create markdown file in appropriate directory:
   ```bash
   touch src/user-guide/new-page.md
   ```

2. Add to `SUMMARY.md`:
   ```markdown
   - [New Page](user-guide/new-page.md)
   ```

3. Write content using markdown

4. Preview with `mdbook serve`

### Linking Between Pages

Use relative links:

```markdown
See [Installation Guide](installation.md) for details.
```

From different sections:

```markdown
See [Technical Architecture](../reference/technical.md)
```

### Including Code

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

With syntax highlighting:

````markdown
```bash
cargo build --release
```
````

### Admonitions

For important notes:

```markdown
> **Note**: This is important information.

> **Warning**: Be careful here!
```

## Building for Production

```bash
# Clean build
mdbook clean
mdbook build

# Output is in book/
```

Deploy `book/` directory to web server.

## Deployment

### GitHub Pages

Add to `.github/workflows/docs.yml`:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup mdBook
        uses: peaceiris/actions-mdbook@v1
      - name: Build
        run: |
          cd acvp-rust/docs
          mdbook build
      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./acvp-rust/docs/book
```

### Gitea Pages

Similar workflow for Gitea Actions.

### Static Hosting

Upload `book/` directory to:
- Netlify
- Vercel
- AWS S3
- Any static host

## Maintenance

### Checking Links

```bash
mdbook test
```

### Spell Check

Use a spell checker on markdown files:

```bash
aspell -c src/user-guide/quick-start.md
```

### Format Markdown

Use prettier or similar:

```bash
prettier --write "src/**/*.md"
```

## Resources

- [mdBook User Guide](https://rust-lang.github.io/mdBook/)
- [Markdown Guide](https://www.markdownguide.org/)
- [Rust Documentation](https://doc.rust-lang.org/)
