# thought-repo

Zola-powered blog, deployed to GitHub Pages.

## Local development

- `zola serve` for local preview
- `zola build` to generate `public/`
- `zola check` to validate config and links
- `make new-post slug=my-new-post` to scaffold a new blog post in `content/blog/`

## Publishing to dev.to

This repository publishes new articles to dev.to automatically via GitHub Actions.

Workflow:
- File: `.github/workflows/publish.yml`
- Trigger: Push to `main` or manual `workflow_dispatch`

Article sync scope:
- Includes all markdown files under `content/blog/`
- Excludes section index files (`_index.md`)
- Posts without the `software` tag are skipped with a warning during DEV conversion

Publishing:
- Builds and deploys the Zola site to GitHub Pages in the same workflow
- Uses `sinedied/publish-devto@v2`
- Lets the action handle create/update sync for matching files

Required repository secrets:
- `DEVTO_API_KEY`: dev.to API key for publishing

Notes:
- The workflow is push-driven on `main`. Feature branch updates do not publish unless manually triggered after merge.
- If a draft file (`draft = true`) reaches DEV conversion, it is sent with `published: false`.
- For SEO attribution, set `canonical_url` in each post front matter to the GitHub Pages URL for that post.
