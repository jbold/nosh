# Nosh Specification v1.0

**Version**: 1.0
**Date**: 2026-02-08
**License**: MIT
**Status**: Draft

## 1. Introduction

Nosh is an open standard for machine-readable companion files that sit
alongside human-readable web content. A `.nosh` file contains the structured
knowledge of a web page in JSON format, enabling AI agents to consume page
content directly without parsing HTML.

**Design philosophy**: One JSON file per page. Four required fields. No build
steps required for producers. No query languages, negotiation protocols, or
runtime dependencies.

### 1.1 Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

### 1.2 File Format

- **File extension**: `.nosh`
- **MIME type**: `application/nosh+json`
- **Encoding**: UTF-8
- **Content**: Valid JSON (RFC 8259)
- **Naming convention**: `<page-name>.nosh` placed alongside the HTML file
  (e.g., `my-post.nosh` for `my-post.html`). Sites MAY use alternative
  conventions (e.g., a `/nosh/` directory) as long as discovery mechanisms
  point to the correct URLs.
- **Maximum recommended size**: 1 MB. Validators SHOULD warn (not error) on
  files exceeding this limit.

### 1.3 Goals

1. Enable AI agents to consume web content without HTML parsing
2. Provide typed, structured content that agents can dispatch on
3. Complement (not replace) existing standards like JSON-LD and llms.txt
4. Be simple enough to author by hand in under 10 minutes

---

## 2. Schema Definition

A Nosh file is a JSON object with exactly 4 required top-level fields and up
to 8 optional top-level fields.

### 2.1 Required Fields

| Field     | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| `nosh`    | string | Spec version as MAJOR.MINOR (e.g., `"1.0"`)     |
| `type`    | string | Content type from the taxonomy (Section 3)       |
| `title`   | string | The page's primary heading (non-empty)           |
| `content` | object | Structured knowledge payload (Section 2.3)       |

**FR-001**: Every Nosh file MUST contain all 4 required fields.

**FR-002**: The `nosh` field MUST contain the spec version as a semver
MAJOR.MINOR string (e.g., `"1.0"`).

**FR-003**: The `type` field MUST be a string. Known types are defined in the
content type taxonomy (Section 3). Unknown types MUST be accepted by consumers
with fallback to `article` behavior.

**FR-004**: The `title` field MUST be a non-empty string representing the
page's primary heading.

### 2.2 Optional Fields

| Field         | Type            | Description                          |
|---------------|-----------------|--------------------------------------|
| `description` | string          | Brief summary of the page            |
| `url`         | string (URI)    | Canonical URL of the companion page  |
| `language`    | string          | BCP 47 language tag (e.g., `"en-US"`)|
| `authors`     | array\<string\> | Author names                         |
| `published`   | string          | ISO 8601 date of original publish    |
| `updated`     | string          | ISO 8601 date of last update         |
| `tags`        | array\<string\> | Topic tags for categorization        |
| `related`     | array\<string\> | URLs to related resources            |

**FR-006**: All optional fields are OPTIONAL. A valid Nosh file MAY omit all
of them.

### 2.3 The `content` Object

The `content` field MUST be a JSON object. It MUST always include a `body`
field.

**FR-005**: The `body` field MUST be either a string or an array of strings,
and MUST be non-empty. It contains the page's main text content.

The remaining fields within `content` vary by content type (Section 3). Each
content type documents which additional fields are expected.

### 2.4 Nesting Depth

**FR-007**: Required fields MUST NOT have nesting deeper than 3 levels.
Example: `root.content.steps[].text` is 3 levels and acceptable.

**FR-008**: Optional fields MAY nest to 4 levels maximum.

### 2.5 Minimal Example

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

This file is under 200 bytes and contains all required fields.

---

## 3. Content Type Taxonomy

**FR-009**: The Nosh specification defines 10 content types. Each type
determines the expected shape of the `content` object.

**FR-014**: The taxonomy is extensible. Custom types are permitted but not
guaranteed to be understood by all agents. Consumers MUST treat unknown types
as `article` (Section 3.1).

### 3.1 `article` (default/fallback)

**FR-011**: The simplest content type. Serves as the default when the `type`
is unknown to the consumer.

**Content fields:**

| Field  | Type                      | Required | Description       |
|--------|---------------------------|----------|-------------------|
| `body` | string \| array\<string\> | yes      | Main text content |

```json
{
  "nosh": "1.0",
  "type": "article",
  "title": "Understanding Web Standards",
  "content": {
    "body": "Web standards are the foundation of the open web..."
  }
}
```

