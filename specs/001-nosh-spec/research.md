# Research: Nosh Specification v1.0

**Date**: 2026-02-08
**Branch**: `001-nosh-spec`

## 1. Zola JSON Output Generation

**Decision**: Use Zola's Tera template engine with `json_encode` filter
to generate nosh.json companion files alongside HTML pages.

**Rationale**: Zola natively supports non-HTML output through custom
templates. The Tera engine provides `{{ page | json_encode(pretty=true)
| safe }}` for structured JSON output. Zola already generates XML
(Atom/RSS feeds, sitemaps) and robots.txt, demonstrating multi-format
capability. Front matter fields pass directly to templates, enabling
Nosh metadata to be embedded in Markdown content files.

**Alternatives considered**:
- Post-processing script (separate Rust/Python tool transforms HTML to
  JSON during build) — rejected: adds build complexity, violates
  Simplicity First principle
- Hugo/Jekyll — rejected: require external plugins for JSON; Zola's
  unified Tera system is simpler
- Static JSON files without templating — rejected: no DRY, no content
  reuse, high maintenance burden

**Key Tera features for Nosh**:
- `json_encode` filter with `pretty` parameter
- `map`, `filter`, `sort` filters for data manipulation
- Serde-compatible serialization for complex types
- Template inheritance for shared JSON structure

## 2. Rust JSON Schema Validation

**Decision**: Use `jsonschema` crate (jsonschema-rs) as the validation
library, supporting JSON Schema draft 2020-12 natively. Build a
Nosh-specific CLI wrapping this library rather than using the generic
`jsonschema-cli` directly.

**Rationale**: The `jsonschema` crate explicitly supports draft 2020-12
with full specification compliance, high performance, and comprehensive
error reporting. Building a Nosh-specific CLI (rather than wrapping the
generic `jsonschema-cli`) allows Nosh-specific features: content-type
validation, `--strict` mode for type-specific field checks, directory
scanning, and custom error messages referencing Nosh spec sections.

**Alternatives considered**:
- `boon` crate — viable alternative with multi-draft support, smaller
  community. Considered as fallback if `jsonschema` has issues.
- `jsonschema-cli` (generic) — rejected as the end product: too generic,
  no Nosh-specific validation (content type shapes, discovery manifest
  format). Used as reference for CLI UX patterns.
- Node.js with `ajv` — rejected: user preference for Rust. Rust produces
  single-binary distribution, no runtime dependency.

**CLI design**:
- `nosh validate <file|dir>` — validate against Nosh schema
- `nosh validate --strict` — also check content-type-specific fields
- Exit code 0 (valid), 1 (invalid), 2 (error)
- JSON and human-readable output modes

## 3. Well-Known URI Strategy

**Decision**: Define `/.well-known/nosh` in the spec as the primary
discovery mechanism. Do not pursue IANA registration immediately; follow
the llms.txt precedent of pragmatic deployment with retroactive
registration once adoption warrants it.

**Rationale**: RFC 8615 technically requires IANA registration for
`.well-known` URIs, but the process requires a specification document
and expert review (~2-3 months). Projects like llms.txt have deployed
without registration and gained wide adoption. The pragmatic approach
is to define the endpoint, gain adoption, then register. The Nosh spec
will document the intent to register and the IANA submission process.

**Alternatives considered**:
- Root-level `/nosh-manifest.json` only — simpler but doesn't follow
  the `.well-known` convention that agents increasingly check
- Formal IANA registration before launch — rejected: blocks
  deployment on bureaucratic timeline, no precedent needed given
  llms.txt established the pattern
- Both `.well-known/nosh` and root-level fallback — accepted: the spec
  defines `.well-known/nosh` as primary and `robots.txt` directive as
  secondary, giving flexibility

## 4. Spec Document Format

**Decision**: Write the Nosh specification as Markdown, hosted on a
Zola-built website. The spec site itself uses Nosh (dogfooding).

**Rationale**: Markdown is universally readable, version-controllable
in git, and can be rendered to a polished website via Zola. Using the
Nosh Zola template on the spec site itself demonstrates the reference
implementation and proves it works.

**Alternatives considered**:
- RFC-style plain text (IETF format) — rejected: poor readability,
  targets a different audience (standards bodies vs. web developers)
- HTML-only spec — rejected: harder to contribute to, no git-friendly
  diffing
- GitHub wiki — rejected: not self-hosted, no custom styling, no
  dogfooding opportunity

## 5. Content Type Shape Design

**Decision**: Each content type defines a fixed set of expected fields
within the `content` object. The `body` field is always required. Other
fields are type-specific and optional (but documented). The `article`
type has the simplest shape (just `body`).

**Rationale**: Fixed shapes per type let agents dispatch on `type` and
know what fields to expect. Making type-specific fields optional (with
documentation) balances strictness with flexibility — producers can
include as much or as little structure as their content supports.

**Alternatives considered**:
- Fully rigid per-type schemas (all fields required) — rejected:
  too rigid for real-world content that varies in structure
- No per-type shapes (just `body` for everything) — rejected:
  undermines Agent-Native principle; agents can't dispatch on type
- Union/discriminated type schema — considered but deferred: adds
  JSON Schema complexity. Simpler to document shapes per type.
