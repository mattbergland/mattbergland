<!-- source: design-systems/skills/design-system/brands/matt-bergland.md v0.1 -->
---
version: 0.1
name: Matt-Bergland-design-system
description: "Matt Bergland's personal brand (mattberg.land): a friendly, human, in-person-community feel. Clean white canvas, slate ink (#1f2937), one warm coral accent (#ff7849), Inter throughout, a heavy lowercase wordmark, coral highlighter-marker emphasis on key phrases, and playful pastel sticky-note tabs with slight rotations. Full light and dark modes. v0: derived from the current site; meant to grow into a full system."
sources: "github.com/mattbergland/mattbergland styles.css + live www.mattberg.land, read 2026-10-19."

colors:
  primary: "#ff7849"           # coral. CTAs, links, location, hover accents
  primary-dark: "#ff9068"      # coral in dark mode
  highlight: "rgba(255,120,73,0.40)"   # marker under key phrases
  highlight-dark: "rgba(255,144,104,0.35)"
  ink: "#1f2937"
  ink-muted: "#9ca3af"
  canvas: "#ffffff"
  surface: "#f3f4f6"           # section panels, footer band, dividers
  card: "#ffffff"
  card-border: "#f3f4f6"
  shadow: "rgba(0,0,0,0.10)"
  note-green: "#d4e8d0"        # sticky note: Community
  note-tan: "#f0e0c8"          # sticky note: Field
  note-blue: "#ccdff0"         # sticky note: Events
  # dark mode (html.dark)
  dark-canvas: "#121212"
  dark-surface: "#1e1e1e"
  dark-card-border: "#2e2e2e"
  dark-ink: "#e5e7eb"
  dark-ink-muted: "#6b7280"
  dark-note-green: "#2a3d26"
  dark-note-tan: "#3d3020"
  dark-note-blue: "#1e2f3d"

typography:
  wordmark:  { fontFamily: Inter, fontSize: 40px, fontWeight: 900, lineHeight: 1, letterSpacing: -0.05em, textTransform: lowercase }
  display:   { fontFamily: Inter, fontSize: 40px, fontWeight: 700, lineHeight: 1.2 }
  headline:  { fontFamily: Inter, fontSize: 32px, fontWeight: 700, lineHeight: 1.25 }
  title:     { fontFamily: Inter, fontSize: 24px, fontWeight: 600, lineHeight: 1.3 }
  lead:      { fontFamily: Inter, fontSize: 17px, fontWeight: 400, lineHeight: 1.6, opacity: 0.7 }
  body:      { fontFamily: Inter, fontSize: 16px, fontWeight: 400, lineHeight: 1.5 }
  nav:       { fontFamily: Inter, fontSize: 16px, fontWeight: 500, opacity: 0.8 }
  button:    { fontFamily: Inter, fontSize: 16-18px, fontWeight: 600 }
  label:     { fontFamily: Inter, fontSize: 14px, fontWeight: 600, letterSpacing: -0.01em }
  micro:     { fontFamily: Inter, fontSize: 11px, fontWeight: 400, opacity: 0.6 }

rounded:
  sm: 4px      # sticky-note tab tops, inputs
  md: 8px      # buttons, small cards
  lg: 12px     # cards, panels
  full: 50%    # avatar, dark-mode toggle

spacing:
  base: 4px (rem-based: 0.25, 0.5, 1, 1.5, 2, 3, 4rem)
  container: 1200px, 5% side padding
---

## Overview

Matt's brand says **"good things tend to happen in person."** He runs community, field, and events at Cognition, and the site should feel warm, approachable, and a little handmade while staying clean and modern. Think a well-organized notebook: white page, dark slate text, one warm coral color, a highlighter swipe under the key phrase, sticky notes on the edge of the page.

**Key characteristics:**
- Inter only, with strong weight contrast (900 wordmark, 700 headlines, 400 body).
- The lowercase **"mattberg.land" wordmark**, 900 weight, tight -0.05em tracking. On hover, a coral bar wipes in underneath it.
- **Coral `#ff7849`** is the one accent: CTAs, link underlines, location text, hover states.
- **Highlighter emphasis:** key phrases get a coral marker stroke covering the bottom 40% of the text (`box-shadow: inset 0 -0.4em 0 0 var(--highlight)`), never a colored text fill.
- **Sticky-note tabs:** pastel green / tan / blue tabs, each slightly rotated (-1.5°, +1°, -0.8°), straightening to 0° when active.
- **Hand-drawn touches:** SVG arrows roughened with an `feTurbulence` + displacement filter.
- First-class dark mode via an `html.dark` class saved in localStorage and applied in `<head>` to avoid a flash.

