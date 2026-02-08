# Feature Specification: Nosh Specification v1.0

**Feature Branch**: `001-nosh-spec`
**Created**: 2026-02-08
**Status**: Draft
**Input**: User description: "Create the Nosh specification — an open standard for machine-readable companion files (nosh.json) that sit alongside human-readable web content. RSS for the agentic web."

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Author a nosh.json File (Priority: P1)

A web developer has a blog post (or tutorial, API doc, product page, etc.)
and wants AI agents to consume its content without parsing HTML. The
developer creates a `nosh.json` companion file alongside the page,
filling in required fields (nosh version, content type, title, content
body) and optional fields (tags, prerequisites, related links). The
developer validates the file against the JSON Schema using the validator
CLI, confirms it passes, and deploys it alongside the HTML page.

**Why this priority**: Without producers creating nosh.json files,
the entire ecosystem has no content. This is the foundational action
that makes everything else possible.

**Independent Test**: Can be tested by writing a nosh.json file by hand
in a text editor, running the validator, and confirming the file is
well-formed and contains structured knowledge equivalent to the HTML
page.

**Acceptance Scenarios**:

1. **Given** a developer has a published blog post at `/blog/my-post.html`,
   **When** the developer creates `/blog/my-post.nosh.json` with the
   required fields (`nosh`, `type`, `title`, `content`),
   **Then** the file validates successfully against the Nosh JSON Schema.

2. **Given** a developer writes a nosh.json with only the 4 required
   fields,
   **When** the validator checks it,
   **Then** it passes with zero errors and zero warnings.

3. **Given** a developer opens a valid nosh.json in a text editor,
   **When** the developer reads the raw JSON,
   **Then** the content and structure are immediately understandable
   without consulting documentation.

4. **Given** a developer writes a nosh.json for a tutorial page,
   **When** the developer uses `type: "tutorial"` and provides `steps`
   in the content body,
   **Then** the structured steps contain the same information a human
   would get from reading the HTML page.

---

### User Story 2 — Discover and Consume nosh.json as an AI Agent (Priority: P2)

An AI agent is browsing a website to answer a user's question. The agent
checks the site's discovery mechanisms (robots.txt, llms.txt, .well-known,
or HTML link tag) and finds that the site supports Nosh. The agent
fetches the nosh.json file for the relevant page, parses the structured
content, and uses the typed fields directly — no HTML parsing or content
extraction needed.

**Why this priority**: Consumption is the demand side of the ecosystem.
Agents must be able to find and use nosh.json files reliably, or
producers have no incentive to create them.

**Independent Test**: Can be tested by setting up a static site with
nosh.json files and discovery entries, then writing a script that
follows the discovery protocol and fetches/parses the structured content.

**Acceptance Scenarios**:

1. **Given** a site has a `robots.txt` entry `Nosh: /nosh-manifest.json`,
   **When** an agent reads robots.txt,
   **Then** the agent can locate the manifest listing all nosh.json URLs.

2. **Given** a site has an `llms.txt` file referencing nosh.json URLs,
   **When** an agent reads llms.txt,
   **Then** the agent can follow the references to individual nosh.json
   files.

3. **Given** a page has `<link rel="nosh" href="page.nosh.json">` in
   the HTML head,
   **When** an agent parses the page head (without full HTML parsing),
   **Then** the agent locates the companion nosh.json file.

4. **Given** a site has `/.well-known/nosh` returning a JSON manifest,
   **When** an agent fetches that well-known URL,
   **Then** the agent receives a list of all nosh.json URLs on the site.

5. **Given** an agent fetches a valid nosh.json file,
   **When** the agent reads the `type` field,
   **Then** the agent can dispatch to a type-specific handler without
   heuristics (e.g., "tutorial" means steps exist, "api-reference"
   means endpoints exist).

---

### User Story 3 — Generate nosh.json via a Zola Template (Priority: P3)

A Zola-based static site owner wants to automatically generate nosh.json
files for every page during the build process. The owner installs the
Nosh Zola template, configures their `config.toml` with site-level
metadata, and adds per-page Nosh front matter to their Markdown content.
On `zola build`, a nosh.json companion file is generated for each page.

**Why this priority**: Automated generation lowers the barrier to
adoption dramatically. Manual authoring (US1) proves the spec works;
automated generation proves it scales.

