#!/bin/sh

set -eu

slug="${1:-}"

if [ -z "$slug" ]; then
    echo "Usage: make new-post slug=my-new-post"
    exit 1
fi

target="content/blog/$slug.md"

if [ -e "$target" ]; then
    echo "$target already exists"
    exit 1
fi

date="$(date +%F)"
title="$(printf '%s' "$slug" | tr '-' ' ' | awk '{for (i = 1; i <= NF; i++) $i = toupper(substr($i, 1, 1)) substr($i, 2)} 1')"

printf '+++\n' > "$target"
printf 'title = "%s"\n' "$title" >> "$target"
printf 'date = %s\n' "$date" >> "$target"
printf 'description = "Short description for SEO."\n' >> "$target"
printf 'canonical_url = "https://johnnylarner.github.io/thought-repo/blog/%s/"\n' "$slug" >> "$target"
printf '[taxonomies]\n' >> "$target"
printf 'tags = []\n' >> "$target"
printf '[extra]\n' >> "$target"
printf 'copy_button = true\n' >> "$target"
printf '+++\n\n' >> "$target"

echo "Created $target"
