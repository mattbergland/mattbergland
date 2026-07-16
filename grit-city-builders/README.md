# Grit City Builders

A rebuilt, business-optimized marketing site for **Grit City Builders** — a family-run custom home builder and general contractor in Tacoma, WA and the South Sound.

This is a modern, fast, static rebuild of the original Wix site (gritcitybuilders.com) with improved design, SEO, conversion, and easy-to-access image assets.

## What's here

| Page | File | Purpose |
|------|------|---------|
| Home | `index.html` | Value proposition, services overview, featured project, process, CTAs |
| About | `about.html` | Founders' story (Matt & Alexis Bender), values |
| Services | `services.html` | New construction, full-scale renovations, ADUs/DADUs, process, FAQ |
| Portfolio | `portfolio.html` | Project grid across the South Sound |
| Prospect House | `prospect-house.html` | Full before/after case study (1904 restoration) |
| Contact | `contact.html` | Lead form + service area |

- `css/styles.css` — design system (responsive, accessible, reduced-motion aware)
- `js/main.js` — nav, scroll reveal, gallery lightbox, contact form handling
- `images/` — optimized, descriptively named photos + `manifest.json`
- `sitemap.xml` — for the gritcitybuilders.com domain

## Improvements over the original

- **Clear value prop + primary CTA** ("Start Your Project") on every page and above the fold
- **Conversion-focused structure**: trust bar, service cards, featured case study, transparent 4-step process, FAQ, and repeated CTAs
- **SEO**: descriptive titles/meta, Open Graph tags, `GeneralContractor` + `Service` JSON-LD structured data, canonical URLs, sitemap, semantic headings, local service-area keywords
- **Performance**: images resized and re-encoded (**~685 MB → ~23 MB**), lazy loading, `fetchpriority` on the hero, system/Google fonts with preconnect
- **Accessible & responsive**: keyboard-navigable lightbox, focus styles, mobile nav, `prefers-reduced-motion` support
- **Working contact form**: posts to Formspree when configured, with a `mailto:` fallback so it works immediately

## Images — easy to access

All photos were scraped from the live site and re-optimized for the web. They live in `images/` with human-readable names, e.g.:

- `images/hero-home.jpg`, `images/founders-matt-alexis.jpg`
- `images/service-new-construction.jpg`, `images/service-renovation.jpg`, `images/service-adu.jpg`
- `images/portfolio/*.jpg` — one per project
- `images/prospect-house/*.jpg` — full before/after set (files ending `-before` are the "before" shots)
- `images/brand/logo.png` (dark) and `images/brand/logo-white.png` (for dark backgrounds)

`images/manifest.json` maps every original Wix media ID to its optimized file, dimensions, and byte size.

## Run locally

```bash
python3 -m http.server 8000
# then open http://localhost:8000/grit-city-builders/
```

## Go live with the contact form

1. Create a free form endpoint at [formspree.io](https://formspree.io).
2. In `contact.html`, replace `FORM_ID` in the form `action` with your endpoint ID.

Until then, the form opens a pre-filled email to `gritcitybuilders@gmail.com`.
