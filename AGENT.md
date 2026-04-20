# Rhexiom Frontend: Agent Instructions

This guide provides high-fidelity UI protocols for AI Agents interacting with the Rhexiom Studio V2 and the Obsidian Forensic Design System.

## Design Tokens: Obsidian Forensic
When modifying or creating UI components, strictly adhere to these CSS/Design tokens:
- **Background**: `--bg` (#0B0C0E) - The deep-dark forensic canvas.
- **Surface/Card**: `--card-bg` (glassmorphic #121417 with 0.7 opacity).
- **Primary Color**: `--primary` (#00A67E) - The forensic emerald indicator.
- **High-Depth Shadow**: `--shadow` (0 8px 32px rgba(0,0,0,0.4)).
- **Glass Blur**: `backdrop-filter: blur(12px)`.

## Component Hierarchy & Layout
- **Control Tower Navigation**: The sidebar must remain fixed on the left with high-depth contrast indicators for the active route.
- **Industrial Card**: All mission-critical data must be wrapped in `.industrial-card`, which provides the standard glassmorphic border and background.
- **Status Pills**: Utilize the `.status-pill` classes for telemetry visualization. Do not use plain text for state indicators.
- **Input Fields**: Form inputs must use the Obsidian-themed classes in `main.css`. Avoid browser-default borders or focus rings.

## Implementation Protocols (Dioxus)
- **Modularity**: Components should be decoupled and reusable across the Control Tower and Execution Sandbox.
- **Event Handling**: Minimize inline logic; delegate complex state transitions to the `DashboardState` or specialized hooks.
- **Hydration & WASM**: Ensure all components are compatible with WASM-target serialization. Avoid browser-only globals (like `window` or `document`) without safe conditional blocks or `web-sys` abstractions.

## Forensic Aesthetic
- **Contrast**: Maintain a high-depth contrast ratio (minimum 7:1) for all data-heavy views.
- **Micro-Animations**: All interactive elements (buttons, nav items) must include subtle hover-scale or color-shift transitions.
- **Typography**: Inter is for UI; JetBrains Mono is for operational code and forensic results.
