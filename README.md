# Rhexiom Frontend: Studio V2 (Obsidian Forensic Console)
**High-Fidelity Management Console for Post-Sovereign Policy Orchestration.**

## 🎭 The Rhexiom Aesthetic
The frontend adheres to the **Obsidian Forensic** design system. Every component must prioritize data-density, high-fidelity contrast, and professional-tier industrial depth.

## 🏗️ Design System Invariants
- **Theme**: Deep Dark (Obsidian) with Glassmorphic depth.
- **Palette**: Deep Gray (#0A0A0B), Emerald Highlights, Translucent Panels.
- **Components**: Use `industrial-card`, `glass-panel`, and `status-pill-forensic` classes exclusively.
- **No Hardcoded Styles**: All semantic tokens must be mapped to the global `main.css` design system.

## 🛠️ Core Modules
- **Policy Studio**: The AI-augmented deconstruction and bulk creation workbench.
- **Execution Sandbox**: The high-fidelity forensic replay and audit environment.
- **Service Registry**: The central control room for HubSpot, Salesforce, and Email integrations.
- **Dashboard**: Real-time stats and dynamic system alerts.

## ✨ UI Components (v1.2)
- **SimulationView**: Tabbed interface (Execution Trace, Decision Paths, Actions, Timing & Memory) with working tab switching.
- **ActivityList**: Debounced search (300ms) for execution history.
- **SystemAlerts**: Dynamic alert fetching from `/api/alerts` endpoint.
- **ConfirmDialog**: Reusable confirmation dialogs for destructive actions.
- **ValidationInput**: Form inputs with inline validation errors.
- **Breadcrumb**: Dynamic breadcrumb navigation showing current route context.
- **ErrorBoundary**: Global error handling with fallback UI.

## 🚀 Development
```bash
# Launch the dev server with proxy to :3001
dx serve
```

## 📱 Mobile Support
- Responsive sidebar with slide-in mobile menu.
- Touch-friendly modal overlay for navigation.
- Adaptive grid layouts for smaller screens.
