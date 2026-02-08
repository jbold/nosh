# Tasks: Nosh Specification v1.0

**Input**: Design documents from `/specs/001-nosh-spec/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Test tasks are included where the spec mandates validation (US4: validator CLI, SC-005: test suite of ≥50 known-bad files). Other stories rely on manual verification per their acceptance scenarios.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **spec/**: Specification document, JSON Schemas, examples
- **validator/**: Rust CLI crate
- **zola-template/**: Zola reference implementation

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize all three project directories and shared tooling

- [ ] T001 Create top-level project structure: `spec/`, `validator/`, `zola-template/` directories per plan.md
- [ ] T002 Initialize Rust project with `cargo init --name nosh` in `validator/` with dependencies: `jsonschema`, `clap`, `serde`, `serde_json` in `validator/Cargo.toml`
- [ ] T003 [P] Create `spec/examples/` directory for per-content-type example files
- [ ] T004 [P] Add MIT LICENSE file at repository root
- [ ] T005 [P] Create `.gitignore` with Rust (`target/`), Zola (`public/`), and OS entries

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: The JSON Schema and spec document are the foundation everything else depends on — the validator validates against the schema, the Zola template generates files that conform to it, and examples must pass it.

**CRITICAL**: No user story work can begin until these tasks are complete.

- [ ] T006 Write the Nosh JSON Schema (draft 2020-12) defining required fields (`nosh`, `type`, `title`, `content`), optional fields, and type enum in `spec/nosh.schema.json` — use data-model.md as source of truth for field names, types, and nesting constraints (FR-001 through FR-008)
- [ ] T007 Add `if`/`then` conditional sub-schemas for each content type's `content` shape in `spec/nosh.schema.json` — `article` (body only), `tutorial` (body + steps + prerequisites + duration), `api-reference` (body + base_url + endpoints), `product`, `recipe`, `faq`, `changelog`, `dataset`, `event`, `profile` per data-model.md (FR-009 through FR-014)
- [ ] T008 Write the discovery manifest JSON Schema in `spec/manifest.schema.json` — validates `nosh` (version string) and `pages` array of `{url, type, title}` entries per data-model.md Discovery Manifest entity
- [ ] T009 Write the Nosh specification document in `spec/nosh-spec.md` — sections: Introduction, Schema Definition (required/optional fields), Content Type Taxonomy (10 types with shapes), Discovery Mechanisms (4 mechanisms with priority order), Versioning Strategy (semver MAJOR.MINOR), Relationship to Existing Standards (llms.txt, JSON-LD, schema.org, robots.txt, RSS/Atom, OpenAPI), Conformance Requirements (FR-001 through FR-032)

**Checkpoint**: Schema and spec document complete — all downstream work can reference them.

---

## Phase 3: User Story 1 — Author a nosh.json File (Priority: P1) MVP

**Goal**: A developer can create a valid nosh.json file by hand using the spec as reference, with inline examples for every content type.

**Independent Test**: Write a nosh.json by hand in a text editor, open the raw JSON, confirm it is immediately understandable without documentation. Validate against the schema using any JSON Schema tool.

### Implementation for User Story 1

- [ ] T010 [P] [US1] Create minimal article example in `spec/examples/article.nosh.json` — 4 required fields only, under 200 bytes (SC-003), matching `contracts/nosh-file-minimal.json`
- [ ] T011 [P] [US1] Create tutorial example in `spec/examples/tutorial.nosh.json` — include `steps` array, `prerequisites`, `duration` per data-model.md tutorial shape
- [ ] T012 [P] [US1] Create api-reference example in `spec/examples/api-reference.nosh.json` — include `endpoints` array with `method`, `path`, `description` per data-model.md
- [ ] T013 [P] [US1] Create product example in `spec/examples/product.nosh.json` — include `price`, `currency`, `features` per data-model.md
- [ ] T014 [P] [US1] Create recipe example in `spec/examples/recipe.nosh.json` — include `ingredients`, `steps`, `prep_time`, `cook_time`, `servings` per data-model.md
- [ ] T015 [P] [US1] Create faq example in `spec/examples/faq.nosh.json` — include `questions` array of `{question, answer}` per data-model.md
- [ ] T016 [P] [US1] Create changelog example in `spec/examples/changelog.nosh.json` — include `entries` array of `{version, date, changes}` per data-model.md
- [ ] T017 [P] [US1] Create dataset example in `spec/examples/dataset.nosh.json` — include `format`, `fields`, `rows` per data-model.md
- [ ] T018 [P] [US1] Create event example in `spec/examples/event.nosh.json` — include `date`, `location`, `organizer` per data-model.md
- [ ] T019 [P] [US1] Create profile example in `spec/examples/profile.nosh.json` — include `name`, `role`, `links` per data-model.md
- [ ] T020 [US1] Validate all 10 examples against `spec/nosh.schema.json` using any JSON Schema validator (e.g., `jsonschema-cli` or online tool) — all must pass with zero errors

**Checkpoint**: A developer can hand-author a nosh.json for any of the 10 content types using examples as reference. SC-001 (under 10 minutes) and SC-003 (under 200 bytes minimal) are verifiable.

---

## Phase 4: User Story 2 — Discover and Consume nosh.json as an AI Agent (Priority: P2)

**Goal**: An AI agent can discover all nosh.json files on a site via any of the 4 discovery mechanisms and parse the structured content by dispatching on `type`.

**Independent Test**: Set up a static file server with example nosh.json files and all 4 discovery mechanisms. Write a script that follows the discovery priority order and fetches/parses each nosh.json.

### Implementation for User Story 2

- [ ] T021 [P] [US2] Create discovery manifest example at `spec/examples/discovery-manifest.json` — list all 10 example nosh.json URLs, validate against `spec/manifest.schema.json`
- [ ] T022 [P] [US2] Create robots.txt discovery example at `spec/examples/discovery-robots.txt` — include `Nosh: /.well-known/nosh` directive per FR-016
- [ ] T023 [P] [US2] Create llms.txt integration example at `spec/examples/discovery-llms.txt` — show nosh.json URL references per FR-017
- [ ] T024 [P] [US2] Create HTML link tag example at `spec/examples/discovery-link-tag.html` — `<link rel="nosh" type="application/json" href="...">` per FR-018
- [ ] T025 [US2] Add Discovery Mechanisms section to `spec/nosh-spec.md` with priority order (`.well-known/nosh` > `robots.txt` > `llms.txt` > `<link>` tag), agent algorithm pseudocode, and all 4 example references (FR-015 through FR-020)

**Checkpoint**: The spec document and examples fully define how agents discover nosh.json files. SC-002 (single HTTP request via .well-known/nosh) is demonstrable.

---

## Phase 5: User Story 3 — Generate nosh.json via Zola Template (Priority: P3)

**Goal**: A Zola site owner adds Nosh front matter to Markdown files and gets valid nosh.json companion files on `zola build`.

**Independent Test**: Create a minimal Zola site with the Nosh template, write 2-3 pages with Nosh front matter, run `zola build`, verify each page has a valid companion nosh.json and the site has a `.well-known/nosh` manifest.

### Implementation for User Story 3

- [ ] T026 [US3] Create Zola Nosh page template in `zola-template/templates/nosh.json` — Tera template that reads `page.extra.nosh.*` front matter fields and outputs a valid nosh.json (use `json_encode` filter per research.md decision 1)
- [ ] T027 [US3] Create Zola discovery manifest template in `zola-template/templates/well-known-nosh.json` — Tera template that iterates all site pages with Nosh front matter and outputs the `/.well-known/nosh` manifest
- [ ] T028 [US3] Create example Zola config in `zola-template/config.toml` — configure `[extra.nosh]` section for site-level Nosh metadata and output paths
- [ ] T029 [P] [US3] Create example Markdown content page in `zola-template/content/blog/getting-started.md` — include Nosh front matter (`type = "tutorial"`, `tags`, etc.) and Markdown body with numbered steps
- [ ] T030 [P] [US3] Create example Markdown content page in `zola-template/content/blog/my-first-post.md` — include Nosh front matter (`type = "article"`) and Markdown body
- [ ] T031 [US3] Create Zola template README in `zola-template/README.md` — installation instructions, front matter reference, build instructions, and example output
- [ ] T032 [US3] Build the example Zola site with `zola build` in `zola-template/`, verify nosh.json outputs are generated alongside HTML and validate them against `spec/nosh.schema.json`

**Checkpoint**: Zola template generates valid nosh.json for every page with Nosh front matter. SC-004 (100% of pages, zero post-processing) is verifiable.

---

## Phase 6: User Story 4 — Validate a nosh.json File (Priority: P4)

**Goal**: A developer or CI pipeline can validate nosh.json files against the Nosh schema using a single-binary CLI tool.

**Independent Test**: Run the validator against the 10 known-good examples (all pass) and a set of known-bad files (all correctly rejected with specific error messages).

### Test Fixtures for User Story 4

- [ ] T033 [P] [US4] Create ≥10 valid test fixtures in `validator/tests/fixtures/valid/` — copy all 10 content type examples from `spec/examples/`
- [ ] T034 [P] [US4] Create ≥50 invalid test fixtures in `validator/tests/fixtures/invalid/` — cover: missing required fields (nosh, type, title, content), wrong field types, unknown type with --strict, nesting depth violations, empty content.body, invalid semver in nosh field, malformed JSON, >1MB file size, missing content.body, invalid date formats, invalid BCP 47 tags (SC-005)

### Implementation for User Story 4

- [ ] T035 [US4] Implement JSON Schema loading and validation logic in `validator/src/schema.rs` — embed `spec/nosh.schema.json` at compile time via `include_str!`, expose `validate(json: &Value) -> Vec<ValidationError>` function using `jsonschema` crate (research.md decision 2)
- [ ] T036 [US4] Implement content-type-specific strict validation in `validator/src/content_types.rs` — for `--strict` mode: verify type-specific fields exist (e.g., tutorial has steps, api-reference has endpoints), emit warnings (not errors) for missing optional-but-expected fields
- [ ] T037 [US4] Implement output formatting in `validator/src/output.rs` — human-readable mode (colored, with file path and line references) and JSON mode (`--json` flag) for CI integration
- [ ] T038 [US4] Implement library exports in `validator/src/lib.rs` — re-export `schema::validate`, `content_types::strict_validate`, `output::format_results` for use as a library
- [ ] T039 [US4] Implement CLI entry point in `validator/src/main.rs` — `nosh validate <file|dir>` with `--strict`, `--json`, `--quiet` flags using `clap`; exit code 0 (valid), 1 (invalid), 2 (error); directory mode recursively finds `*.nosh.json` files
- [ ] T040 [US4] Write unit tests in `validator/tests/validate_test.rs` — test all valid fixtures pass, all invalid fixtures fail with expected error types, strict mode warns on missing type-specific fields
- [ ] T041 [US4] Write CLI integration tests in `validator/tests/cli_test.rs` — test exit codes, human-readable output format, JSON output format, directory scanning, `--strict` flag behavior
- [ ] T042 [US4] Run `cargo test` and `cargo build --release` — all tests pass, release binary builds successfully, verify single-binary with no runtime dependencies

**Checkpoint**: Validator CLI correctly validates all fixtures. SC-005 (100% of schema violations detected in ≥50 known-bad files) and SC-006 (schema accepted by mainstream validators) are verifiable.

---

## Phase 7: User Story 5 — Publish the Nosh Specification Website (Priority: P5)

**Goal**: The spec is published as a readable Zola-built website that dogfoods Nosh by serving its own nosh.json files.

**Independent Test**: Build and serve the spec site, verify all pages render, confirm the site's own nosh.json files validate, and verify `/.well-known/nosh` is functional.

### Implementation for User Story 5

- [ ] T043 [US5] Create spec site Zola project in `site/` — `config.toml` with site metadata, Nosh template from `zola-template/`, and base theme/styling
- [ ] T044 [US5] Convert `spec/nosh-spec.md` into Zola content pages in `site/content/` — split spec into navigable sections: Introduction, Schema, Content Types, Discovery, Versioning, Standards Relationship, Conformance
- [ ] T045 [P] [US5] Create a content type taxonomy page in `site/content/content-types.md` — inline example nosh.json for each of the 10 types from `spec/examples/`
- [ ] T046 [P] [US5] Create a quickstart page in `site/content/quickstart.md` — adapt from `specs/001-nosh-spec/quickstart.md`
- [ ] T047 [P] [US5] Create a downloads page in `site/content/downloads.md` — links to `nosh.schema.json`, `manifest.schema.json`, and validator CLI releases
- [ ] T048 [US5] Add Nosh front matter to all site content pages — every page gets `[extra.nosh]` with appropriate `type` and metadata
- [ ] T049 [US5] Build site with `zola build` in `site/`, verify all pages render, all nosh.json companion files validate against schema, and `/.well-known/nosh` lists all pages

**Checkpoint**: Spec website is self-hosting Nosh. SC-007 (every page has companion nosh.json, .well-known/nosh is functional) is verifiable.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final quality pass across all deliverables

- [ ] T050 [P] Review `spec/nosh-spec.md` for completeness against all 32 functional requirements (FR-001 through FR-032) — add any missing normative language (MUST, SHOULD, MAY per RFC 2119)
- [ ] T051 [P] Add a CONTRIBUTING.md at repository root — contribution guidelines, spec amendment process (from constitution), and development setup
- [ ] T052 [P] Create a top-level README.md — project overview, link to spec, quickstart, validator usage, Zola template usage
- [ ] T053 Run quickstart.md validation — follow every step in `specs/001-nosh-spec/quickstart.md` as a new user and verify all steps work end-to-end
- [ ] T054 Final constitution compliance check — re-verify all 7 principles against delivered artifacts (spec, schema, examples, validator, Zola template, site)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — BLOCKS all user stories
- **Phase 3 (US1: Author)**: Depends on Phase 2 (needs schema to validate against)
- **Phase 4 (US2: Discover)**: Depends on Phase 2 (needs spec document structure)
- **Phase 5 (US3: Zola)**: Depends on Phase 2 (needs schema) — can parallel with US1/US2
- **Phase 6 (US4: Validator)**: Depends on Phase 2 (needs schema) + Phase 3 (needs valid examples as fixtures)
- **Phase 7 (US5: Website)**: Depends on Phase 2 (needs spec), Phase 3 (needs examples), Phase 5 (needs Zola template)
- **Phase 8 (Polish)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (Author)**: Depends on Foundational only — first to implement (MVP)
- **US2 (Discover)**: Depends on Foundational only — can parallel with US1
- **US3 (Zola)**: Depends on Foundational only — can parallel with US1/US2
- **US4 (Validator)**: Depends on US1 (needs valid example files as test fixtures)
- **US5 (Website)**: Depends on US1 (examples) + US3 (Zola template) — implement last

### Within Each User Story

- Schema/foundation before examples
- Examples before validation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T003, T004, T005 can all run in parallel (Phase 1)
- T006, T007, T008 are sequential (schema builds incrementally)
- T010–T019 can ALL run in parallel (10 independent example files)
- T021–T024 can ALL run in parallel (4 independent discovery examples)
- T029, T030 can run in parallel (independent Markdown content pages)
- T033, T034 can run in parallel (valid and invalid fixture sets)
- T045, T046, T047 can run in parallel (independent site pages)
- T050, T051, T052 can run in parallel (independent documents)
- US1, US2, US3 can all run in parallel after Phase 2

---

## Parallel Example: User Story 1

```text
# All 10 example files can be created simultaneously:
Task: "Create article example in spec/examples/article.nosh.json"
Task: "Create tutorial example in spec/examples/tutorial.nosh.json"
Task: "Create api-reference example in spec/examples/api-reference.nosh.json"
Task: "Create product example in spec/examples/product.nosh.json"
Task: "Create recipe example in spec/examples/recipe.nosh.json"
Task: "Create faq example in spec/examples/faq.nosh.json"
Task: "Create changelog example in spec/examples/changelog.nosh.json"
Task: "Create dataset example in spec/examples/dataset.nosh.json"
Task: "Create event example in spec/examples/event.nosh.json"
Task: "Create profile example in spec/examples/profile.nosh.json"
```

## Parallel Example: User Story 4

```text
# Test fixtures and implementation can overlap:
# Step 1 - fixtures in parallel:
Task: "Create valid test fixtures in validator/tests/fixtures/valid/"
Task: "Create invalid test fixtures in validator/tests/fixtures/invalid/"

