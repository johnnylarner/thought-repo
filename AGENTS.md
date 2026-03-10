# AGENTS.md - Coding Agent Guidelines

## Project Overview

Zola static site (blog) using the Terminus theme (submodule at `themes/terminus/`). Deployed to GitHub Pages at `https://johnnylarner.github.io/thought-repo`.

Stack: Zola (Rust SSG), Tera templates, SCSS, vanilla JS, TOML config, Markdown content.

## Commands

| Command | Purpose |
|---------|---------|
| `zola build` | Build site to `public/` |
| `zola serve` | Local dev server at `http://127.0.0.1:1111` |
| `zola serve --port 8080` | Serve on custom port |
| `zola build --drafts` / `zola serve --drafts` | Include draft posts |
| `zola check` | Validate links and config |

No test frameworks, linters, or formatters configured. Use `zola check` and `zola build` for validation.

## Project Structure

- `config.toml` — Main site configuration
- `content/` — Markdown content (`_index.md` for sections, `<slug>.md` for posts)
- `sass/`, `static/`, `templates/` — Override directories (empty = use theme defaults)
- `themes/terminus/` — Git submodule (DO NOT edit directly)
- `public/` — Built output

## Content Authoring

Use TOML front matter (`+++`), not YAML (`---`):

```markdown
+++
title = "Post Title"
date = 2026-03-09
description = "Short description for SEO."
canonical_url = "https://johnnylarner.github.io/thought-repo/blog/post-title/"
[taxonomies]
tags = ["tag1", "tag2"]
[extra]
copy_button = true
+++
```

Front matter fields:
- Standard: `title`, `date`, `description`, `canonical_url`, `updated`, `authors`, `draft`
- `[taxonomies]`: `tags`, `categories`
- `[extra]`: `stylesheets`, `copy_button`, `framed`, `social_media_image`

## Code Style

### General

- No comments unless necessary for complex business logic
- Self-documenting code with clear naming

### Tera Templates

- 4-space indentation
- Use whitespace-trimming markers (`{%-` / `-%}`) on block/control tags
- Block names: lowercase single words (`title`, `main`, `header`, `footer`)
- Import macros at top: `{%- import "macros/X.html" as X_macros -%}`
- Call macros: `post_macros::excerpt(page=page)`
- Include partials: `{% include "partials/X.html" %}`
- Use `{% filter indent %}` for output indentation
- Tera comments: `{# ... #}`

### SCSS

- 4-space indentation
- Use `@use` partials (not `@import`); `style.scss` contains only `@use` statements
- Partial filenames: underscore-prefixed, kebab-case (`_theme-selector.scss`)
- CSS custom properties for theme-switchable values (`--background-color`)
- SCSS variables for static values (`$max-layout-width`)
- Class names: kebab-case (`.copy-button`, `.post-navigation`)
- Nest media queries inside selectors
- `//` comments for section headers

### JavaScript

- `const`/`let` only (never `var`)
- Arrow functions for callbacks
- `querySelector`/`querySelectorAll` for DOM access
- `addEventListener` for event binding
- Scripts loaded with `defer` attribute
- ES6 class pattern for complex features; procedural for simple scripts
- camelCase naming

### TOML

- `config.toml`: site-level settings (`base_url`, `theme`, `title`)
- `[markdown]`: rendering options
- `[extra]`: theme-specific variables
- Use `#` comments to document non-obvious settings

## Theme Customization

Terminus is a git submodule — never edit `themes/terminus/` directly.

Override methods:
- Templates: Copy to `templates/` at same relative path
- SCSS: Add files in `sass/`
- Static files: Place in `static/` (same filename overrides theme)
- Config: Set `[extra]` values in root `config.toml`

Key `[extra]` options in `config.toml`:
- `layout` — Page layout style (e.g., `"center"`)
- `copy_button` — Enable code copy buttons
- `show_default_author` — Show author on posts
- `favicon` / `favicon_emoji` — Custom favicon
- `theme_switcher.enable` / `theme_switcher.default` — Runtime theme switching
- `content_security_policy.enable` / `content_security_policy.allowed_domains` — CSP settings

## Git Workflow

- Submodule init: `git submodule update --init --recursive`
- Update theme: `git submodule update --remote themes/terminus`
- `public/` is tracked in git
- `.gitignore` excludes: `/target`, `.env`, `.idea`, `*.db*`

## CI Publishing (dev.to)

- Workflow file: `.github/workflows/publish.yml`
- Publish is triggered on push to `main` and manual `workflow_dispatch`
- Sync scope is all markdown files under `content/blog/`
- `_index.md` files are excluded during conversion
- Posts without a `software` tag are skipped with a warning during DEV conversion
- Create/update behavior is delegated to `sinedied/publish-devto` sync state
- Publishing is done via `sinedied/publish-devto@v2`
- GitHub Pages deployment happens in the same workflow before DEV publishing
- Required GitHub secret: `DEVTO_API_KEY`
- Draft status is converted to `published: false` in the generated DEV front matter
- Posts should include `canonical_url` pointing to the blog URL for SEO attribution on dev.to

## Requirements

Zola >= 0.20.0 required. Check with `zola --version`.
