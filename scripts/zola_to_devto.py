#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Convert Zola markdown files with TOML front matter to dev.to markdown files with YAML front matter."
    )
    parser.add_argument("source", type=Path, help="Source markdown file or directory")
    parser.add_argument("target", type=Path, help="Target markdown file or directory")
    return parser.parse_args()


def split_zola_front_matter(text: str, path: Path) -> tuple[str, str]:
    if not text.startswith("+++\n"):
        raise ValueError(f"{path}: expected TOML front matter starting with +++")

    closing_marker = "\n+++"
    closing_index = text.find(closing_marker, 4)
    if closing_index == -1:
        raise ValueError(f"{path}: missing closing +++ for TOML front matter")

    front_matter = text[4:closing_index]
    body_start = closing_index + len(closing_marker)
    if body_start < len(text) and text[body_start] == "\n":
        body_start += 1

    return front_matter, text[body_start:]


def yaml_scalar(value: object) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    return json.dumps(str(value), ensure_ascii=True)


def yaml_list(values: list[object]) -> str:
    return "[" + ", ".join(yaml_scalar(value) for value in values) + "]"


def build_devto_front_matter(metadata: dict[str, object]) -> str:
    taxonomies = metadata.get("taxonomies")
    extra = metadata.get("extra")

    tags: list[object] = []
    if isinstance(taxonomies, dict):
        raw_tags = taxonomies.get("tags")
        if isinstance(raw_tags, list):
            tags = raw_tags

    cover_image = None
    devto_id = None
    if isinstance(extra, dict):
        cover_image = extra.get("social_media_image")
        devto_id = extra.get("devto_id")

    lines = [
        "---",
        f"title: {yaml_scalar(metadata.get('title'))}",
        f"description: {yaml_scalar(metadata.get('description'))}",
        f"tags: {yaml_list(tags)}",
        f"cover_image: {yaml_scalar(cover_image)}",
        f"canonical_url: {yaml_scalar(metadata.get('canonical_url'))}",
        f"published: {yaml_scalar(not bool(metadata.get('draft', False)))}",
    ]

    if devto_id is not None:
        lines.append(f"id: {yaml_scalar(devto_id)}")

    lines.append("---")
    return "\n".join(lines)


def convert_file(source_path: Path, target_path: Path) -> None:
    text = source_path.read_text(encoding="utf-8")
    front_matter_text, body = split_zola_front_matter(text, source_path)
    metadata = tomllib.loads(front_matter_text)
    devto_front_matter = build_devto_front_matter(metadata)

    target_path.parent.mkdir(parents=True, exist_ok=True)
    target_path.write_text(f"{devto_front_matter}\n{body}", encoding="utf-8")


def iter_markdown_files(source_dir: Path) -> list[Path]:
    return sorted(
        path
        for path in source_dir.rglob("*.md")
        if path.name != "_index.md"
    )


def convert_path(source: Path, target: Path) -> None:
    if source.is_file():
        output_path = target / source.name if target.exists() and target.is_dir() else target
        convert_file(source, output_path)
        return

    if not source.is_dir():
        raise ValueError(f"{source}: source path does not exist")

    target.mkdir(parents=True, exist_ok=True)
    for source_file in iter_markdown_files(source):
        relative_path = source_file.relative_to(source)
        convert_file(source_file, target / relative_path)


def main() -> int:
    args = parse_args()

    try:
        convert_path(args.source, args.target)
    except Exception as exc:
        print(str(exc), file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