### 3.2 `tutorial`

**FR-012**: Supports structured step-by-step instructions.

**Content fields:**

| Field           | Type            | Required | Description            |
|-----------------|-----------------|----------|------------------------|
| `body`          | string          | yes      | Overview text          |
| `prerequisites` | array\<string\> | no       | What's needed first    |
| `steps`         | array\<Step\>   | no       | Ordered instructions   |
| `duration`      | string          | no       | Estimated time         |

**Step object**: `{ "title": string, "text": string }`

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
      { "title": "Start a container", "text": "Run: docker run -p 80:80 nginx" }
    ],
    "duration": "15 minutes"
  }
}
```

### 3.3 `api-reference`

**FR-013**: Supports structured API endpoint documentation.

**Content fields:**

| Field       | Type              | Required | Description      |
|-------------|-------------------|----------|------------------|
| `body`      | string            | yes      | API overview     |
| `base_url`  | string            | no       | API base URL     |
| `endpoints` | array\<Endpoint\> | no       | API endpoints    |

**Endpoint object**: `{ "method": string, "path": string, "description": string }`

```json
{
  "nosh": "1.0",
  "type": "api-reference",
  "title": "Users API",
  "content": {
    "body": "The Users API manages user accounts.",
    "base_url": "https://api.example.com/v1",
    "endpoints": [
      { "method": "GET", "path": "/users", "description": "List all users" },
      { "method": "POST", "path": "/users", "description": "Create a user" }
    ]
  }
}
```

### 3.4 `product`

**Content fields:**

| Field      | Type            | Required | Description           |
|------------|-----------------|----------|-----------------------|
| `body`     | string          | yes      | Product description   |
| `price`    | string          | no       | Price with currency   |
| `currency` | string          | no       | ISO 4217 currency code|
| `features` | array\<string\> | no       | Key features list     |

### 3.5 `recipe`

**Content fields:**

| Field         | Type            | Required | Description          |
|---------------|-----------------|----------|----------------------|
| `body`        | string          | yes      | Recipe description   |
| `ingredients` | array\<string\> | no       | Ingredient list      |
| `steps`       | array\<Step\>   | no       | Cooking instructions |
| `prep_time`   | string          | no       | Preparation time     |
| `cook_time`   | string          | no       | Cooking time         |
| `servings`    | string          | no       | Number of servings   |

**Step object**: `{ "title": string, "text": string }`

### 3.6 `faq`

**Content fields:**

| Field       | Type        | Required | Description           |
|-------------|-------------|----------|-----------------------|
| `body`      | string      | yes      | FAQ overview          |
| `questions` | array\<QA\> | no       | Question-answer pairs |

**QA object**: `{ "question": string, "answer": string }`

### 3.7 `changelog`

**Content fields:**

| Field     | Type           | Required | Description        |
|-----------|----------------|----------|--------------------|
| `body`    | string         | yes      | Changelog overview |
| `entries` | array\<Entry\> | no       | Version entries    |

**Entry object**: `{ "version": string, "date": string, "changes": array<string> }`

### 3.8 `dataset`

**Content fields:**

| Field    | Type           | Required | Description           |
|----------|----------------|----------|-----------------------|
| `body`   | string         | yes      | Dataset description   |
| `format` | string         | no       | Data format (CSV, etc)|
| `fields` | array\<Field\> | no       | Data field definitions|
| `rows`   | integer        | no       | Approximate row count |

**Field object**: `{ "name": string, "type": string, "description": string }`

### 3.9 `event`

**Content fields:**

| Field       | Type   | Required | Description           |
|-------------|--------|----------|-----------------------|
| `body`      | string | yes      | Event description     |
| `date`      | string | no       | ISO 8601 event date   |
| `location`  | string | no       | Venue or "Online"     |
| `organizer` | string | no       | Organizer name        |

### 3.10 `profile`

**Content fields:**

| Field   | Type            | Required | Description       |
|---------|-----------------|----------|-------------------|
| `body`  | string          | yes      | Bio or description|
| `name`  | string          | no       | Full name         |
| `role`  | string          | no       | Title or role     |
| `links` | array\<string\> | no       | Profile URLs      |

---

## 4. Discovery Mechanisms

Sites MUST be able to advertise Nosh support so agents can find `.nosh` files
without prior knowledge of a site's structure.

**FR-020**: Discovery mechanisms have the following priority order (agents
MUST attempt in this order, using the first successful result):

1. `/.well-known/nosh` (highest priority)
2. `robots.txt` directive
3. `llms.txt` references
4. HTML `<link>` tag (lowest priority)

**FR-019**: Discovery mechanisms MUST NOT require HTML parsing or JavaScript
execution. The `<link>` tag is the sole exception and is OPTIONAL (SHOULD,
not MUST).

### 4.1 `.well-known/nosh` Endpoint

**FR-015**: A site MAY serve a JSON manifest at `/.well-known/nosh` listing
all `.nosh` file URLs. This is the RECOMMENDED primary discovery mechanism.

The manifest MUST conform to the Nosh manifest schema:

```json
{
  "nosh": "1.0",
  "pages": [
    {
      "url": "/blog/my-post.nosh",
      "type": "article",
      "title": "My Blog Post"
    },
    {
      "url": "/docs/getting-started.nosh",
      "type": "tutorial",
      "title": "Getting Started"
    }
  ]
}
```

### 4.2 `robots.txt` Directive

**FR-016**: A site MAY include a `Nosh` directive in `robots.txt` pointing
to the site-level manifest:

```
User-agent: *
Allow: /

