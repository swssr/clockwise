# Clockwise

A lightweight macOS menubar app that displays multiple time zones in your menu bar. Built with Tauri, React, and Rust.

## Features

- **Multi-timezone display** - Shows current time for Austin (TX), New York (NY), and Manila (PH) in the menu bar
- **Live updates** - Times refresh every second
- **Native menubar integration** - Click the tray icon to show/hide the app panel
- **Minimal footprint** - Runs as an accessory app without a dock icon

## Prerequisites

- [Node.js](https://nodejs.org)
- [Tauri CLI](https://tauri.studio/docs/getting-started/installation)
- [pnpm](https://pnpm.io) (or use the packageManager field in package.json)

## Getting Started

```bash
# Install dependencies
pnpm install

# Run in development
pnpm tauri dev
```

The app will appear in your menu bar showing the three timezones.

## Usage

- Look in your macOS menu bar for the time display
- Click the tray icon to toggle the app panel
- Times update automatically every second

## Tech Stack

- **Frontend:** React + TypeScript + Vite
- **Backend:** Rust + Tauri
- **Plugins:** tauri-nspanel (for native menu bar panels)

## License

MIT License. See the [LICENSE](./LICENSE.md) file for details.