**Independent Test**: Can be tested by creating a minimal Zola site with
the Nosh template, writing 2–3 pages with Nosh front matter, running
`zola build`, and verifying each page has a valid companion nosh.json.

**Acceptance Scenarios**:

1. **Given** a Zola site with the Nosh template installed and a
   Markdown page with Nosh front matter,
   **When** the site owner runs `zola build`,
   **Then** a valid nosh.json file is generated alongside the HTML output.

2. **Given** a Zola page with `type = "tutorial"` in the Nosh front
   matter and a Markdown body containing numbered steps,
   **When** the build runs,
   **Then** the generated nosh.json contains the steps as a structured
   array in the `content.steps` field.

3. **Given** a site with 50 pages and Nosh front matter on all of them,
   **When** the site owner runs `zola build`,
   **Then** 50 valid nosh.json files are generated, plus a site-level
   manifest at `/.well-known/nosh`.

---

### User Story 4 — Validate a nosh.json File (Priority: P4)

A developer or CI pipeline wants to validate that nosh.json files
conform to the Nosh specification. The user runs the validator CLI
tool, pointing it at a single file or a directory of files. The
validator reports any schema violations, missing required fields, or
content-type-specific issues.

**Why this priority**: Validation tooling ensures ecosystem quality
and gives producers confidence their files are correct. It is essential
for CI integration but depends on having the schema defined first (P1).

**Independent Test**: Can be tested by running the validator against
a set of known-good and known-bad nosh.json files and verifying it
correctly accepts/rejects each.

**Acceptance Scenarios**:

1. **Given** a valid nosh.json file,
   **When** the user runs `nosh validate file.nosh.json`,
   **Then** the tool exits with code 0 and prints "Valid".

2. **Given** a nosh.json file missing the required `title` field,
   **When** the user runs `nosh validate file.nosh.json`,
   **Then** the tool exits with code 1 and prints an error message
   identifying the missing field and its expected type.

3. **Given** a directory containing 10 nosh.json files (8 valid, 2
   invalid),
   **When** the user runs `nosh validate ./content/`,
   **Then** the tool reports the 2 invalid files with specific errors
   and exits with code 1.

4. **Given** a nosh.json file with `type: "tutorial"` but no `steps`
   in the content body,
   **When** the user runs `nosh validate --strict file.nosh.json`,
   **Then** the tool warns that tutorials are expected to have steps.

---

### User Story 5 — Publish the Nosh Specification Website (Priority: P5)

The Nosh project maintainer publishes the specification as a readable
website. The site contains the full spec text, JSON Schema downloads,
content type examples, a quickstart guide, and links to reference
implementations. The site itself serves as a living example of Nosh
by publishing its own nosh.json files.

**Why this priority**: A published spec website is necessary for
community adoption but depends on the spec content being finalized
(P1, P2) and the Zola template being functional (P3).

**Independent Test**: Can be tested by building and deploying the spec
site, verifying all pages render, and confirming the site's own
nosh.json files validate.

**Acceptance Scenarios**:

1. **Given** the spec content is finalized in Markdown,
   **When** the maintainer runs `zola build`,
   **Then** the spec site is generated with all sections, examples,
   and downloadable JSON Schema files.

2. **Given** the spec site is deployed,
   **When** a visitor navigates to the content type taxonomy page,
   **Then** each content type has a complete example nosh.json file
   inline.

3. **Given** the spec site is deployed,
   **When** an AI agent checks the site's `/.well-known/nosh`,
   **Then** the agent discovers nosh.json files for every spec page —
   the site practices what it preaches.

---

### Edge Cases

- What happens when a nosh.json file references a `type` not in the
  official taxonomy? The validator MUST accept it with a warning
  (forward-compatibility). Agents MUST treat unknown types as generic
  `article` type.

- What happens when a site has both `/.well-known/nosh` and a
  `robots.txt` Nosh directive? Agents MUST prefer `.well-known/nosh`
  as the canonical source (more structured), falling back to
  robots.txt if .well-known returns 404.

- What happens when a nosh.json file is larger than 1 MB? The spec
  MUST recommend a maximum file size of 1 MB. Validators MUST warn
  (not error) on files exceeding this limit.

