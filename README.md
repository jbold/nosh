# 🍽️ Nosh

**Machine-readable companion files for AI agents. RSS for the agentic web.**

Nosh is an open specification for structured content files that sit alongside human-readable web pages. A `.nosh` file gives AI agents exactly what they need — structured knowledge, typed fields, and clean data — without parsing HTML.

---

## The Problem

AI agents consume the web by scraping HTML. They parse navbars, sidebars, footers, cookie banners, and ad blocks just to find the actual content. Then they guess at the structure.

Meanwhile, the content creator *knows* the structure. They know the steps, the prerequisites, the key findings, the cost data. But they publish it as prose in HTML, and agents have to reverse-engineer it.

## The Fix

Drop a `.nosh` file next to your page. Done.

```
/blog/                             ← index (list of posts, for humans)
/blog/post-title/                  ← the post (for humans)
/blog/post-title.nosh              ← the post (for agents)
```

A `.nosh` file is JSON with a simple schema: 4 required fields, typed content, and room for whatever domain-specific data matters for your page.

---

## Quick Example

```json
{
  "nosh": "1.0",
  "type": "tutorial",
  "title": "Getting Started with Nosh",
  "description": "Create your first .nosh companion file in under 10 minutes.",
  "url": "https://example.com/docs/getting-started",
  "authors": ["Your Name"],
  "published": "2026-02-08",
  "tags": ["nosh", "getting-started"],
  "content": {
    "body": "This tutorial walks you through creating your first .nosh file.",
    "prerequisites": [
      "A published web page",
      "A text editor",
      "Basic understanding of JSON"
    ],
    "steps": [
      {
        "title": "Create the file",
        "text": "Create page-name.nosh alongside your HTML file."
      },
      {
        "title": "Add required fields",
        "text": "Add nosh (version), type, title, and content with at least a body field."
      },
      {
        "title": "Validate",
        "text": "Run: nosh validate page-name.nosh"
      }
    ],
    "duration": "10 minutes"
  }
}
```

An agent reads this and instantly has structured steps, prerequisites, and timing — no HTML parsing, no guessing.

---

## The Schema

### Required Fields

| Field     | Type   | Description                    |
|-----------|--------|--------------------------------|
| `nosh`    | string | Spec version (e.g., `"1.0"`)  |
| `type`    | string | Content type from taxonomy     |
| `title`   | string | Page's primary heading         |
| `content` | object | Structured knowledge (must include `body`) |

### Optional Fields

| Field         | Type     | Description                         |
|---------------|----------|-------------------------------------|
| `description` | string   | Brief summary                       |
| `url`         | string   | Canonical URL of the companion page |
| `language`    | string   | BCP 47 language tag (e.g., `"en"`)  |
| `authors`     | string[] | Author names                        |
| `published`   | string   | ISO 8601 date                       |
| `updated`     | string   | ISO 8601 date of last update        |
| `tags`        | string[] | Topic tags                          |
| `related`     | string[] | URLs to related resources           |

### Content Types

The `type` field determines the expected shape of `content`:

| Type | Use Case | Key Content Fields |
|------|----------|--------------------|
| `article` | Blog posts, news, essays | `body` |
| `tutorial` | How-to guides, walkthroughs | `body`, `steps`, `prerequisites`, `duration` |
| `api-reference` | API documentation | `body`, `endpoints`, `base_url` |
| `product` | Product pages, listings | `body`, `features`, `price` |
| `recipe` | Cooking, DIY instructions | `body`, `ingredients`, `steps`, `prep_time`, `cook_time` |
| `faq` | Q&A pages | `body`, `questions` |
| `changelog` | Release notes, version history | `body`, `entries` |
| `dataset` | Data descriptions | `body`, `fields`, `format`, `rows` |
| `event` | Events, conferences | `body`, `date`, `location`, `organizer` |
| `profile` | People, team pages | `body`, `name`, `role`, `links` |

