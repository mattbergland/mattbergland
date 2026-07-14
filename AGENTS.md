# AGENTS.md

Guidance for AI agents (and humans) working on this repo.

## What this is

The personal website for Matt Bergland, deployed to **www.mattberg.land**
(see `CNAME`). It is a **zero-build static site**: vanilla HTML5, CSS3, and
JavaScript with no package manager, bundler, or build step.

## Structure

- `index.html` — home page (bio, guides, newsletter, podcast, footer).
- `projects.html` — portfolio grid with client-side voting.
- `about.html` — about page.
- Service / content pages: `community-building.html`, `content-creation.html`,
  `event-playbooks.html`, `growth-strategies.html`, `community-in-action.html`.
- `styles.css` — all styling, including CSS variables (`:root`), dark-mode
  rules (`.dark`), responsive media queries, and print overrides.
- `script.js` — all interactivity (guide accordions, newsletter form,
  dark-mode toggle, project voting, footer arrow animation).
- `images/` — project screenshots; `bioimage.jpeg` — profile photo.
- `CNAME` — custom domain for GitHub Pages.

## Conventions

- No build step. Edit HTML/CSS/JS directly; changes are live on reload.
- Shared header/footer markup is duplicated across pages (no templating) —
  when changing nav or footer, update every `*.html` page consistently.
- Colors and theme come from CSS variables in `styles.css` `:root`; prefer
  editing variables over hard-coding colors.
- Dark mode: preference is stored in `localStorage` under `darkMode` and
  applied via the `dark` class on `<html>` (inline script in `<head>`
  prevents flash). Style dark variants with `.dark` selectors.
- Client-side persistence uses `localStorage` (dark mode, project votes) —
  there is no backend.
- Keep the design minimalist and consistent with existing pages.

## Running locally

No build required. Serve the folder with any static server, e.g.:

```
python3 -m http.server 8000
```

then open http://localhost:8000. Opening `index.html` directly in a browser
also works for most things.

## Deployment

Deployed via GitHub Pages on push to the default branch; `CNAME` maps it to
www.mattberg.land. There is nothing to build or configure.