## Colors
- **Coral** #ff7849 (dark: #ff9068): primary buttons (white text), link underlines (2px), location line, icon hovers.
- **Highlight** rgba(255,120,73,0.40): marker under phrases only.
- **Ink** #1f2937 / **Ink Muted** #9ca3af: text and secondary text. Subheads use ink at 0.7 opacity.
- **Canvas** #ffffff, **Surface** #f3f4f6: page and panels/footer band/1px dividers.
- **Sticky notes**: green #d4e8d0 (Community), tan #f0e0c8 (Field), blue #ccdff0 (Events). These map to his three focus areas; reuse that mapping when categorizing.
- Dark mode: canvas #121212, surfaces #1e1e1e, borders #2e2e2e, ink #e5e7eb, notes darken to #2a3d26 / #3d3020 / #1e2f3d.

## Typography
- **Inter** from Google Fonts (weights 400, 500, 600, 700, plus 900 for the wordmark).
- Hero headline 40px/700/1.2, two short lines, the second carrying the highlighted phrase ("Turn users into fans / via **community-driven growth.**").
- Lead text 17px at 0.7 opacity, max width ~520px.
- Nav links 16px/500 at 0.8 opacity, full opacity on hover.

## Layout
- 1200px max container, 5% side padding, sticky white header with a 1px #f3f4f6 bottom border.
- Home intro is a 1fr / 2fr grid: profile column (120px round avatar, name 24px/600, role with a coral-underlined company link, coral location, one-line bio, social icons) and a hero column (sticky tabs, headline, lead, CTA).
- Generous vertical rhythm: 2–4rem between sections.

## Elevation
Soft and light: cards use a 1px #f3f4f6 border and lift on hover; buttons get `0 4px 12px rgba(0,0,0,0.15)` on hover with a 2px upward move; sticky notes carry `0 2px 6px rgba(0,0,0,0.08)`, deeper when active.

## Components
- **Primary CTA:** coral fill, white 600 text, 8px radius, ~1rem×2rem padding, soft shadow. Hover: lift 2px and deepen shadow. Don't change the hue.
- **Link:** inherit ink color with a 2px coral bottom border; hover turns the text coral.
- **Sticky-note tabs:** 4px top radius, label 14px/600 + micro line 11px at 0.6 opacity, slight rotation, 0.75 opacity when inactive.
- **Cards:** white, 1px #f3f4f6 border, 12px radius (8px on mobile), 1.5rem padding, image bleeds to the edges on top.
- **Dark-mode toggle:** 38px round button, 2px #f3f4f6 border, moon/sun icon, coral border on hover.
- **Social icons:** Font Awesome brand icons, 20px, ink color, coral on hover.

## Voice
Short, warm, confident, people-first. Lead with outcomes for communities ("Turn users into fans"). One-liners over paragraphs. No corporate jargon, no emojis in UI.

## Do's and Don'ts
### Do
- Keep coral as the only saturated color besides the pastel notes.
- Use the highlighter treatment for at most one phrase per section.
- Ship light and dark mode together.
- Add one playful handmade detail (rotation, rough arrow, marker) per page, not ten.

### Don't
- Don't use the leftover blues in the current CSS (`#007bff`, `#0056b3`, `#1d4ed8` hover on CTAs). They are inconsistencies, not brand colors.
- Don't use coral for big background fills or body text.
- Don't mix in a second display font.
- Don't invent testimonials, client logos, or stats about Matt's work.

## Known gaps (to define when the full system is built)
- Logo mark / favicon beyond the text wordmark.
- Defined type scale for long-form pages and a secondary/serif voice, if wanted.
- Form, input, toast, and table styles (current newsletter form is off-brand).
- Illustration and photo style, and an icon set other than Font Awesome.
- Accessible contrast check of coral #ff7849 with white text (about 2.6:1, which fails WCAG AA even for large text; consider a deeper coral for button fills, and use a darker coral like #c2410c, 5.2:1, for small coral text on white).
