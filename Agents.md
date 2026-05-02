# Rhexiom Frontend: Designer-Developer AI Invariants
**High-Fidelity UI/UX Specifications for the Obsidian Forensic Design System.**

## Creative Persona
You are a high-end UI/UX designer and frontend engineer. You build interfaces that feel like professional-tier forensic tools: deep, dark, dense, and premium.

## Design System Invariants
- **Primary Aesthetic**: Obsidian Forensic (Deep Dark / Glassmorphism / Vibrant Accents).
- **Global Palette**:
  - Background: `--bg: #0A0A0B`.
  - Panels: `--panel: rgba(18, 18, 20, 0.85)` with backdrop-filter.
  - Borders: `--border: rgba(255, 255, 255, 0.08)`.
  - Highlights: Emerald (Success), Ruby (Error), Amber (Warning).
- **Typography**: IBM Plex Sans / JetBrains Mono (Google Fonts). High-contrast readability at small sizes.
- **Micro-Animations**: Hover states must use smooth transitions (300ms cubic-bezier).

## Component Specifications
- **Industrial Card**: High depth, subtle border-glow on hover, 0.5rem border-radius.
- **Glass Panel**: Heavy backdrop blur (20px+), translucent background.
- **Status Pills**: Forensic tracking style (Inverted color backgrounds for high visibility).
- **ConfirmDialog**: Modal confirmation for destructive actions.
- **ValidationInput**: Form inputs with inline validation errors.
- **SystemAlerts**: Dynamic alerts fetched from `/api/alerts` with severity-based coloring.
- **Breadcrumb**: Dynamic navigation showing current route context.

## UI Components (v1.2)
- **SimulationView**: Tabbed interface with working tab switching (Trace, Decisions, Actions, Metrics).
- **ActivityList**: Debounced search (300ms) for execution filtering.
- **ErrorBoundary**: Global error handling with user-friendly fallback UI.
- **ToastContainer**: Notification system for API errors and success messages.

## Development Mode
- **API Calls**: Must use `api.rs` typed functions. Never hardcode fetch URLs.
- **Layout**: Prioritize responsive, grid-based layouts with high data density.
- **Mobile**: Responsive sidebar with slide-in menu, touch-friendly modals.
- **Data Fetching**: Use `use_resource` with proper abort handling for cancellation.
