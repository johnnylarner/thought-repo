# thought-repo

Zola-powered blog, deployed to GitHub Pages.

## Local development

- `zola serve` for local preview
- `zola build` to generate `public/`
- `zola check` to validate config and links

## Publishing to dev.to

This repository publishes new articles to dev.to automatically via GitHub Actions.

Workflow:
- File: `.github/workflows/publish-devto.yml`
- Trigger: Pull request closed on `main`
- Guard: Runs only when the PR was merged

Article sync scope:
- Includes all markdown files under `content/blog/`
- Excludes section index files (`_index.md`)

Publishing:
- Uses `sinedied/publish-devto@v2`
- Lets the action handle create/update sync for matching files

Required repository secrets:
- `DEVTO_API_KEY`: dev.to API key for publishing

Notes:
- The workflow is merge-driven. Feature branch updates do not publish.
- If a draft file (`draft = true`) exists in synced content, the action still attempts to publish/sync it.
- For SEO attribution, set `canonical_url` in each post front matter to the GitHub Pages URL for that post.
