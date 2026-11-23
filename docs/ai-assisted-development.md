# AI-assisted development

Zashikura is designed to be understandable and maintainable by both people and AI coding tools. AI assistance is a development workflow, not a runtime requirement.

## Repository context

Start with:

1. `AGENTS.md` for repository-wide engineering, safety, and privacy rules.
2. `README.md` for product purpose and project structure.
3. `docs/architecture/overview.md` for crate responsibilities and dependency direction.
4. `docs/architecture/extension-points.md` before adding an interface or integration.
5. `docs/decisions/` for recorded architectural decisions.

Then inspect the actual code and tests. Documentation is guidance, not evidence that an unfinished feature exists.

## Design for tool interoperability

- Keep the application API independent of CLI output, UI state, JSON formats, and MCP schemas.
- Give application use cases explicit inputs, typed outputs, and predictable error behavior.
- Keep operating-system and persistence concerns behind application ports.
- Convert application results into JSON, terminal output, UI models, or MCP responses at the relevant boundary.
- Avoid exposing internal Rust types as a permanent external API.
- When external interfaces are introduced, document their schemas, compatibility expectations, and versioning separately.

## AI safety

AI tools must not receive broader system access than the task requires. Read-only capabilities should be separable from capabilities that change system state. Any future AI-facing tool must have a narrow purpose, validated inputs, bounded output, clear permission requirements, and explicit confirmation for consequential actions.

Do not send local machine information to a model or remote service by default. Any future model integration must be optional, transparent, and separately reviewed for privacy, security, cost, and offline behavior.

## Keeping context reliable

When a design decision changes:

1. Update the relevant architecture document.
2. Add or amend an architecture decision record for decisions that affect boundaries or long-term compatibility.
3. Keep `AGENTS.md` focused on durable instructions, not temporary task notes.
4. Remove obsolete guidance instead of leaving conflicting instructions.
5. Ensure examples match the current implementation.

Do not create a second source of truth for architecture. Prefer linking to the canonical document.
