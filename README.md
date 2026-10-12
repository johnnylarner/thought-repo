# thought-repo

Zola-powered blog, deployed to GitHub Pages.

## Local development

- `zola serve` for local preview
- `zola build` to generate `public/`
- `zola check` to validate config and links
- `make new-post slug=my-new-post` to scaffold a new blog post in `content/blog/`

## Build and serve with Docker

Initialize the theme submodule, then build and start the container:

```sh
git submodule update --init --recursive
docker build -t thought-repo .
docker run --rm -p 8080:80 thought-repo
```

Open http://localhost:8080. The image builds the site with Zola 0.22.0 and serves the generated files with nginx. Rebuild the image after changing site content. Both builder and runtime images support AMD64 and ARM64, including Apple Silicon.

The default build uses root-relative URLs, so the same image works on any host or published port:

```sh
docker run --rm -p 9090:80 thought-repo
```

For production, set the full public URL at build time:

```sh
docker build --build-arg BASE_URL=https://example.com -t thought-repo .
```

Existing post `canonical_url` values still point to the original blog.

## Publish the Docker image to GHCR

Build, tag, and push the image to GitHub Container Registry. Log in with a GitHub personal access token that has `write:packages` permission:

```sh
docker build -t ghcr.io/johnnylarner/thought-repo:latest .
docker login ghcr.io -u YOUR_GITHUB_USERNAME
docker push ghcr.io/johnnylarner/thought-repo:latest
```

Pull and run it on a host with Docker:

```sh
docker pull ghcr.io/johnnylarner/thought-repo:latest
docker run --rm -p 8080:80 ghcr.io/johnnylarner/thought-repo:latest
```

Open http://localhost:8080. The default root-relative URLs work on any host or published port. To set canonical URLs and feeds to a public domain, pass `--build-arg BASE_URL=https://example.com` when building the image.

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
