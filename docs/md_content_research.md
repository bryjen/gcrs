# Markdown content pipeline (context/ui)

This note summarizes how the **context/ui** project turns `.md` files into Leptos-rendered pages. It is concise but references the exact components and files so both humans and agents can trace the flow.

## 1) Example input (MD file)

File: `context/ui/public/docs/components/accordion.md`

Frontmatter is TOML between `+++` and `+++`, followed by markdown that can include custom component tags:

```
+++
title = "Accordion"
description = "Rust/UI component that displays an Accordion."
tags = ["accordion"]
image = "/images/thumbnails/accordion.webp"
+++

<StaticAccordion />
```

The custom tags (e.g., `<StaticAccordion />`) are interpreted as Leptos components later in the pipeline.

## 2) Server-side read + parse

File: `context/ui/app/src/domain/markdown_ui/api_read_md.rs`

Key server functions:

- `read_md_file_typed(path: String) -> MdFile<RegistryEntry>`
- `read_raw_md_file(path: String) -> String`

Important behavior:

- Uses `LEPTOS_SITE_ROOT` and strips a `public/` prefix.
- Loads file contents from the filesystem at runtime.
- Parses with `MdFile::parse_md_file(...)`.

## 3) Frontmatter parsing

File: `context/ui/crates/_markdown_crate/src/config/md_file.rs`

`MdFile<T>` parses:

- TOML frontmatter from `+++` delimited block.
- Remaining markdown content.

If frontmatter is missing or invalid, parse errors are surfaced.

## 4) Markdown → HTML (with classes)

File: `context/ui/crates/_markdown_crate/src/html_converter.rs`

Uses `pulldown-cmark` to convert markdown to HTML and adds Tailwind-friendly classes for headings, lists, tables, etc. The converter is used by the Leptos renderer.

## 5) HTML → Leptos views (custom components)

File: `context/ui/crates/_markdown_crate/src/leptos/leptos_converter.rs`

`LeptosConverter(md_content, md_components)`:

- Converts markdown to HTML.
- Parses HTML into a DOM tree.
- Walks the DOM and maps elements to Leptos views.
- If an element name matches a registered component (e.g., `StaticAccordion`), it renders that component instead of generic HTML.

Component registry:

```
pub struct MdComponents { components: HashMap<String, Box<dyn Fn(MdComponentProps) -> View<AnyView>>> }
```

This allows mapping `<CustomTag />` in markdown to any Leptos component.

## 6) Runtime rendering in docs pages

File: `context/ui/app/src/domain/markdown_ui/components/md_shared_with_toc.rs`

`SharedDemoMdWithToc`:

- Calls `read_md_file_typed` via a `Resource`.
- Extracts table of contents (`extract_toc_from_md`).
- Renders the content with `MyMd`.

`MyMd` is the registry-backed component that wires **shared component registrations** and passes them into `LeptosConverter`.

## 7) Static registry and shared components

File: `context/ui/app/src/__registry__/static_md_registry.rs`

`MyMd` sets up a global registry of markdown components (e.g., `StaticAccordion`, `StaticAccordionBordered`) using:

```
combined_components.add("StaticAccordion", StaticDemoAccordion);
```

This makes custom tags in markdown resolve to actual Leptos components.

Also related:

- `context/ui/app/src/__registry__/demos_sidenav.rs` registers nav entries (`RegistryEntry`) including `path_md`.

## 8) Key takeaway for a blog

The pipeline already supports:

1) markdown frontmatter + body,
2) server-side read/parse,
3) Leptos conversion with custom components,
4) shared rendering with TOC.

To create a blog:

- Put posts in `context/ui/public/docs/blog/*.md`.
- Add a blog route that uses `SharedDemoMdWithToc` (or a lighter wrapper).
- Optionally create a registry list for blog entries (like `demos_sidenav.rs`) for navigation/metadata.

## 9) Minimal call chain (TL;DR)

```
MD file
  → read_md_file_typed (api_read_md.rs)
  → MdFile::parse_md_file (md_file.rs)
  → LeptosConverter (leptos_converter.rs)
  → MyMd (static_md_registry.rs)
  → rendered Leptos view
```
