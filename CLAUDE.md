# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **Polza AI Model Browser** - a single-page web application built with Dioxus that fetches and displays AI models from the Polza AI API with real-time filtering. The entire application logic is contained in `src/main.rs`.

## Development Commands

### Running the Application

```bash
# Start development server (defaults to web platform)
dx serve

# Run for specific platforms
dx serve --platform web
dx serve --platform desktop
```

### Building

```bash
# Production build
dx build --release
```

### Code Quality

```bash
# Run clippy with project-specific configuration
cargo clippy

# Check formatting
cargo fmt --check

# Format code
cargo fmt
```

### Running Tests

Due to the default `wasm32-unknown-unknown` target, tests must be run with an explicit native target:

```bash
# Run tests on native platform
cargo test --target x86_64-unknown-linux-gnu

# Or for other platforms, use your native target (find with: rustc -vV | grep host)
cargo test --target $(rustc -vV | grep host | cut -d' ' -f2)
```

**Why this is needed**: The default build target is `wasm32-unknown-unknown` (see Build Target Configuration below), but test binaries cannot execute directly as WebAssembly. By specifying the native target explicitly, cargo compiles and runs tests on your local platform.

**Optional**: Add an alias to `.cargo/config.toml` for convenience:

```toml
[alias]
test-native = "test --target x86_64-unknown-linux-gnu"
```

Then simply run `cargo test-native`.

## Deployment

### GitHub Pages Deployment

The project is configured for automatic deployment to GitHub Pages via GitHub Actions.

#### Automatic Deployment

Every push to the `master` branch automatically triggers a deployment workflow:

1. **Build Process**: The workflow compiles the application to WebAssembly
2. **base_path Configuration**: Automatically injects the repository name as base_path for proper routing
3. **Client-Side Routing**: Creates 404.html for GitHub Pages SPA support
4. **Deployment**: Publishes to the `gh-pages` branch

**Live URL**: `https://mexus.github.io/Polza-AI-models-viewer/`

#### Workflow Details

The deployment workflow (`.github/workflows/deploy.yml`) performs:
- Caches Rust toolchain and cargo dependencies for faster builds
- Dynamically configures `base_path` in `Dioxus.toml` based on repository name
- Runs `dx bundle --release` to create optimized production build
- Creates `404.html` copy for client-side routing support
- Deploys to `gh-pages` branch using `peaceiris/actions-gh-pages@v4`

#### Manual Deployment

You can also trigger deployment manually:

```bash
# Via GitHub UI: Actions tab → Deploy to GitHub Pages → Run workflow

# Or push to master branch:
git push origin master
```

#### Local vs CI base_path Handling

**Problem**: GitHub Pages subdirectory deployments require `base_path` configuration, but this breaks local `dx serve`.

**Solution**: The workflow dynamically injects base_path only during CI:
- **Local development**: No base_path in `Dioxus.toml` - serves from root (http://localhost:8080/)
- **CI deployment**: Workflow adds `base_path = "Polza-AI-models-viewer"` automatically
- **Result**: Both environments work correctly without manual configuration switching

#### First-Time Setup

After the first workflow run, configure GitHub Pages settings:

1. Go to repository **Settings** → **Pages**
2. Under **Source**, select **Deploy from a branch**
3. Choose **gh-pages** branch and **/ (root)** directory
4. Click **Save**

The site will be available at the URL shown in the Pages settings (usually within a few minutes).

#### Troubleshooting

**Workflow fails with permission error:**
- The workflow includes `permissions: contents: write` which should be sufficient
- If issues persist, check repository Settings → Actions → General → Workflow permissions

**Site shows 404 or broken routes:**
- Verify base_path is correctly injected (check workflow logs)
- Ensure 404.html was created (check deployment artifacts)
- Confirm GitHub Pages is serving from gh-pages branch

**Local development broken:**
- Ensure `Dioxus.toml` does NOT have base_path uncommented locally
- The workflow modifies it only during CI - your local file should remain unchanged

## Build Target Configuration

This project is primarily a web application targeting `wasm32-unknown-unknown`. To ensure consistency between development tools and the actual build:

### Default Target Setup

The `.cargo/config.toml` file sets `wasm32-unknown-unknown` as the default build target:

```toml
[build]
target = "wasm32-unknown-unknown"
```

**Why this matters**: Dioxus signals have different mutability requirements between native and wasm32 targets. Without this configuration, `cargo check` runs on your native target and may produce false warnings about unused mutability that would break `dx build --platform web`.

### IDE Configuration (Zed)

For rust-analyzer in Zed editor, the `.zed/settings.json` file configures the LSP to use the wasm32 target:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "wasm32-unknown-unknown"
        }
      }
    }
  }
}
```

This ensures that rust-analyzer's code analysis matches the actual build configuration.

### Other IDEs

For VS Code, add to `.vscode/settings.json`:
```json
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown"
}
```

For other editors, consult their rust-analyzer configuration documentation.

## Architecture

### Single-File Application Structure

The entire application is in `src/main.rs` and organized as follows:

1. **Data Models** (lines 11-50): Structs for API response deserialization
   - `ApiResponse`, `Model`, `Architecture`, `Pricing`
   - Uses `rust_decimal::Decimal` for precise pricing values
   - `Pricing::is_empty()` method filters out models with zero pricing

2. **Main Component** (lines 65-269): Single `App` component containing
   - State management with `use_signal` (filter text) and `use_resource` (API data)
   - Inline CSS styling (no external stylesheets)
   - Three UI states: Loading, Error (with retry), Success (with filtered results)

3. **API Integration**: Fetches from `https://api.polza.ai/api/v1/models`
   - Uses `reqwest` for async HTTP
   - Filters models with empty pricing on fetch (line 77)
   - Case-insensitive filtering by model name (line 177)

### Platform Support

The project uses Cargo features for cross-platform support:
- `web` (default): WebAssembly target
- `desktop`: Native desktop application
- `mobile`: Mobile platforms

Custom build profiles for optimized WASM development (`wasm-dev`) are configured in `Cargo.toml`.

### Clippy Configuration

The `clippy.toml` file includes critical Dioxus-specific lints:
- **Never hold Dioxus signal borrows across await points**
- Violating this will cause runtime panics when reads/writes fail
- Applies to: `GenerationalRef`, `GenerationalRefMut`, `dioxus_signals::Write`

## Key Constraints

- **Non-snake-case allowed** (`#![allow(non_snake_case)]`): Required for Dioxus component naming conventions
- **Single-file architecture**: All code lives in `main.rs` - this is intentional for this educational project
- **Test target**: Tests must be run with `--target x86_64-unknown-linux-gnu` (or your native target) due to the default wasm32 build target
