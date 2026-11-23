# Extension points

This document defines how future entry points should integrate without expanding the initial CLI scope.

## Desktop or web interface

Add a separate application entry point under `apps/` when a real UI is planned. It should call `zashikura-application` and compose the platform adapters and, when persistence is required, storage adapters it needs. It must not duplicate discovery, policy, or maintenance logic.

A web interface must not imply a remote service or telemetry requirement. Local execution and local data ownership remain the defaults.

## Language bindings and wrappers

Add bindings under `bindings/` only when a specific host language or wrapper is selected. Bindings should expose a deliberately designed API, not the internal implementation details of every crate.

Prefer a narrow, stable facade over exposing domain internals. Define ownership, error conversion, threading, and lifecycle behavior before publishing a binding API.

## MCP integration

Add an MCP entry point under `integrations/mcp/` when MCP support is implemented. The MCP server should translate tool requests into application use cases and translate results into MCP responses.

The MCP layer must not implement its own system discovery or maintenance logic. Tools that change system state must be separately identified, narrowly scoped, and require explicit user authorization. Read-only tools should be available independently of write-capable tools.

## Automation and other protocols

Place protocol-specific entry points under `integrations/`. Keep protocol schemas, transport dependencies, authentication, and serialization at that boundary. Do not make the application layer depend on a specific protocol.

## Adding an extension

Before adding a new workspace member:

1. Identify the concrete user-facing capability it provides.
2. Define which application use cases it needs.
3. Keep protocol and presentation types at the boundary.
4. Reuse existing platform adapters and any storage adapters already justified by concrete persistence requirements.
5. Add tests for authorization, errors, and data handling.
6. Document platform support and any new permissions.

Do not create empty crates or add dependencies solely to reserve a future feature.
