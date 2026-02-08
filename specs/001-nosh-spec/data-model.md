# Data Model: Nosh Specification v1.0

**Date**: 2026-02-08
**Branch**: `001-nosh-spec`

## Entity: Nosh File

A single nosh.json file representing structured knowledge for one web
page.

### Required Fields (top-level)

| Field     | Type   | Description                                    |
|-----------|--------|------------------------------------------------|
| `nosh`    | string | Spec version (semver MAJOR.MINOR, e.g., "1.0") |
| `type`    | string | Content type from taxonomy                     |
| `title`   | string | Page's primary heading                         |
| `content` | object | Structured knowledge payload (see below)       |

### Optional Fields (top-level)

| Field         | Type            | Description                          |
|---------------|-----------------|--------------------------------------|
| `description` | string          | Brief summary of the page            |
| `url`         | string          | Canonical URL of the companion page  |
| `language`    | string          | BCP 47 language tag (e.g., "en-US")  |
| `authors`     | array\<string\> | Author names                         |
| `published`   | string          | ISO 8601 date of original publish    |
| `updated`     | string          | ISO 8601 date of last update         |
| `tags`        | array\<string\> | Topic tags for categorization        |
| `related`     | array\<string\> | URLs to related resources            |

### Nesting Depth Rules

- Required fields: max 3 levels (root → content → field → leaf)
- Optional fields: max 4 levels

## Entity: Content Object (per type)

The `content` field shape varies by `type`. `body` is always required.

### article (default/fallback)

| Field  | Type                    | Required | Description      |
|--------|-------------------------|----------|------------------|
| `body` | string \| array\<string\> | yes      | Main text content |

### tutorial

| Field           | Type              | Required | Description          |
|-----------------|-------------------|----------|----------------------|
| `body`          | string            | yes      | Overview text        |
| `prerequisites` | array\<string\>   | no       | What's needed first  |
| `steps`         | array\<Step\>     | no       | Ordered instructions |
| `duration`      | string            | no       | Estimated time       |

**Step**: `{ "title": string, "text": string }`

### api-reference

| Field       | Type              | Required | Description          |
|-------------|-------------------|----------|----------------------|
| `body`      | string            | yes      | API overview         |
| `base_url`  | string            | no       | API base URL         |
| `endpoints` | array\<Endpoint\> | no       | API endpoints        |

**Endpoint**: `{ "method": string, "path": string, "description": string }`

### product

| Field      | Type              | Required | Description             |
|------------|-------------------|----------|-------------------------|
| `body`     | string            | yes      | Product description     |
| `price`    | string            | no       | Price with currency      |
| `currency` | string            | no       | ISO 4217 currency code   |
| `features` | array\<string\>   | no       | Key features list        |

### recipe

| Field          | Type              | Required | Description             |
|----------------|-------------------|----------|-------------------------|
| `body`         | string            | yes      | Recipe description      |
| `ingredients`  | array\<string\>   | no       | Ingredient list          |
| `steps`        | array\<Step\>     | no       | Cooking instructions     |
| `prep_time`    | string            | no       | Preparation time         |
| `cook_time`    | string            | no       | Cooking time             |
| `servings`     | string            | no       | Number of servings       |

### faq

| Field       | Type              | Required | Description             |
|-------------|-------------------|----------|-------------------------|
| `body`      | string            | yes      | FAQ overview            |
| `questions` | array\<QA\>       | no       | Question-answer pairs    |

**QA**: `{ "question": string, "answer": string }`

### changelog

| Field      | Type              | Required | Description              |
|------------|-------------------|----------|--------------------------|
| `body`     | string            | yes      | Changelog overview       |
| `entries`  | array\<Entry\>    | no       | Version entries           |

**Entry**: `{ "version": string, "date": string, "changes": array<string> }`

### dataset

| Field      | Type              | Required | Description              |
|------------|-------------------|----------|--------------------------|
| `body`     | string            | yes      | Dataset description      |
| `format`   | string            | no       | Data format (CSV, etc.)  |
| `fields`   | array\<Field\>    | no       | Data field descriptions   |
| `rows`     | number            | no       | Approximate row count     |

**Field**: `{ "name": string, "type": string, "description": string }`

### event

| Field       | Type   | Required | Description               |
|-------------|--------|----------|---------------------------|
| `body`      | string | yes      | Event description         |
| `date`      | string | no       | ISO 8601 event date       |
| `location`  | string | no       | Venue or "Online"         |
| `organizer` | string | no       | Organizer name             |

### profile

| Field    | Type              | Required | Description               |
|----------|-------------------|----------|---------------------------|
| `body`   | string            | yes      | Bio or description        |
| `name`   | string            | no       | Full name                 |
| `role`   | string            | no       | Title or role             |
| `links`  | array\<string\>   | no       | Profile URLs              |

## Entity: Discovery Manifest

The JSON document served at `/.well-known/nosh`.

### Top-level Structure

| Field     | Type                  | Required | Description              |
|-----------|-----------------------|----------|--------------------------|
| `nosh`    | string                | yes      | Spec version ("1.0")     |
| `pages`   | array\<ManifestEntry\> | yes      | List of nosh.json URLs  |

### ManifestEntry

| Field   | Type   | Required | Description                 |
|---------|--------|----------|-----------------------------|
| `url`   | string | yes      | URL to the nosh.json file   |
| `type`  | string | yes      | Content type                |
| `title` | string | yes      | Page title                  |

## Entity: Nosh JSON Schema

A JSON Schema (draft 2020-12) document that formally defines the
validation rules for nosh.json files. Published as a downloadable
`.schema.json` file.

- Top-level schema validates required/optional fields and types
- Uses `if`/`then` conditional schemas for content-type-specific
  validation (e.g., if `type == "tutorial"` then `content` MAY
  contain `steps` with specific shape)
- The schema file itself is versioned alongside the spec

## Relationships

```
Nosh File  ──references──>  Content Type (via `type` field)
Nosh File  ──conforms-to──> Nosh JSON Schema
Manifest   ──lists──>       Nosh File URLs
Manifest   ──conforms-to──> Nosh JSON Schema (manifest sub-schema)
```
