<!--
Sync Impact Report
===================
Version change: N/A (initial) → 1.0.0
Modified principles: N/A (initial creation)
Added sections:
  - 7 Core Principles (I–VII)
  - Standards Alignment section
  - Licensing & Openness section
  - Governance section with amendment procedure
Removed sections: N/A
Templates requiring updates:
  - .specify/templates/plan-template.md — ✅ No updates needed
    (Constitution Check section is generic; will be filled per-feature)
  - .specify/templates/spec-template.md — ✅ No updates needed
    (Spec template is domain-agnostic; constitution principles
    apply via the plan's Constitution Check gate)
  - .specify/templates/tasks-template.md — ✅ No updates needed
    (Task categorization is feature-driven, not principle-driven)
  - .specify/templates/agent-file-template.md — ✅ No updates needed
    (Generated from plans, not constitution directly)
Follow-up TODOs: None
-->

# Nosh Constitution

## Core Principles

### I. Simplicity First

Every design decision MUST favor the simplest viable approach.
One JSON file per page. No build steps required for producers.
No query languages, no negotiation protocols, no runtime dependencies.

- The specification MUST be implementable by a single developer
  in an afternoon using only a text editor.
- If a feature cannot be explained in one paragraph, it is too
  complex and MUST be simplified or deferred.
- New optional fields MUST NOT increase the burden on minimal
  implementations.

### II. Complementary, Not Competing

Nosh occupies a specific niche: structured page-level content
delivery for AI agents. It MUST NOT attempt to replace or
duplicate existing standards.

- `llms.txt` is the directory; Nosh is the content. These are
  complementary layers, not alternatives.
- JSON-LD and schema.org provide metadata and entity relationships;
  Nosh provides the actual structured knowledge payload.
- Implementations SHOULD reference and interoperate with existing
  standards rather than reinventing their capabilities.
- The spec MUST explicitly document the boundary between Nosh
  and adjacent standards.

### III. Human-Readable

The JSON format MUST be immediately comprehensible to a developer
reading the raw file without tooling or documentation.

- Field names MUST be self-documenting English words or
  widely-understood abbreviations.
- Nesting depth MUST NOT exceed 3 levels for required fields.
- The specification MUST include inline examples for every
  content type.
- No encoded, compressed, or binary content within nosh.json files.

### IV. Content-Forward

A nosh.json file MUST contain the actual structured knowledge of
the page, not merely metadata about it.

- Required content MUST include substantive information: steps,
  findings, data points, prerequisites, parameters, return values —
  whatever constitutes the page's core knowledge.
- Metadata (author, dates, tags) is secondary; the primary payload
  is always the knowledge itself.
- An agent reading only the nosh.json file MUST be able to answer
  the same questions a human could answer by reading the HTML page.

### V. Permissively Licensed

The Nosh specification and all reference implementations MUST be
released under permissive open-source licenses (MIT, Apache 2.0,
or equivalent).

- No contributor license agreement (CLA) required beyond the
  license itself.
- The specification text MUST be freely redistributable and
  modifiable.
- Proprietary implementations and extensions are explicitly
  permitted and encouraged.

### VI. Agent-Native

The schema MUST be designed for how AI agents actually consume
information: key-value pairs, typed fields, structured lists, and
predictable shapes.

- Every field MUST have a well-defined type (string, array, object)
  documented in the specification.
- Content types MUST use a fixed taxonomy so agents can dispatch
  on type without heuristics.
- Arrays MUST be preferred over prose where information is
  inherently list-like (steps, prerequisites, parameters).
- The schema MUST be validatable with standard JSON Schema tooling.

### VII. Discoverable

Sites MUST be able to advertise Nosh support through standard
discovery mechanisms so agents can find nosh.json files without
prior knowledge of a site's structure.

- The specification MUST define at least one discovery mechanism
  (e.g., `robots.txt`, `llms.txt`, `.well-known/nosh`).
- Discovery MUST NOT require parsing HTML or executing JavaScript.
- The specification SHOULD define a `<link>` tag for in-page
  discovery as a progressive enhancement.

## Standards Alignment

Nosh exists within a broader ecosystem of web standards and
agent-facing protocols. The following boundaries apply:

| Standard       | Nosh Relationship                              |
|----------------|-------------------------------------------------|
| llms.txt       | Nosh is the content; llms.txt is the directory  |
| JSON-LD        | Complementary: JSON-LD for entities, Nosh for   |
|                | page knowledge                                  |
| schema.org     | Nosh MAY reference schema.org types but MUST NOT |
|                | require schema.org compliance                   |
| robots.txt     | Nosh uses robots.txt as a discovery vector       |
| RSS/Atom       | Nosh is per-page structured content, not a feed  |
| OpenAPI        | Nosh describes page content; OpenAPI describes   |
|                | APIs. Nosh's `api-reference` type bridges them   |

Implementations MUST NOT introduce dependencies on standards not
listed above without a constitution amendment.

## Licensing & Openness

- The Nosh specification is released under the MIT License.
- Reference implementations are released under the MIT License.
- Third-party implementations MAY use any license.
- The specification repository MUST accept community contributions
  via standard pull request workflows.
- Breaking changes to the spec MUST follow the versioning strategy
  defined in the specification itself.

## Governance

This constitution is the highest-authority document for the Nosh
project. All specifications, implementations, and contributions
MUST comply with its principles.

**Amendment Procedure:**

1. Proposed amendments MUST be submitted as pull requests with
   a rationale section explaining the change.
2. Amendments that add, remove, or redefine a Core Principle
   require a MAJOR version bump to this constitution.
3. Amendments that expand guidance within existing principles
   require a MINOR version bump.
4. Clarifications, typos, and non-semantic changes require a
   PATCH version bump.
5. All amendments MUST update the `Last Amended` date.

**Compliance:**

- All pull requests to the specification MUST cite which
  principles they satisfy or are neutral to.
- If a proposed spec change conflicts with a principle, the
  principle wins unless the constitution is amended first.
- The Constitution Check gate in implementation plans MUST
  verify compliance with all 7 principles before work begins.

**Version**: 1.0.0 | **Ratified**: 2026-02-08 | **Last Amended**: 2026-02-08
