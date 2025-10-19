# Polza AI Model Browser

A web application built with Dioxus that provides an interactive browser for AI models available through the Polza AI API. The application features real-time filtering, responsive design, and elegant error handling.

## Features

- **Real-time Model Filtering**: Filter models by name as you type with instant results
- **API Integration**: Fetches live data from the Polza AI models endpoint
- **Smart Data Handling**: Automatically filters out models with empty pricing information
- **Loading States**: Smooth loading animations while fetching data
- **Error Recovery**: User-friendly error messages with retry functionality
- **Responsive Design**: Clean, modern UI that works across different screen sizes
- **Interactive UI**: Hover effects and smooth transitions for better user experience

## Tech Stack

- **[Dioxus](https://dioxuslabs.com/)**: React-like framework for building cross-platform user interfaces in Rust
- **[reqwest](https://docs.rs/reqwest/)**: HTTP client for async API requests
- **[serde](https://serde.rs/)**: Serialization/deserialization of JSON data
- **[rust_decimal](https://docs.rs/rust_decimal/)**: Precise decimal handling for pricing information

## Project Structure

```
polza-models/
├─ assets/          # Static assets (images, fonts, etc.)
├─ src/
│  └─ main.rs       # Main application file containing:
│                   #   - Data models (Model, Pricing, Architecture, etc.)
│                   #   - API integration logic
│                   #   - UI components and styling
│                   #   - Application entry point
├─ Cargo.toml       # Project dependencies and configuration
└─ README.md        # This file
```

## API Integration

The application connects to the **Polza AI Models API**:
- **Endpoint**: `https://api.polza.ai/api/v1/models`
- **Response Format**: JSON array of model objects with pricing and architecture information
- **Modalities Supported**: Text, Image, File, Embeddings

Each model includes detailed pricing information for:
- Prompt tokens
- Completion tokens
- Image processing
- Per-request costs
- Web search functionality
- Internal reasoning
- Input cache (read/write)

## Development

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform` flag:
```bash
dx serve --platform desktop
dx serve --platform web
```

### Building for Production

```bash
dx build --release
```

