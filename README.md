# Polza AI Model Browser

A web application built with Dioxus that provides an interactive browser for AI models available through the Polza AI API. The application features intelligent real-time filtering, caching, responsive design, and elegant error handling.

**Live Demo**: [https://mexus.github.io/Polza-AI-models-viewer/](https://mexus.github.io/Polza-AI-models-viewer/)

## Features

### Core Functionality
- **Intelligent Real-time Filtering**: Advanced tokenization-based search that understands:
  - Multiple word separators (spaces, hyphens, underscores, dots, slashes)
  - Punctuation (parentheses, brackets, colons, commas, semicolons)
  - camelCase and PascalCase splitting
  - Case-insensitive matching
  - Consecutive token matching
- **Modality Filtering**: Interactive toggle-button filters for model capabilities:
  - Filter by input modalities (Text, Image, File, Audio, Embeddings)
  - Filter by output modalities (Text, Image, File, Audio, Embeddings)
  - AND logic: models must have ALL selected modalities
  - Works in combination with text search
- **Flexible Sorting**: Modern segmented control for sorting filtered results:
  - Sort by Name (alphabetical), Created Date (chronological), Prompt Price, or Completion Price
  - Toggle between ascending (↑) and descending (↓) order
  - Default: Prompt Price (High to Low)
- **API Integration**: Fetches live data from the Polza AI models endpoint
- **Smart Data Handling**: Automatically filters out models with empty pricing information
- **Performance Caching**: 1-hour localStorage cache to minimize API calls and improve load times

### User Interface
- **Modal Detail Views**: Click any model to see comprehensive details including:
  - Full pricing breakdown (prompt, completion, images, requests, caching)
  - Architecture details (input/output modalities)
  - Provider configuration (context length, max tokens, moderation status)
  - Supported parameters
- **Copy to Clipboard**: One-click copying of canonical model slugs
- **Loading States**: Smooth loading animations while fetching data
- **Error Recovery**: User-friendly error messages with retry functionality
- **Responsive Design**: Clean, modern UI that works across different screen sizes
- **Interactive UI**: Hover effects and smooth transitions for better user experience
- **Manual Refresh**: Clear cache and reload data with the refresh button

## Tech Stack

- **[Dioxus](https://dioxuslabs.com/)**: React-like framework for building cross-platform user interfaces in Rust
- **[reqwest](https://docs.rs/reqwest/)**: HTTP client for async API requests
- **[serde](https://serde.rs/)**: Serialization/deserialization of JSON data
- **[rust_decimal](https://docs.rs/rust_decimal/)**: Precise decimal handling for pricing information
- **[time](https://docs.rs/time/)**: Date/time handling and formatting
- **[gloo-storage](https://docs.rs/gloo-storage/)**: LocalStorage API for web caching (wasm32 only)
- **[gloo-console](https://docs.rs/gloo-console/)**: Console logging for browser debugging (wasm32 only)
- **[web-sys](https://docs.rs/web-sys/)**: Web APIs for clipboard functionality (wasm32 only)

## Project Structure

```
polza-models/
├─ assets/            # Static assets (images, fonts, etc.)
├─ src/
│  ├─ api/            # API client and data fetching
│  │  ├─ client.rs    # API endpoint integration
│  │  └─ mod.rs
│  ├─ cache/          # Platform-specific caching implementations
│  │  ├─ wasm.rs      # localStorage-based cache (web)
│  │  ├─ native.rs    # Native cache implementation
│  │  └─ mod.rs
│  ├─ components/     # UI components
│  │  ├─ app.rs       # Main application component
│  │  ├─ filters.rs   # Text and modality filter controls
│  │  ├─ modal.rs     # Model detail modal dialog
│  │  ├─ model_card.rs    # Individual model display card
│  │  ├─ model_list.rs    # Model list container
│  │  ├─ sort_controls.rs # Sort field and direction controls
│  │  ├─ styles.rs    # Global CSS styles
│  │  └─ mod.rs
│  ├─ models/         # Data models and type definitions
│  │  ├─ api.rs       # API response types
│  │  ├─ architecture.rs  # Model architecture and modalities
│  │  ├─ pricing.rs   # Pricing information
│  │  ├─ ui.rs        # UI-specific types (sorting, etc.)
│  │  └─ mod.rs
│  ├─ utils/          # Utility functions
│  │  ├─ tokenize.rs  # Text tokenization for intelligent filtering
│  │  ├─ filter.rs    # Filter logic (modality matching)
│  │  ├─ format.rs    # Formatting utilities (dates, decimals)
│  │  └─ mod.rs
│  ├─ lib.rs          # Library root
│  └─ main.rs         # Application entry point
├─ .cargo/
│  └─ config.toml     # Cargo configuration (wasm32 default target)
├─ clippy.toml        # Clippy lints configuration (Dioxus-specific)
├─ Cargo.toml         # Project dependencies and configuration
├─ CLAUDE.md          # Developer guide for AI assistants
└─ README.md          # This file
```

## API Integration

The application connects to the **Polza AI Models API**:
- **Endpoint**: `https://api.polza.ai/api/v1/models`
- **Response Format**: JSON array of model objects with pricing and architecture information
- **Modalities Supported**: Text, Image, File, Audio, Embeddings

Each model includes detailed pricing information for:
- Prompt tokens (per 1M tokens)
- Completion tokens (per 1M tokens)
- Image processing (per image)
- Per-request costs
- Web search functionality
- Internal reasoning (per 1M tokens)
- Input cache read/write (per 1M tokens)

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started): `cargo install dioxus-cli`

### Running the Application

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform` flag:
```bash
dx serve --platform web      # WebAssembly (default)
dx serve --platform desktop  # Native desktop app
```

### Building for Production

```bash
# Production build for web (optimized WASM)
dx build --release

# Production build for desktop
dx build --release --platform desktop
```

### Testing

The project includes comprehensive unit tests for the tokenization and filtering logic (21 tests total).

**Important**: Due to the default `wasm32-unknown-unknown` target, tests must be run with an explicit native target:

```bash
# Run tests on Linux
cargo test --target x86_64-unknown-linux-gnu

# Run tests on macOS
cargo test --target x86_64-apple-darwin

# Or automatically use your native target
cargo test --target $(rustc -vV | grep host | cut -d' ' -f2)
```

**Why this is needed**: The project defaults to the `wasm32-unknown-unknown` target for consistent IDE/build behavior. However, test binaries cannot execute as WebAssembly without a runtime, so they must be compiled for your native platform.

**Test Coverage**:
- `tokenize()` function: Basic tokenization, delimiters, camelCase splitting, edge cases, Unicode support
- `matches_any_token_sequence()` function: Single token matching, consecutive token matching, prefix matching
- `has_all_modalities()` function: Empty filters, single/multiple modality requirements, edge cases
- Integration tests: Real-world filtering scenarios including the parentheses bug fix

### Code Quality

```bash
# Run Clippy (includes Dioxus-specific lints)
cargo clippy

# Check code formatting
cargo fmt --check

# Auto-format code
cargo fmt
```

The project uses a `clippy.toml` configuration with important Dioxus-specific lints:
- Never hold Dioxus signal borrows across await points (prevents runtime panics)

## Advanced Features

### Intelligent Filtering System

The application uses a sophisticated tokenization system that makes searching intuitive and flexible:

**Supported Delimiters**:
- Whitespace (spaces, tabs, newlines)
- Punctuation: `-`, `_`, `.`, `/`
- Brackets: `(`, `)`, `[`, `]`, `{`, `}`
- Other: `:`, `,`, `;`

**camelCase Splitting**:
- `XMLHttpRequest` → `["xml", "http", "request"]`
- `PascalCase` → `["pascal", "case"]`

**Example**: Searching for `"nano"` will match:
- `"Nano Banana"`
- `"(Nano Edition)"`
- `"Model-nano-v2"`
- `"nanoGPT"`

**Multi-word Filters**: All search terms must match (AND logic)
- Search: `"google flash"` → Matches: `"Google: Gemini 2.5 Flash"` ✓
- Search: `"google claude"` → Matches: `"Google: Gemini 2.5 Flash"` ✗

### Modality Filtering

The application provides powerful filtering based on model input and output capabilities:

**Available Modalities**:
- **Text**: Traditional text-based input/output
- **Image**: Image processing and generation
- **File**: File handling capabilities
- **Audio**: Audio processing and generation
- **Embeddings**: Vector embeddings for semantic search

**Filter Behavior**:
- **Separate Filters**: Independent controls for input and output modalities
- **AND Logic**: Models must have ALL selected modalities in each category
- **Visual Feedback**: Toggle buttons use color-coding matching the modality badges:
  - Text: Blue
  - Image: Purple
  - File: Orange
  - Audio: Red
  - Embeddings: Teal
- **Combined with Text Search**: All filters (text + input + output) work together using AND logic

**Example Use Cases**:
1. Find models that accept both text and images as input:
   - Select "Text" and "Image" under Input Modalities

2. Find models that can output both text and images:
   - Select "Text" and "Image" under Output Modalities

3. Find multimodal models with specific capabilities:
   - Text search: "gemini"
   - Input: "Text" + "Image"
   - Output: "Text" + "Image"
   - Result: Only Gemini models that accept and produce both text and images

### Sorting

The application provides flexible sorting controls to organize filtered results:

**Sort Fields**:
- **Name**: Alphabetical order (A-Z ascending, Z-A descending)
- **Created**: Chronological order by model creation date (newest first descending, oldest first ascending)
- **Prompt Price**: Cost per million input tokens (highest/lowest first)
- **Completion Price**: Cost per million output tokens (highest/lowest first)

**User Interface**:
- **Segmented Control**: Modern button group showing all four sort options
- **Direction Toggle**: Separate button to switch between ascending (↑) and descending (↓) order
- **Visual Feedback**: Active sort field highlighted in blue, clear indication of current direction
- **Default Sort**: Prompt Price (High to Low) - shows premium/most capable models first

**Behavior**:
- Sorting applies to filtered results only
- Sort settings persist across filter changes
- Combines seamlessly with text search and modality filters

**Example Use Cases**:
1. Find cheapest models for input processing:
   - Sort by "Prompt Price" + "↑ Ascending"

2. Browse newest models first:
   - Sort by "Created" + "↓ Descending"

3. Find models alphabetically:
   - Sort by "Name" + "↑ Ascending"

### Caching Strategy

- **Duration**: 1 hour (3600 seconds)
- **Storage**: Browser localStorage (web platform only)
- **Invalidation**: Manual refresh button or expired cache
- **Benefits**: Faster load times, reduced API calls, offline-like experience

## Build Target Configuration

This project uses `wasm32-unknown-unknown` as the default build target (configured in `.cargo/config.toml`). This ensures:

1. **Consistency**: `cargo check`, `cargo clippy`, and IDE analysis use the same target as `dx build`
2. **Correct Linting**: Dioxus signals have different mutability requirements on wasm32 vs native targets
3. **IDE Alignment**: rust-analyzer provides accurate diagnostics matching the actual build

**Trade-off**: Tests require explicit native target specification (see Testing section above).

For detailed configuration instructions for different IDEs, see `CLAUDE.md`.

## Platform Support

The project supports multiple platforms through Cargo features:

- `web` (default): WebAssembly for browsers
  - Includes localStorage caching
  - Includes clipboard API support
  - Locale-aware time formatting
- `desktop`: Native desktop application
  - Falls back to RFC2822 time formatting
  - No caching (could be added with file-based cache)
- `mobile`: Mobile platforms (experimental)

## Deployment

The application is automatically deployed to GitHub Pages on every push to the `master` branch.

### Automatic Deployment

The GitHub Actions workflow (`.github/workflows/deploy.yml`) automatically:
1. Builds the optimized WebAssembly bundle
2. Configures the application for GitHub Pages subdirectory routing
3. Creates client-side routing support (404.html)
4. Deploys to the `gh-pages` branch

**Deployment URL**: [https://mexus.github.io/Polza-AI-models-viewer/](https://mexus.github.io/Polza-AI-models-viewer/)

### Manual Deployment

You can trigger a deployment manually via the GitHub Actions UI or by pushing to master:

```bash
git push origin master
```

### First-Time Setup

After creating the workflow, configure GitHub Pages:
1. Repository **Settings** → **Pages**
2. **Source**: Deploy from a branch
3. **Branch**: `gh-pages` / `/ (root)`
4. **Save**

For detailed deployment documentation, troubleshooting, and technical details, see the [Deployment section in CLAUDE.md](CLAUDE.md#deployment).

## Contributing

When contributing to this project:

1. Run tests: `cargo test --target $(rustc -vV | grep host | cut -d' ' -f2)`
2. Run clippy: `cargo clippy`
3. Format code: `cargo fmt`
4. Test the app: `dx serve`

## License

[Add your license information here]

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/) 🦀
- Data from [Polza AI API](https://api.polza.ai/)
