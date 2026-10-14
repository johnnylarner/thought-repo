# AGENTS.md - Coding Agent Guidelines

## Project Overview

Zola static site (blog) using the Terminus theme (submodule at `themes/terminus/`). Published as a multi-platform Docker image on GHCR with a matching GitHub release. Existing canonical URLs retain `https://johnnylarner.github.io/thought-repo` until a new public domain is chosen.

Stack: Zola (Rust SSG), Tera templates, SCSS, vanilla JS, TOML config, Markdown content.

## Important Rules

- **Never modify files in `themes/terminus/` directly.** It is a git submodule. Always override by copying to the equivalent path in the project root (`templates/`, `sass/`, `static/`).

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

## CI publishing and container releases

- Workflow file: `.github/workflows/publish.yml`
- Publish is triggered on push to `main` and manual `workflow_dispatch` on `main`
- Both jobs skip manual runs on other branches
- The release job checks out the exact triggering SHA with recursive submodules
- The release job pushes AMD64 and ARM64 images to `ghcr.io/<lowercase-owner>/<lowercase-repository>` before creating a GitHub release at that SHA
- Git tags and image tags match: `YYYY.MM.DD.HH.MM`, for example `2026.10.14.16.05`
- The workflow records the UTC timestamp once when it defines the release identity, with minute precision
- The workflow reserves the Git tag at the source commit before publishing an image. If the tag already exists, another run in the same minute fails before it can push an image
- A failed build can leave the reserved Git tag. A failed release step can leave a pushed image without a GitHub release. A rerun in a later minute gets a new tag
- No `latest` image alias is published
- Release notes include the image tag, manifest digest commands, source SHA, package link, and workflow link
- Registry authentication uses `GITHUB_TOKEN`, with no additional registry secret
- New GHCR packages default to private. Existing manually pushed packages may need repository Actions access
- GitHub suppresses push workflows for commits created with `GITHUB_TOKEN`, including DEV sync state commits
- Sync scope is all markdown files under `content/blog/`
- `_index.md` files are excluded during conversion
- Posts without a `software` tag are skipped with a warning during DEV conversion
- Create/update behavior is delegated to `sinedied/publish-devto` sync state
- Publishing is done via `sinedied/publish-devto@v2`
- Container releases and DEV publishing run as independent jobs
- Only DEV uses a concurrency group, with older runs canceled
- GitHub Pages deployment is no longer part of the workflow
- Required GitHub secret: `DEVTO_API_KEY`
- Draft status is converted to `published: false` in the generated DEV front matter
- Posts should include `canonical_url` pointing to the blog URL for SEO attribution on dev.to

## Requirements

Zola >= 0.20.0 required. Check with `zola --version`.
