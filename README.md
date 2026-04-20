# Rhexiom Studio V2: Forensic Execution Platform

The high-fidelity frontend for Rhexiom, harmonized with the **Obsidian Forensic Design System**. Built with Dioxus and Rust-WASM for high-performance data transparency.

## Studio V2 Features
- **Control Tower**: Unified cockpit for workflow orchestration and telemetry monitoring.
- **Policy Studio**: Advanced bulk deconstruction of PDF policies into executable forensic blueprints.
- **Execution Sandbox**: Glassmorphic terminal for real-time trace testing.
- **Service Registry**: Unified management for integrations (HubSpot, Salesforce, Email).

## Design System: Obsidian Forensic
The UI utilizes a deep-dark, high-depth aesthetic to ensure high-fidelity visibility of forensic data:
- **Palette**: Deep-slate background (#0B0C0E) with glassmorphic depth tokens.
- **Typography**: Inter (UI) and JetBrains Mono (Code/Forensic).
- **Interactions**: Subtle micro-animations and status-pill telemetry for real-time state visualization.

## Technical Stack
- **Framework**: Dioxus (Rust-native frontend).
- **WASM**: Compiled via `wasm-bindgen` for near-native performance.
- **Styling**: Vanilla CSS leveraging CSS Variables for global Obsidian tokens.
- **Client**: `Reqwest` and `Gloo-net` for high-concurrency API orchestration.

## Development
1. Ensure `dx` CLI is installed: `cargo install dioxus-cli`
2. `dx serve` for local dev (Proxying to Backend on Port 3001).
3. `dx build --release` for high-performance production bundles.
