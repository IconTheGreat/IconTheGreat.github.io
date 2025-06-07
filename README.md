# Xeniria

Xeniria is a lightweight and fast static site generator built in Rust. It allows you to create and deploy static websites easily using Markdown.

## Features

* Converts Markdown (`.md`) files into static HTML pages
* Simple configuration with `config.toml`
* Built-in local development server
* SEO optimized
* Syntax highlighting for code blocks
* OG image support
* Easy deployment to GitHub Pages
* Fast and lightweight — built in Rust

---

## Installation

### 1. Install Rust

Ensure you have Rust installed on your system:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository

To set up your own Xeniria-based site, clone the repository and name it using your GitHub Pages format: `<your-username>.github.io`. For example, if your GitHub username is `johndoe`, the repository should be named `johndoe.github.io`.

```sh
git clone git@github.com:0xh4ty/xeniria.git <your-repo-name>
cd <your-repo-name>
```

### 3. Create a New GitHub Repository

Go to GitHub and create a new repository named `<your-username>.github.io`. For example, if your GitHub username is `johndoe`, create a repository called `johndoe.github.io`. Ensure it is **empty** (do not add a README or `.gitignore`).

### 4. Set Up Remote Repositories

```sh
git remote remove origin
git remote add origin git@github.com:<your-username>/<your-repo-name>.git
```

### 5. Add Xeniria as Upstream

```sh
git remote add upstream git@github.com:0xh4ty/xeniria.git
```

### 6. Build and Install Xeniria

```sh
cargo install --path .
```

---

## Usage

### 1. Build the Site

```sh
xeniria build
```

This converts all Markdown files into HTML inside the `docs/` directory.

### 2. Run Local Development Server

```sh
xeniria serve
```

Your site will be available at `http://localhost:8464`.

### 3. Writing Blog Posts

To write a blog post, create a `.md` file inside the `content/` directory. Each post should include **front matter** at the top:

```
---
title: "The Art of Writing"
date: "2025-02-08"
author: "John Doe"
description: "A reflection on writing as a thinking tool — techniques, mindset, and clarity."
---
```

### 4. Editing About and License Pages

Use the same filenames (`about.md` and `license.md`) inside the `content/` directory and modify them as needed.

---

## Deploying to GitHub Pages

For a live example, check out [my website](https://0xh4ty.github.io/) which is built using Xeniria.

### 1. Configure GitHub Pages

* Go to your repository → **Settings → Pages**.
* Set the deployment branch to `main` and folder to `/docs`.

### 2. Push Your Site to GitHub

```sh
git add .
git commit -m "Deploy site"
git push origin main
```

Your website will be live at:

```
https://<your-username>.github.io/
```

---

## Configuration

Modify `config.toml` to customize your site:

```toml
# Site Configuration
[site]
title = "Xeniria"  # Change this to your website's title
description = "A minimal Static Site Generator built with Rust."  # Short description of yourself
author = "Xeniria"  # Enter your name
profile_picture = "assets/img/xeniria.png"  # Place your image in the "assets/img/" directory and update the reference here, or provide the URL of your Twitter profile picture
base_url = "https://yourdomain.com"  # The full URL where your site is hosted (used for sitemap.xml and SEO metadata). Example: "https://0xh4ty.github.io" or "https://myblog.com"

# Optional: specify a dedicated OG image (1200x630 recommended) for better link previews on Twitter, Telegram, Discord, etc.
# If not specified, the profile_picture will be used as the fallback OG image.
# og_image = "assets/img/og-default.png"

# Navigation Links
[links]
github = "#Your_Github_Link"  # Replace with your GitHub profile link (e.g., "https://github.com/yourusername")
twitter = "#Your_Twitter_Link"  # Replace with your Twitter profile link (e.g., "https://twitter.com/yourusername")

# Analytics
[analytics]
plausible_domain = ""  # Optional: set your domain here to enable Plausible (example: "myblog.com")
cloudflare_beacon_token = ""  # Optional — if set, inserts Cloudflare Analytics
```

---

## Keeping Up with Updates

To stay up to date with the latest changes in Xeniria, fetch and merge updates from the upstream repository:

```sh
git fetch upstream
git merge upstream/main
```

---

## Development

To contribute or modify Xeniria:

```sh
git clone git@github.com:0xh4ty/xeniria.git
cd xeniria
cargo build
cargo run -- build
cargo run -- serve
```

---

## License

Xeniria is open-source under the MIT License.
