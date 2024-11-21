# mattberg.land Website

A minimalist and clean personal website showcasing startup ideas and resources.

## Features

- Clean, modern design with focus on content
- Newsletter subscription form
- Popular guides section with featured content
- Podcast section with platform links
- Portfolio showcase
- Responsive design for all devices

## Setup

1. Replace the Font Awesome kit script in `index.html` with your own kit from [Font Awesome](https://fontawesome.com/)
2. Add your profile image by updating the `.profile-image` class in `styles.css`
3. Update social media links in the HTML file
4. Customize the color scheme by modifying the CSS variables in `styles.css`
5. Add your own content to the guides and portfolio sections

## Customization

### Colors
The color scheme can be customized by modifying the CSS variables in `styles.css`:

```css
:root {
    --primary-color: #2563eb;
    --text-color: #1f2937;
    --background-color: #ffffff;
    --gray-light: #f3f4f6;
    --gray-medium: #9ca3af;
}
```

### Typography
The website uses Inter as the primary font. To change this:
1. Update the Google Fonts link in `index.html`
2. Modify the `font-family` in `styles.css`

## Newsletter Integration

The newsletter form is currently set up with a basic JavaScript handler. To integrate with your email service:
1. Modify the form submission handler in `script.js`
2. Add your email service API endpoint
3. Handle the response appropriately

## Development

To run the website locally, simply open `index.html` in your web browser. No build process is required.