Nosh: /.well-known/nosh
```

### 4.3 `llms.txt` Integration

**FR-017**: A site MAY reference `.nosh` file URLs in its `llms.txt` file.
Nosh URLs SHOULD be listed under a clearly labeled section:

```
# Nosh Files
- /blog/my-post.nosh
- /docs/getting-started.nosh
```

### 4.4 HTML `<link>` Tag

**FR-018**: A page SHOULD include a `<link>` tag in the HTML `<head>` for
per-page discovery:

```html
<link rel="nosh" type="application/nosh+json" href="my-post.nosh">
```

This is a progressive enhancement — agents that parse HTML can discover
individual `.nosh` files page-by-page.

### 4.5 Agent Discovery Algorithm

An agent discovering Nosh files on a site SHOULD follow this procedure:

1. Fetch `/.well-known/nosh`. If it returns a valid manifest, use it.
2. Else, fetch `/robots.txt` and look for a `Nosh:` directive. If found,
   fetch the referenced manifest URL.
3. Else, fetch `/llms.txt` and look for `.nosh` URL references.
4. Else, when visiting individual pages, check for
   `<link rel="nosh">` tags.

---

## 5. Versioning Strategy

**FR-021**: The Nosh specification uses semantic versioning. The `nosh` field
in files uses MAJOR.MINOR format (e.g., `"1.0"`, `"1.1"`, `"2.0"`).

**FR-022**: MAJOR version bumps (e.g., 1.x → 2.0) indicate
backward-incompatible changes: new required fields, removed fields, or type
changes to existing fields.

**FR-023**: MINOR version bumps (e.g., 1.0 → 1.1) indicate
backward-compatible additions: new optional fields, new content types.

**FR-024**: Files MUST be forward-compatible within a major version. A
consumer supporting Nosh 1.0 MUST be able to read a Nosh 1.x file by
ignoring unknown fields.

### 5.1 Consumer Behavior

- A consumer supporting Nosh 1.0 that encounters a Nosh 1.3 file MUST
  process it, ignoring any fields it does not recognize.
- A consumer that encounters a Nosh 2.0 file (unsupported major version)
  SHOULD attempt best-effort processing and emit a warning.
- Validators MUST warn (not error) when a file targets a newer spec version
  than the validator supports.

---

## 6. Relationship to Existing Standards

**FR-025**: Nosh occupies a specific niche — structured page-level content
delivery for AI agents. It is designed to complement, not compete with,
existing standards.

**FR-026**: Nosh MUST NOT require producers or consumers to implement any
other standard as a prerequisite.

| Standard    | Relationship to Nosh                                     |
|-------------|----------------------------------------------------------|
| llms.txt    | Nosh is the content; llms.txt is the directory. They     |
|             | are complementary layers — llms.txt indexes a site's     |
|             | resources, Nosh provides the structured knowledge.       |
| JSON-LD     | Complementary. JSON-LD provides entity metadata and      |
|             | relationships (schema.org types). Nosh provides the      |
|             | actual page knowledge. Overlap in titles/descriptions    |
|             | is expected and not a conflict.                          |
| schema.org  | Nosh MAY reference schema.org types in an optional       |
|             | `@type` field for interoperability, but MUST NOT require |
|             | schema.org compliance. (FR-027)                          |
| robots.txt  | Nosh uses robots.txt as a discovery vector (Section 4.2).|
|             | The `Nosh:` directive is additive — it does not modify   |
|             | crawling semantics.                                      |
| RSS/Atom    | Nosh is per-page structured content, not a feed. RSS     |
|             | provides chronological content summaries; Nosh provides  |
|             | full structured knowledge per page.                      |
| OpenAPI     | Nosh describes page content; OpenAPI describes APIs.     |
|             | Nosh's `api-reference` type bridges them — it provides   |
|             | endpoint documentation in the Nosh format for agent      |
|             | consumption without requiring the full OpenAPI spec.     |

---

## 7. Conformance Requirements

### 7.1 Producer Conformance

A conforming Nosh producer:

- MUST generate files with all 4 required fields (FR-001)
- MUST use a valid spec version in the `nosh` field (FR-002)
- MUST include a non-empty `body` in the `content` object (FR-005)
- MUST NOT exceed 3 levels of nesting for required fields (FR-007)
- SHOULD use content types from the defined taxonomy (FR-009)
- SHOULD provide at least one discovery mechanism (FR-015–FR-018)
- SHOULD keep files under 1 MB

### 7.2 Consumer Conformance

A conforming Nosh consumer:

- MUST accept files with unknown `type` values, falling back to `article`
  behavior (FR-003, FR-014)
- MUST ignore unknown fields in files targeting the same major version
  (FR-024)
- SHOULD attempt discovery in priority order (FR-020)
- SHOULD NOT require HTML parsing for discovery (FR-019)

### 7.3 Validator Conformance

A conforming Nosh validator:

- MUST validate against the published JSON Schema (FR-030)
- MUST accept files with unknown `type` values (FR-003)
- MUST warn (not error) on files exceeding 1 MB
- MUST warn (not error) on files targeting a newer spec version
- SHOULD support a strict mode that checks content-type-specific fields

---

## 8. JSON Schema

The formal JSON Schema for Nosh files is published at:

- **Nosh file schema**: `nosh.schema.json` (JSON Schema draft 2020-12)
- **Manifest schema**: `manifest.schema.json` (JSON Schema draft 2020-12)

These schemas are the normative machine-readable definition of this
specification. In case of conflict between the prose specification and the
JSON Schema, the JSON Schema takes precedence for validation purposes.

---

## 9. Licensing

**FR-031**: This specification document is released under the MIT License.

**FR-032**: All reference implementations (validator CLI, Zola template) are
released under the MIT License.

Third-party implementations MAY use any license. Proprietary implementations
and extensions are explicitly permitted.

---

## Appendix A: Complete Example

A fully-populated tutorial Nosh file:

```json
{
  "nosh": "1.0",
  "type": "tutorial",
  "title": "Getting Started with Nosh",
  "description": "Learn how to create your first .nosh companion file.",
  "url": "https://nosh.example.com/docs/getting-started",
  "language": "en",
  "authors": ["Nosh Contributors"],
  "published": "2026-02-08",
  "tags": ["nosh", "getting-started", "tutorial"],
  "content": {
    "body": "This tutorial walks you through creating your first .nosh file.",
    "prerequisites": [
      "A published web page (HTML)",
      "A text editor",
      "Basic understanding of JSON"
    ],
    "steps": [
      {
        "title": "Create the file",
        "text": "Create a new file named page.nosh alongside your HTML file."
      },
      {
        "title": "Add required fields",
        "text": "Add the four required fields: nosh, type, title, and content with a body field."
      },
      {
        "title": "Add optional metadata",
        "text": "Optionally add description, url, language, authors, published, tags, and related."
      },
      {
        "title": "Validate",
        "text": "Run the validator CLI: nosh validate page.nosh"
      }
    ],
    "duration": "10 minutes"
  }
}
```

## Appendix B: Edge Cases

- **Unknown content type**: Validators MUST accept it with a warning.
  Consumers MUST treat it as `article`.
- **Multiple discovery mechanisms**: Agents MUST prefer `.well-known/nosh`
  as the canonical source, falling back through the priority order.
- **File size > 1 MB**: Validators MUST warn, not error.
- **Newer spec version**: Validators MUST attempt validation using the
  latest schema they know and report a warning.
- **No meaningful content**: A minimal Nosh file with `type: "article"` and
  a brief `content.body` is valid. Not every page needs a `.nosh` file.
- **Overlap with JSON-LD**: Expected and correct. Nosh provides the knowledge
  payload; JSON-LD provides entity metadata.