Custom types are allowed — agents treat unknown types as `article` (fallback).

Content objects accept **additional properties** beyond the defined fields. The typed fields give structure; your domain-specific data rides alongside it.

---

## Discovery

How do agents find your `.nosh` files?

**Priority order:**

1. **`/.well-known/nosh`** — JSON manifest listing all .nosh URLs on your site
2. **`robots.txt`** — `Nosh: https://example.com/.well-known/nosh`
3. **`llms.txt`** — reference .nosh URLs from your llms.txt entries
4. **`<link>` tag** — `<link rel="nosh" type="application/nosh+json" href="page.nosh">`

None of these require JavaScript execution. The `<link>` tag is optional progressive enhancement.

---

## Validator CLI

A Rust-based validator ships with the project.

```bash
# Validate a single file
nosh validate my-post.nosh

# Validate a directory
nosh validate ./content/

# Strict mode (type-specific checks)
nosh validate --strict my-post.nosh

# JSON output (for CI pipelines)
nosh validate --json my-post.nosh
```

### Build from source

```bash
cd validator
cargo build --release
# Binary at target/release/nosh
```

---

## Zola Template

Auto-generate `.nosh` files during `zola build`. See [`zola-template/README.md`](zola-template/README.md) for setup.

---

## How Nosh Relates to Other Standards

| Standard | What It Does | Nosh Relationship |
|----------|-------------|-------------------|
| **llms.txt** | Site-level directory for AI crawlers | llms.txt is the map; nosh is the content |
| **JSON-LD** | Embedded metadata (author, type, schema.org) | JSON-LD describes *what the page is*; nosh contains *what the page knows* |
| **schema.org** | Vocabulary for structured metadata | Nosh MAY reference schema.org types but doesn't require it |
| **robots.txt** | Crawler access control | Nosh uses robots.txt as a discovery vector |
| **RSS/Atom** | Feed of recent content | RSS is a timeline; nosh is per-page structured content |
| **OpenAPI** | API specification | Nosh's `api-reference` type bridges page content to API docs |

Nosh is **complementary** — it fills the gap between "here's my sitemap" (llms.txt) and "here's metadata about this page" (JSON-LD). Nosh says: **here's the actual knowledge, structured and ready.**

---

## Talk Nosh 🗣️

Nosh is a real English word — it means to snack, to munch. You're putting out a little snack for AI agents to consume. They don't need the full meal (your HTML with nav, sidebar, footer) — they just need the nosh.

Use it as a verb:

- **"Did you nosh it?"** — Does this post have a .nosh file?
- **"Nosh your posts"** — Add .nosh files to your content
- **"Is it noshed?"** — Is the content available in nosh format?
- **"Drop a nosh"** — Create a .nosh companion file
- **"That site is noshed up"** — The whole site has .nosh files

> *"I just published that new blog post."*
> *"Oh nice, did you nosh it? You'll get way better pickup in AI search."*

---

## Project Structure

```
spec/                    # The specification
├── nosh-spec.md         # Full spec document (Markdown)
├── nosh.schema.json     # JSON Schema (draft 2020-12)
├── manifest.schema.json # Discovery manifest schema
└── examples/            # One .nosh file per content type

validator/               # Rust CLI tool
├── Cargo.toml
├── src/
└── tests/

zola-template/           # Reference implementation for Zola
├── templates/
├── config.toml
└── README.md
```

---

## File Extension

`.nosh` — not `.nosh.json`.

The content is JSON. The MIME type is `application/nosh+json`. We use a distinct extension because nosh is a protocol, not just another JSON file.

---

## License

MIT — spec, validator, and all reference implementations.

---

## Status

**v1.0 — Draft**

The first `.nosh` file in the wild lives at [bold.casa](https://bold.casa/blog/10-walls-to-1m-context/).

Built by [John Rembold](https://bold.casa) and [Kit](https://github.com/jbold/nosh) 🐾
