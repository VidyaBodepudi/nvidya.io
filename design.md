# Hacker Blog Design Baseline

This document serves as the baseline design specification for the Hacker Blog template, blending the professional layout of Hydejack with a cyberpunk/matrix aesthetic.

## 1. Core Concept
A static, easy-to-deploy personal website (HTML/CSS/JS) designed for GitHub Pages. It maintains a clean, professional structure while using styling and micro-animations to achieve a "hacker" identity.

## 2. Layout Structure
**Inspiration:** Hydejack Theme
- **Sidebar (Left):** Fixed navigation panel (320px width). Contains the user's avatar, name, role, main navigation links, and social links. It acts as the anchor for the user's identity. On mobile (< 900px), this collapses to a top-level header.
- **Main Content (Right):** Smooth-scrolling content area displaying sections like Home, Projects, Blog, and Contact.

## 3. Color Palette
The color scheme relies on high contrast between deep dark backgrounds and vibrant neon accents.
- **Main Background:** `#090b10` (Deep GitHub Dark Mode)
- **Sidebar Background:** `#030508` (Nearly black, provides depth)
- **Accent Color:** `#00ff41` (Classic terminal/matrix neon green)
- **Accent Glow:** `rgba(0, 255, 65, 0.4)` (Used for hover states and text shadows)
- **Primary Text:** `#c9d1d9` (Off-white for readability)
- **Muted Text:** `#8b949e` (Used for secondary info and prompts)
- **Card Background:** `#11151c` (Slightly lighter than main background to create elevation)
- **Borders:** `#242931` (Subtle dividers)

## 4. Typography
- **Headings, Navigation, Code, & Accents:** `Fira Code` (Monospace). Establishes the technical, developer-centric feel.
- **Body Text:** `Inter` (Sans-serif). Ensures long-form text (like blog posts or bio) remains highly readable and professional.

## 5. UI Elements
- **Terminal Windows:** Used to display bio (`whoami`) and contact information. Styled with top-left window controls (Red, Yellow, Green dots) and a prompt/output structure.
- **Cards (Projects):** Clean borders that feature a glowing neon green bottom-to-top border reveal on hover.
- **Badges:** Small, pill-shaped tags for technologies (e.g., Rust, Go) with a translucent green background.

## 6. Animations & Dynamic Effects
- **Matrix Rain Background:** A `<canvas>` element in the sidebar background streams falling, randomized green characters. Opacity is kept low (`0.15`) to avoid distracting from navigation.
- **Avatar Hover:** The profile picture defaults to 100% grayscale and high contrast. On hover, it transitions to full color, scales up slightly (`1.05`), and gains a neon green box-shadow glow.
- **Text Glitch Effect:** The main profile name uses a continuous, pure CSS RGB split (`#ff00c1` and `#00fff9`) text-shadow animation to simulate a digital glitch. The animation pauses momentarily on hover.
- **Typing Effect:** The hero header (`Initializing system...`) is typed out character-by-character on page load using JavaScript.
- **Scroll Fade-in:** As the user scrolls, `IntersectionObserver` triggers sections to smoothly fade in and translate upwards.
- **Active Navigation State:** The navigation links update their active state dynamically based on which section is currently visible in the viewport.

---
*Note: This baseline serves as a reference point. Any future modifications to the layout, color scheme, or animations should be documented here.*
