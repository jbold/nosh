# Quickstart: Nosh Specification v1.0

## 1. Create a nosh.json file

Create a file named `my-page.nosh.json` alongside your HTML page:

```json
{
  "nosh": "1.0",
  "type": "article",
  "title": "My Blog Post",
  "content": {
    "body": "This is the structured content of my blog post."
  }
}
```

That's it — 4 required fields, under 200 bytes.

## 2. Add optional metadata

Enrich the file with optional fields:

```json
{
  "nosh": "1.0",
  "type": "article",
  "title": "My Blog Post",
  "description": "A deep dive into structured web content.",
  "url": "https://example.com/blog/my-post",
  "language": "en",
  "authors": ["Jane Developer"],
  "published": "2026-02-08",
  "tags": ["web", "ai", "standards"],
  "content": {
    "body": "This is the structured content of my blog post."
  }
}
```

## 3. Use a content type

For a tutorial page, use `type: "tutorial"` and add steps:

```json
{
  "nosh": "1.0",
  "type": "tutorial",
  "title": "Getting Started with Docker",
  "content": {
    "body": "Learn Docker basics in 15 minutes.",
    "prerequisites": ["Docker installed", "Terminal access"],
    "steps": [
      { "title": "Pull an image", "text": "Run: docker pull nginx" },
      { "title": "Start a container", "text": "Run: docker run -p 80:80 nginx" },
      { "title": "Verify", "text": "Open http://localhost in your browser." }
    ],
    "duration": "15 minutes"
  }
}
```

## 4. Validate

Run the validator CLI:

```bash
nosh validate my-page.nosh.json
# Output: ✓ Valid (nosh 1.0, type: article)

nosh validate ./content/
# Validates all .nosh.json files in the directory
```

## 5. Enable discovery

Choose one or more discovery mechanisms:

**Option A: .well-known endpoint** (recommended)

Serve a manifest at `/.well-known/nosh`:

```json
{
  "nosh": "1.0",
  "pages": [
    { "url": "/blog/my-post.nosh.json", "type": "article", "title": "My Blog Post" }
  ]
}
```

**Option B: robots.txt directive**

Add to your `robots.txt`:

```
Nosh: /.well-known/nosh
```

**Option C: HTML link tag** (per-page)

Add to your page's `<head>`:

```html
<link rel="nosh" type="application/json" href="my-post.nosh.json">
```

## 6. Automate with Zola (optional)

Add Nosh front matter to your Markdown files:

```toml
+++
title = "My Blog Post"

[extra.nosh]
type = "article"
tags = ["web", "ai"]
+++

Your Markdown content here...
```

Run `zola build` — nosh.json files are generated automatically.

## Verify

- [ ] nosh.json validates against the schema
- [ ] At least one discovery mechanism is configured
- [ ] An agent can find and parse your nosh.json files