- What happens when the `nosh` version field in a file is newer than
  the validator supports? The validator MUST attempt validation using
  the latest schema it knows and report a warning that the file
  targets a newer spec version.

- What happens when a page has no meaningful structured content
  (e.g., a "Contact Us" page with just an email address)? A valid
  minimal nosh.json with `type: "article"` and a brief `content.body`
  is acceptable. Not every page needs a nosh.json — it is optional.

- What happens when the same content exists in both the nosh.json and
  JSON-LD on the page? This is expected and correct. Nosh provides the
  knowledge payload; JSON-LD provides entity metadata. Overlap in
  titles and descriptions is normal and not a conflict.

## Requirements *(mandatory)*

### Functional Requirements

**Schema Definition**

- **FR-001**: The spec MUST define a JSON object schema with exactly 4
  required top-level fields: `nosh` (version string), `type` (content
  type string), `title` (string), and `content` (object).

- **FR-002**: The `nosh` field MUST contain the spec version as a
  semver string (e.g., `"1.0"`) indicating which version of the Nosh
  specification the file conforms to.

- **FR-003**: The `type` field MUST be a string from the defined
  content type taxonomy. Unknown types MUST be accepted by consumers
  with fallback to `article` behavior.

- **FR-004**: The `title` field MUST be a human-readable string
  representing the page's primary heading.

- **FR-005**: The `content` field MUST be an object containing the
  page's structured knowledge. Its shape varies by content type but
  MUST always include at least a `body` field (string or array of
  strings).

- **FR-006**: The spec MUST define the following optional top-level
  fields: `description` (string), `url` (canonical URL string),
  `language` (BCP 47 language tag), `authors` (array of strings),
  `published` (ISO 8601 date string), `updated` (ISO 8601 date
  string), `tags` (array of strings), `related` (array of URL strings).

- **FR-007**: Required fields MUST NOT have nesting deeper than 3
  levels (e.g., `root.content.steps[].text` is 3 levels and
  acceptable; a 4th level is prohibited for required fields).

- **FR-008**: Optional fields MAY nest to 4 levels maximum.

**Content Type Taxonomy**

- **FR-009**: The spec MUST define a fixed taxonomy of content types:
  `article`, `tutorial`, `api-reference`, `product`, `recipe`, `faq`,
  `changelog`, `dataset`, `event`, `profile`.

- **FR-010**: Each content type MUST document its expected `content`
  object shape, including which sub-fields are required vs. optional
  for that type.

- **FR-011**: The `article` type MUST serve as the default/fallback
  type with the simplest content shape (just `body`).

- **FR-012**: The `tutorial` type MUST support a `steps` array where
  each step has `title` (string) and `text` (string).

- **FR-013**: The `api-reference` type MUST support an `endpoints`
  array where each endpoint has `method` (string), `path` (string),
  and `description` (string).

- **FR-014**: The content type taxonomy MUST be extensible — custom
  types are permitted but not guaranteed to be understood by all
  agents.

**Discovery Mechanisms**

- **FR-015**: The spec MUST define a `.well-known/nosh` endpoint
  that returns a JSON manifest listing all nosh.json URLs on the site.

- **FR-016**: The spec MUST define a `robots.txt` directive format:
  `Nosh: <url-to-manifest>` pointing to the site-level manifest.

- **FR-017**: The spec MUST define an `llms.txt` integration pattern
  where nosh.json URLs can be listed or referenced from llms.txt
  entries.

- **FR-018**: The spec SHOULD define an HTML `<link>` tag format:
  `<link rel="nosh" type="application/json" href="page.nosh.json">`
  for per-page discovery as progressive enhancement.

- **FR-019**: Discovery mechanisms MUST NOT require HTML parsing or
  JavaScript execution. The `<link>` tag is the sole exception, and
  it is optional (SHOULD, not MUST).

- **FR-020**: The spec MUST define a priority order for discovery:
  `.well-known/nosh` > `robots.txt` > `llms.txt` > `<link>` tag.

**Versioning Strategy**

- **FR-021**: The Nosh specification MUST use semantic versioning
  (MAJOR.MINOR format for the `nosh` field in files, with PATCH
  reserved for errata).

- **FR-022**: MAJOR version bumps indicate backward-incompatible
  schema changes (new required fields, removed fields, type changes).

