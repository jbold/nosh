# Nosh Zola Template

Generate [Nosh](../spec/nosh-spec.md) companion files (`.nosh`) and a discovery manifest from your [Zola](https://www.getzola.org/) site.

## Quick start

1. Copy `templates/nosh.html`, `templates/nosh-manifest.html`, and the `content/.well-known/` directory into your Zola project.

2. Add Nosh settings to your `config.toml`:

```toml
[extra]
nosh_enabled = true
nosh_version = "1.0"
```

3. For each page that should have a `.nosh` companion, add `nosh_*` fields to the page's `[extra]` front matter and create a companion `.nosh.md` content file.

## Per-page setup

Every page that should emit a `.nosh` file needs two content files:

### 1. The HTML page (`blog/hello-world.md`)

```markdown
+++
title = "Hello World"
date = 2026-02-08

[extra]
nosh_type = "article"
nosh_description = "My first post."
nosh_language = "en"
nosh_authors = ["Jane Developer"]
nosh_published = "2026-02-08"
nosh_tags = ["hello", "nosh"]
nosh_content = '{"body": "Full text content of the page."}'
+++

Page content here (rendered as HTML).
```

### 2. The `.nosh` companion (`blog/hello-world.nosh.md`)

```markdown
+++
title = "Hello World"
date = 2026-02-08
template = "nosh.html"
slug = "hello-world.nosh"

[extra]
nosh_type = "article"
nosh_description = "My first post."
nosh_language = "en"
nosh_authors = ["Jane Developer"]
nosh_published = "2026-02-08"
nosh_tags = ["hello", "nosh"]
nosh_content = '{"body": "Full text content of the page."}'
+++
```

Key points:
- Set `template = "nosh.html"` to use the Nosh JSON template
- Set `slug = "<page-slug>.nosh"` so the output path ends in `.nosh`
- Mirror the `[extra]` fields from the HTML page

### HTML `<link>` tag

The `page.html` template automatically adds a `<link>` discovery tag when `nosh_type` is set:

```html
<link rel="alternate" type="application/nosh+json" href="/blog/hello-world.nosh">
```

## Discovery manifest

The `/.well-known/nosh` manifest is generated from `content/.well-known/nosh/index.md`. It lists every page that has `nosh_type` set in its front matter.

Output:

```json
{
  "nosh": "1.0",
  "pages": [
    {
      "url": "https://example.com/blog/hello-world.nosh",
      "type": "article",
      "title": "Hello World"
    }
  ]
}
```

## `[extra]` front matter reference

| Field | Required | Description |
|---|---|---|
| `nosh_type` | Yes | Content type: `article`, `tutorial`, `recipe`, etc. |
| `nosh_content` | Yes | JSON string with the `content` object (must include `body`) |
| `nosh_description` | No | Brief page summary |
| `nosh_language` | No | BCP 47 language tag (e.g. `en`) |
| `nosh_authors` | No | Array of author names |
| `nosh_published` | No | ISO 8601 publication date |
| `nosh_updated` | No | ISO 8601 last-update date |
| `nosh_tags` | No | Array of topic tags |
| `nosh_related` | No | Array of related URLs |

## Content types

The `nosh_content` JSON string must always include a `body` field. Type-specific fields vary — see the [Nosh specification](../spec/nosh-spec.md) for the full taxonomy.

Example for a recipe:

```toml
nosh_content = '''
{
  "body": "A classic margherita pizza recipe.",
  "ingredients": ["dough", "tomato sauce", "mozzarella", "basil"],
  "steps": [
    {"title": "Prep dough", "text": "Stretch the dough into a circle."},
    {"title": "Add toppings", "text": "Spread sauce, add cheese and basil."},
    {"title": "Bake", "text": "Bake at 475°F for 12 minutes."}
  ],
  "servings": "4"
}
'''
```

## File structure

```
zola-template/
├── config.toml                           # Zola config with nosh_* extras
├── content/
│   ├── _index.md                         # Site root section
│   ├── .well-known/
│   │   └── nosh/
│   │       └── index.md                  # Discovery manifest content page
│   └── blog/
│       ├── _index.md                     # Blog section
│       ├── hello-world.md                # HTML page
│       └── hello-world.nosh.md           # .nosh companion generator
├── templates/
│   ├── base.html                         # Base HTML template
│   ├── index.html                        # Home page template
│   ├── section.html                      # Section listing template
│   ├── page.html                         # Page template (adds <link> tag)
│   ├── nosh.html                         # .nosh JSON output template
│   └── nosh-manifest.html                # Discovery manifest template
└── README.md
```

## Building

```sh
zola build
```

Output in `public/`:
```
public/
├── blog/
│   ├── hello-world/
│   │   └── index.html
│   └── hello-world.nosh/
│       └── index.html          # Contains .nosh JSON
├── .well-known/
│   └── nosh/
│       └── index.html          # Contains manifest JSON
└── index.html
```

Configure your web server to serve `hello-world.nosh/index.html` with `Content-Type: application/nosh+json` and the `.well-known/nosh/index.html` with `Content-Type: application/json`.

Example nginx configuration:

```nginx
location ~ \.nosh/ {
    default_type application/nosh+json;
}

location /.well-known/nosh/ {
    default_type application/json;
}
```

## Validating output

After building, validate your generated `.nosh` files with the Nosh validator:

```sh
# Validate a single file
nosh validate public/blog/hello-world.nosh/index.html

# Validate all .nosh output (rename index.html files first)
find public -path "*.nosh/index.html" -exec sh -c \
  'cp "$1" "${1%/index.html}.nosh.json"' _ {} \;
nosh validate public/
```

## License

MIT
