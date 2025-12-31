# Contributing to Stream Clipper

First off, thank you for considering contributing to Stream Clipper! 🎉

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Pull Request Process](#pull-request-process)
- [Style Guidelines](#style-guidelines)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

## Code of Conduct

This project and everyone participating in it is governed by our commitment to creating a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** 18 or higher
- **Rust** 1.70 or higher
- **Tauri CLI** (`cargo install tauri-cli`)

### Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stream-clipper.git
   cd stream-clipper
   ```

3. **Install dependencies**:
   ```bash
   npm install
   ```

4. **Run in development mode**:
   ```bash
   npm run tauri dev
   ```

5. **Build for production**:
   ```bash
   npm run tauri build
   ```

## Making Changes

### Branch Naming

Create a branch with a descriptive name:

- `feature/add-batch-processing`
- `fix/audio-detection-crash`
- `docs/update-readme`
- `refactor/improve-scoring-algorithm`

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation only
- `style` - Code style (formatting, semicolons, etc.)
- `refactor` - Code change that neither fixes a bug nor adds a feature
- `perf` - Performance improvement
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

**Examples:**
```
feat(audio): add voice activity detection
fix(export): resolve crash when exporting long videos
docs(readme): add installation instructions for Linux
```

## Pull Request Process

1. **Update documentation** if you're changing functionality
2. **Add tests** for new features when applicable
3. **Run linting and tests**:
   ```bash
   # Frontend
   npm run check
   
   # Backend
   cd src-tauri
   cargo clippy
   cargo test
   ```
4. **Update CHANGELOG.md** with your changes
5. **Create a Pull Request** with a clear description

### PR Title Format

Use the same format as commit messages:
```
feat(audio): add voice activity detection
```

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How has this been tested?

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review
- [ ] I have added tests (if applicable)
- [ ] I have updated documentation (if applicable)
```

## Style Guidelines

### Rust

- Follow `rustfmt` formatting
- Address all `clippy` warnings
- Use meaningful variable names
- Add doc comments for public functions

```rust
/// Analyzes audio file and detects volume spikes
/// 
/// # Arguments
/// * `path` - Path to the WAV file
/// * `settings` - Analysis settings
/// 
/// # Returns
/// Analysis result with detected spikes
pub fn analyze_audio(path: &str, settings: &Settings) -> Result<AnalysisResult, Error> {
    // Implementation
}
```

### TypeScript/Svelte

- Follow ESLint and Prettier configurations
- Use TypeScript strict mode
- Prefer `const` over `let`
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`)

```typescript
// Good
const settings = $derived($settingsStore);

// Avoid
let settings;
$: settings = $settingsStore;
```

### CSS

- Use CSS variables for theming
- Follow BEM-like naming for classes
- Keep styles scoped to components

```css
.clip-card {
  /* Component styles */
}

.clip-card__title {
  /* Element styles */
}

.clip-card--selected {
  /* Modifier styles */
}
```

## Reporting Bugs

### Before Submitting

1. Check existing issues to avoid duplicates
2. Try to reproduce with the latest version
3. Collect relevant information (OS, version, logs)

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g., Windows 11]
- Version: [e.g., 0.1.0]
- Video format: [e.g., MP4]

**Additional context**
Any other relevant information.
```

## Suggesting Features

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
What you want to happen.

**Describe alternatives you've considered**
Other solutions you've thought about.

**Additional context**
Any other information or screenshots.
```

## Project Structure

```
stream-clipper/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Svelte stores
│   │   ├── types/          # TypeScript types
│   │   └── utils/          # Utility functions
│   └── routes/             # SvelteKit routes
├── src-tauri/              # Rust backend
│   └── src/
│       ├── audio/          # Audio analysis
│       ├── chat/           # Chat parsing
│       ├── commands/       # Tauri commands
│       ├── highlight/      # Highlight scoring
│       ├── license/        # License validation
│       ├── video/          # Video processing
│       └── utils/          # Utilities
└── static/                 # Static assets
```

## Questions?

Feel free to open a discussion or reach out if you have any questions!

---

Thank you for contributing! 🙏