- **FR-023**: MINOR version bumps indicate backward-compatible
  additions (new optional fields, new content types).

- **FR-024**: Files MUST be forward-compatible: a consumer supporting
  Nosh 1.0 MUST be able to read a Nosh 1.x file by ignoring unknown
  fields.

**Standards Relationship**

- **FR-025**: The spec MUST include a "Relationship to Existing
  Standards" section documenting boundaries with llms.txt, JSON-LD,
  schema.org, robots.txt, RSS/Atom, and OpenAPI.

- **FR-026**: Nosh MUST NOT require producers or consumers to
  implement any other standard as a prerequisite.

- **FR-027**: The spec SHOULD document how nosh.json content can
  reference schema.org types in a `@type` optional field for
  interoperability, without requiring it.

**Reference Implementation**

- **FR-028**: The project MUST produce a Zola template/plugin as the
  first reference implementation for generating nosh.json files from
  Markdown content with front matter.

- **FR-029**: The project MUST produce a validator CLI tool that
  checks nosh.json files against the JSON Schema.

- **FR-030**: The project MUST publish a formal JSON Schema file
  (draft 2020-12 or later) that can be used by any JSON Schema
  validator.

**Licensing**

- **FR-031**: The specification document MUST be released under the
  MIT License.

- **FR-032**: All reference implementations MUST be released under
  the MIT License.

### Key Entities

- **Nosh File**: A JSON file (typically `*.nosh.json`) conforming to
  the Nosh schema. Contains structured knowledge for a single web
  page. Has exactly 4 required fields (`nosh`, `type`, `title`,
  `content`) and up to 8 optional top-level fields.

- **Content Type**: A string from the fixed taxonomy that determines
  the expected shape of the `content` object. Enables agents to
  dispatch on type without heuristics. 10 defined types plus support
  for custom types.

- **Discovery Manifest**: A JSON file at `/.well-known/nosh` listing
  all nosh.json URLs on a site. Contains an array of objects, each
  with `url` (string), `type` (content type), and `title` (string).

- **Nosh Schema**: A formal JSON Schema document defining the
  validation rules for nosh.json files. Published as a downloadable
  file and used by the validator CLI.

### Assumptions

- The naming convention for companion files is `<page-name>.nosh.json`
  placed alongside the HTML file. Sites MAY use a different convention
  (e.g., a `/nosh/` directory) as long as discovery mechanisms point
  to the correct URLs.

- The initial content type taxonomy of 10 types covers the vast
  majority of web content. Additional types can be added in MINOR
  version bumps without breaking existing files.

- The `.well-known/nosh` manifest is a flat JSON array, not paginated.
  Sites with extremely large numbers of pages (>10,000) may need a
  pagination extension in a future spec version.

- BCP 47 is used for language tags (e.g., `"en"`, `"en-US"`,
  `"ja"`) as this is the web standard for language identification.

- ISO 8601 is used for date fields (e.g., `"2026-02-08"` or
  `"2026-02-08T14:30:00Z"`) as this is the unambiguous machine-
  readable date standard.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer with no prior Nosh knowledge can create a
  valid nosh.json file by hand in under 10 minutes using only the
  spec document as reference.

- **SC-002**: An AI agent can discover all nosh.json files on a
  Nosh-enabled site in a single HTTP request (via `.well-known/nosh`
  or robots.txt manifest).

- **SC-003**: The minimal valid nosh.json file is under 200 bytes,
  demonstrating low barrier to entry.

- **SC-004**: The Zola reference implementation generates valid
  nosh.json files for 100% of pages with Nosh front matter, with
  zero manual post-processing required.

- **SC-005**: The validator CLI correctly identifies 100% of schema
  violations in a test suite of at least 50 known-bad files.

- **SC-006**: The Nosh JSON Schema is accepted without modification
  by at least 3 mainstream JSON Schema validators (e.g., ajv,
  jsonschema, serde_json).

- **SC-007**: The spec website serves as a live example: every page
  has a companion nosh.json file that validates successfully, and the
  site's `.well-known/nosh` endpoint is functional.

- **SC-008**: Community adoption reaches at least 5 independent
  implementations (producers or consumers) within 6 months of spec
  publication.
