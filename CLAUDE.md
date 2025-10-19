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

## Localization

### Overview

This application supports **multiple languages** and uses the **dioxus-i18n** library based on Mozilla's **Fluent Project** for internationalization.

**Currently Supported Languages:**
- English (`en-US`) - Default
- Russian (`ru-RU`) - Full translation

**Translation Files Location:**
- `locales/en-US.ftl` - English translations
- `locales/ru-RU.ftl` - Russian translations

**i18n Module:**
- `src/i18n/mod.rs` - Language detection, switching, and LocalStorage persistence

### CRITICAL: When Adding New Features

**ANY user-facing text MUST be localized.** This includes:

✅ **Always localize:**
- UI labels, buttons, and links
- Headings, titles, and subtitles
- Error messages and success notifications
- Loading states and empty states
- Placeholder text in inputs
- Tooltips and help text
- Modal titles and section headers
- Dynamic messages with variables

❌ **Do NOT localize:**
- API endpoints and URLs
- Technical identifiers (slugs, IDs)
- Code/configuration values
- Console log messages (development only)

### How to Add Translations

When adding new user-facing text, follow these steps:

#### 1. Add Translation Keys to ALL FTL Files

Add the same key to **both** `locales/en-US.ftl` AND `locales/ru-RU.ftl`:

**English** (`locales/en-US.ftl`):
```ftl
# Feature Name
feature-title = Feature Title
feature-description = This is a description
button-action = Click Me
```

**Russian** (`locales/ru-RU.ftl`):
```ftl
# Feature Name
feature-title = Название функции
feature-description = Это описание
button-action = Нажми меня
```

#### 2. Use the `t!` Macro in Rust Code

In your component file:

```rust
use dioxus::prelude::*;
use dioxus_i18n::t;  // Import the t! macro

#[component]
pub fn MyComponent() -> Element {
    rsx! {
        h2 { { t!("feature-title") } }
        p { { t!("feature-description") } }
        button { { t!("button-action") } }
    }
}
```

#### 3. Translations with Variables

For dynamic content, use Fluent's variable interpolation:

**In FTL files:**
```ftl
# English
items-found = Found {$count} item(s)
greeting = Hello, {$name}!

# Russian
items-found = Найдено {$count} элемент/элементов
greeting = Привет, {$name}!
```

**In Rust code:**
```rust
rsx! {
    div { { t!("items-found", count: items.len()) } }
    div { { t!("greeting", name: user_name) } }
}
```

#### 4. Translations in Input Placeholders

For placeholders and attributes:

```rust
rsx! {
    input {
        r#type: "text",
        placeholder: "{t!(\"search-placeholder\")}",
    }
}
```

### Translation Key Naming Conventions

Follow these conventions for consistency:

**Format:** `kebab-case` (lowercase with hyphens)

**Grouping by Component/Feature:**
- `app-*` - Main application (e.g., `app-title`, `app-subtitle`)
- `filter-*` - Filter controls (e.g., `filter-label`, `filter-placeholder`)
- `sort-*` - Sort controls (e.g., `sort-name`, `sort-ascending`)
- `modal-*` - Modal dialogs (e.g., `modal-title`, `modal-close`)
- `button-*` - Button labels (e.g., `button-submit`, `button-cancel`)
- `error-*` - Error messages (e.g., `error-network`, `error-not-found`)
- `label-*` - Form/field labels (e.g., `label-username`, `label-email`)

**Use Descriptive Names:**
- ✅ Good: `button-refresh`, `error-failed-load`, `label-created`
- ❌ Bad: `btn1`, `err`, `lbl`

**Indicate Purpose:**
- ✅ Good: `modal-label-provider` (clearly modal-specific)
- ❌ Bad: `provider` (too generic, unclear context)

### Examples from the Codebase

**Simple Translation:**
```rust
// In component
h1 { { t!("app-title") } }

// In locales/en-US.ftl
app-title = Polza AI Models

// In locales/ru-RU.ftl
app-title = Модели Polza AI
```

**Translation with Variable:**
```rust
// In component
div { { t!("models-found", count: models.len()) } }

// In locales/en-US.ftl
models-found = Found {$count} model(s)

// In locales/ru-RU.ftl
models-found = Найдено {$count} модель/моделей
```

**Conditional Translation:**
```rust
// In component
if is_loading {
    { t!("button-refreshing") }
} else {
    { t!("button-refresh") }
}

// In locales/en-US.ftl
button-refresh = 🔄 Refresh
button-refreshing = ⏳ Refreshing...

// In locales/ru-RU.ftl
button-refresh = 🔄 Обновить
button-refreshing = ⏳ Обновление...
```

### Testing Translations

After adding translations:

1. **Compile Check:**
   ```bash
   cargo check --target x86_64-unknown-linux-gnu
   ```
   Missing translation keys will cause compile errors.

