---
name: SimWare Design System
colors:
  surface: '#10131a'
  surface-dim: '#10131a'
  surface-bright: '#363941'
  surface-container-lowest: '#0b0e15'
  surface-container-low: '#191b23'
  surface-container: '#1d2027'
  surface-container-high: '#272a31'
  surface-container-highest: '#32353c'
  on-surface: '#e1e2ec'
  on-surface-variant: '#c2c6d6'
  inverse-surface: '#e1e2ec'
  inverse-on-surface: '#2e3038'
  outline: '#8c909f'
  outline-variant: '#424754'
  surface-tint: '#adc6ff'
  primary: '#adc6ff'
  on-primary: '#002e6a'
  primary-container: '#4d8eff'
  on-primary-container: '#00285d'
  inverse-primary: '#005ac2'
  secondary: '#c0c7d3'
  on-secondary: '#2a313b'
  secondary-container: '#404752'
  on-secondary-container: '#afb5c2'
  tertiary: '#ffb786'
  on-tertiary: '#502400'
  tertiary-container: '#df7412'
  on-tertiary-container: '#461f00'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#d8e2ff'
  primary-fixed-dim: '#adc6ff'
  on-primary-fixed: '#001a42'
  on-primary-fixed-variant: '#004395'
  secondary-fixed: '#dce3f0'
  secondary-fixed-dim: '#c0c7d3'
  on-secondary-fixed: '#151c25'
  on-secondary-fixed-variant: '#404752'
  tertiary-fixed: '#ffdcc6'
  tertiary-fixed-dim: '#ffb786'
  on-tertiary-fixed: '#311400'
  on-tertiary-fixed-variant: '#723600'
  background: '#10131a'
  on-background: '#e1e2ec'
  surface-variant: '#32353c'
typography:
  display:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '600'
    lineHeight: 40px
    letterSpacing: -0.02em
  h1:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.01em
  h2:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  body-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 18px
  label-md:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0.02em
  code-md:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 20px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 8px
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 32px
  container-margin: 24px
  gutter: 16px
---

## Brand & Style

The design system is built for a high-stakes technical environment where clarity, speed of cognition, and reliability are paramount. The brand personality is **utilitarian, sophisticated, and authoritative**, evoking the precision of developer tools like VS Code and the robust security posture of CrowdStrike.

The visual style is a refined **Modern Minimalist** approach. It prioritizes content and data over decorative elements. By utilizing a deep, monochromatic foundation, the system ensures that malware indicators and status alerts (Success, Warning, Danger) remain the primary focus of the analyst's attention. High-end enterprise aesthetics are achieved through precise alignment, generous whitespace, and a strict adherence to a functional hierarchy.

## Colors

The palette is strictly functional. The foundation is built on a "Dark Mode First" philosophy to reduce eye strain during long analysis sessions. 

- **Neutral Tones:** Used for structural hierarchy. The background (`#0F1115`) provides the base, while surfaces (`#1A1D23`) and lighter layers (`#242830`) create a sense of depth without relying on heavy shadows.
- **Accents:** The Primary Muted Blue is used exclusively for primary actions and active states. 
- **Semantic Colors:** Green, Amber, and Red are reserved for status indicators (Clean, Suspicious, Malicious). They should be used sparingly—only when data requires an emotional or urgent response.
- **Typography:** High contrast is maintained for readability (`#F9FAFB`), while metadata and inactive states use a dampened gray (`#9CA3AF`).

## Typography

This design system utilizes **Inter** for all UI elements to ensure maximum legibility and a neutral, professional tone. A secondary typeface, **JetBrains Mono**, is introduced for technical strings, file paths, hashes, and code snippets, providing the necessary distinction for technical data analysis.

**Hierarchy Rules:**
- Use `display` and `h1` sparingly, mainly for dashboard overviews or report titles.
- `body-md` is the default size for most UI interactions.
- `label-md` should be used for all-caps section headers or small metadata tags.
- All code-related data (SHA-256, Registry Keys) must use `code-md` to prevent character confusion.

## Layout & Spacing

The system follows a strict **8-point grid**. All dimensions, padding, and margins must be multiples of 8px (with 4px used for tight component internals).

- **Grid System:** A 12-column fluid grid is used for dashboards, while sidebars are fixed at 240px or 280px depending on content density.
- **Density:** The layout favors "Comfortable" density for general navigation but allows for "Compact" density (using the 4px unit) within data tables and log viewers.
- **Alignment:** Consistent internal padding of 16px (`md`) should be applied to cards and containers to maintain a rhythmic vertical scan line.

## Elevation & Depth

Depth is communicated primarily through **Tonal Layering** rather than shadows. This mimics the physical layering of a digital workspace.

- **Level 0 (Background):** `#0F1115` - The lowest plane.
- **Level 1 (Default Surface):** `#1A1D23` - Used for primary content cards and navigation sidebars.
- **Level 2 (Raised Surface):** `#242830` - Used for hover states, tooltips, and modals.
- **Borders:** All containers must have a 1px solid border of `#2D333B` to define boundaries against the dark background.
- **Shadows:** Use a single, very subtle ambient shadow for modals: `0 4px 12px rgba(0, 0, 0, 0.5)`. Avoid shadows on standard buttons and cards.

## Shapes

The shape language is structured yet approachable. A consistent **12px (`rounded-lg`)** radius is the standard for cards, modals, and large containers.

- **Small Components:** Buttons, input fields, and tags use an **8px** radius to maintain a crisp, professional look.
- **Selection Indicators:** Use a 4px radius for subtle focus states or sidebar active-item indicators.
- **Consistency:** Avoid pill-shaped buttons; maintain the rectangular-with-soft-corners aesthetic to align with the enterprise-grade tool metaphor.

## Components

### Buttons
- **Primary:** Solid `#3B82F6` with white text. No gradient. 
- **Secondary:** Surface-Lighter (`#242830`) background with a `#2D333B` border.
- **Ghost:** Transparent background, `text-secondary` color, appearing only on hover.

### Input Fields
- Background should be the background color (`#0F1115`) to create an "inset" feel against the Surface panels. 
- 1px border using the Border token, changing to Primary on focus.

### Status Chips (Badges)
- Use a "subtle" style: 10% opacity of the semantic color (Red, Green, Amber) for the background, with the 100% opaque color for the text.

### Data Tables
- Header rows should use `label-md` with `text-secondary`.
- Row borders should be 1px solid `#2D333B` only on the bottom. 
- Hover state on rows should change the background to `surface-lighter`.

### Analysis Cards
- Use for grouping simulation results. Borders are mandatory. Use a small 4px left-accent border of a semantic color (e.g., Red) to indicate the severity of the item within the card.

### Monospaced Code Blocks
- Used for process execution logs. Background: `#000000` (pure black) for maximum contrast against the `code-md` text.