# Step 2 - implementation modules in parallel (after schema.rs):
Task: "Implement content_types.rs"
Task: "Implement output.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T005)
2. Complete Phase 2: Foundational — JSON Schema + spec document (T006–T009)
3. Complete Phase 3: US1 — 10 content type examples (T010–T020)
4. **STOP and VALIDATE**: Hand-author a nosh.json, validate against schema
5. Deliverable: Spec document + schema + examples — enough for early adopters

### Incremental Delivery

1. Setup + Foundational → Schema and spec ready
2. Add US1 (Author examples) → Spec is complete and usable (MVP!)
3. Add US2 (Discovery) → Agents can find nosh.json files
4. Add US3 (Zola template) → Automated generation for static sites
5. Add US4 (Validator CLI) → Quality assurance tooling
6. Add US5 (Website) → Public-facing spec site with dogfooding
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers after Phase 2 is complete:

- Developer A: US1 (examples) + US4 (validator, after US1)
- Developer B: US2 (discovery) + US3 (Zola template)
- Developer C: US5 (website, after US1 + US3)

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- The spec document (T009) is a large task — consider splitting during implementation if it exceeds a reasonable single-task scope
- Test fixtures (T034: ≥50 invalid files) is labor-intensive — use systematic generation from a matrix of violation types × content types