2. **Test in Browser:**
   ```bash
   dx serve
   ```
   - Test in English (default)
   - Click the language switcher in footer (EN/РУ)
   - Verify Russian translations display correctly
   - Check that dynamic variables render properly

3. **Verify Both Languages:**
   - All new text should appear in both languages
   - Layout should work in both languages (Russian text is often longer)
   - Variables should interpolate correctly

### Important Reminders

🚨 **CRITICAL RULES:**

1. **ALWAYS update BOTH language files** when adding new keys
   - If you add a key to `en-US.ftl`, you MUST add it to `ru-RU.ftl`
   - Consistency is critical - missing keys will cause errors

2. **NEVER hardcode user-facing strings** in Rust code
   - ❌ Bad: `"Click here"`
   - ✅ Good: `{ t!("button-click-here") }`

3. **Check existing translations first**
   - Before creating a new key, search FTL files for similar translations
   - Reuse existing keys when appropriate (e.g., `button-copy`, `button-close`)

4. **Test after every change**
   - Run `dx serve` and manually test both languages
   - Ensure variables render correctly
   - Check for layout issues with longer Russian text

5. **Group related translations**
   - Keep translations for the same feature/component together in FTL files
   - Use comments to organize sections (e.g., `# Filter Controls`)

6. **Translation embedding**
   - Translations are embedded in WASM at compile time via `include_str!()`
   - Changes to FTL files require rebuilding the application
   - No runtime file loading - fully offline-capable

### Adding a New Language

To add support for a third language (e.g., Spanish):

1. Create `locales/es-ES.ftl` with all translation keys from `en-US.ftl`
2. Translate all values to Spanish
3. Update `src/i18n/mod.rs`:
   ```rust
   I18nConfig::new(initial_language)
       .with_locale(Locale::new_static(
           langid!("en-US"),
           include_str!("../../locales/en-US.ftl"),
       ))
       .with_locale(Locale::new_static(
           langid!("ru-RU"),
           include_str!("../../locales/ru-RU.ftl"),
       ))
       .with_locale(Locale::new_static(
           langid!("es-ES"),
           include_str!("../../locales/es-ES.ftl"),
       ))
   ```
4. Update `detect_language()` function to handle Spanish
5. Update language toggle logic to cycle through all three languages
6. Add language codes to FTL files (e.g., `language-code = ES`)

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

### Modular Application Structure

The application is organized into a modular structure with clear separation of concerns:

**Core Modules:**

1. **API Layer** (`src/api/`):
   - `client.rs` - API endpoint integration and data fetching
   - Fetches from `https://api.polza.ai/api/v1/models`
   - Filters models with empty pricing
   - Uses `reqwest` for async HTTP requests

2. **Data Models** (`src/models/`):
   - `api.rs` - API response types (`ApiResponse`, `Model`)
   - `architecture.rs` - Model architecture and modalities enum
   - `pricing.rs` - Pricing information with `Decimal` for precision
   - `ui.rs` - UI-specific types (sorting, modality filtering)
   - Uses `rust_decimal::Decimal` for precise pricing values

3. **Components** (`src/components/`):
   - `app.rs` - Main application component with i18n initialization
   - `filters.rs` - Text and modality filter controls
   - `sort_controls.rs` - Sort field and direction controls
   - `model_list.rs` - Model list container with count display
   - `model_card.rs` - Individual model display cards
   - `modal.rs` - Detailed model information modal
   - `styles.rs` - Global CSS styles

4. **Utilities** (`src/utils/`):
   - `tokenize.rs` - Intelligent text tokenization for filtering
   - `filter.rs` - Modality matching logic
   - `format.rs` - Date, decimal, and number formatting

5. **Caching** (`src/cache/`):
   - `wasm.rs` - localStorage-based cache for web (1-hour duration)
   - `native.rs` - Native cache implementation stub
   - Platform-specific caching implementations

6. **Internationalization** (`src/i18n/`):
   - `mod.rs` - i18n configuration, browser language detection
   - Language switching and LocalStorage persistence
   - Integration with Fluent-based translation system

**State Management:**
- Uses Dioxus signals (`use_signal`) for reactive state
- Resource loading with `use_resource` for async API data
- Separate signals for filters, sorting, modals, and UI feedback

**UI States:**
- Loading: Animated spinner while fetching data
- Error: User-friendly error message with retry functionality
- Success: Filtered and sorted model list with interactive controls

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
- **Test target**: Tests must be run with `--target x86_64-unknown-linux-gnu` (or your native target) due to the default wasm32 build target
- **Localization required**: All user-facing text must be localized in both `en-US.ftl` and `ru-RU.ftl` - never hardcode strings in Rust code
- **Translation files**: Changes to `.ftl` files require rebuilding as translations are embedded at compile time
