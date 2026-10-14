# thought-repo

Zola-powered blog, published as a Docker image on GitHub Container Registry.

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

## Run a published container release

The [publish workflow](.github/workflows/publish.yml) builds an AMD64 and ARM64 image for each push to `main`. After GHCR accepts the image, the workflow creates a GitHub release at the exact source commit. Manual runs publish only when you select `main`.

Each release has a matching Git tag and image tag, `YYYY.MM.DD.HH.MM`, for example `2026.10.14.16.05`. The workflow records the UTC timestamp once when it defines the release identity, with minute precision. Before publishing an image, the workflow reserves the Git tag at the source commit. If the tag already exists, another run in the same minute fails before it can push an image. A failed build can leave the reserved Git tag, and a failed release step can leave a pushed image without a GitHub release. A rerun in a later minute gets a new tag. The workflow does not publish a `latest` image tag. Container releases and DEV publishing run independently, so a DEV failure does not block a container release. DEV runs keep their own concurrency group and cancel older runs.

Open a [GitHub release](https://github.com/johnnylarner/thought-repo/releases) and use its digest-based pull and run commands to run the exact published image. You can also use its matching image tag, replacing `2026.10.14.16.05` with the release tag:

```sh
docker pull ghcr.io/johnnylarner/thought-repo:2026.10.14.16.05
docker run --rm -p 8080:80 ghcr.io/johnnylarner/thought-repo:2026.10.14.16.05
```

Open http://localhost:8080. Published images use root-relative URLs and work on any host or published port. Existing post canonical URLs still point to the original blog until a new public domain is chosen.

The workflow authenticates to GHCR with `GITHUB_TOKEN`. No additional registry secret is required. New GHCR packages are private by default. To allow anonymous pulls, change the package visibility to public. For a private package, authenticate Docker with a token that has `read:packages` permission. If the package was previously pushed manually, grant this repository Actions access in the package settings before running the workflow.

GitHub does not trigger another push workflow for commits created with `GITHUB_TOKEN`, including DEV sync state commits.

## Publishing to dev.to

This repository publishes new articles to dev.to automatically via GitHub Actions.

Workflow:
- File: `.github/workflows/publish.yml`
- Trigger: Push to `main` or manual `workflow_dispatch` on `main`

Article sync scope:
- Includes all markdown files under `content/blog/`
- Excludes section index files (`_index.md`)
- Posts without the `software` tag are skipped with a warning during DEV conversion

Publishing:
- Runs independently of the container release job
- Uses `sinedied/publish-devto@v2`
- Lets the action handle create/update sync for matching files

Required repository secrets:
- `DEVTO_API_KEY`: dev.to API key for publishing

Notes:
- Feature branch pushes and manual runs on feature branches do not publish.
- If a draft file (`draft = true`) reaches DEV conversion, it is sent with `published: false`.
- Existing `canonical_url` values retain the original GitHub Pages URLs until a new public domain is chosen.
