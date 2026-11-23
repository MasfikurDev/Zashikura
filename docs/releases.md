# Release process

## Release channels

Zashikura uses one Linux packaging pipeline for beta and stable releases.

- **Beta:** Automatically published from qualifying changes merged into `main`. Intended for early public testing and may change frequently.
- **Stable:** Published only through an explicit maintainer-triggered release workflow after the version has been updated and reviewed.
- **Alpha / release candidate:** Supported as manually dispatched pre-releases when a version containing `-alpha.` or `-rc.` is supplied.

A pre-release is a normal downloadable GitHub Release marked as a pre-release. It is not a separate update service.

## Automatic beta release policy

Every push to `main` is inspected, but a release is created only when new product changes are present since the most recent tag.

The beta detector considers Conventional Commit subjects and changed paths:

| Change | Beta release |
| --- | --- |
| `feat:`, `fix:`, `perf:`, `refactor:`, or other product-code changes | Yes |
| Workspace/package manifests or lockfile changes | Yes |
| Documentation-only changes | No |
| README, typo, or Markdown-only changes | No |
| Tests-only changes | No |
| CI workflow-only changes | No |
| Formatting-only changes | No |
| Release workflow changes | No |

Test-, CI-, and style-only commits are excluded, as are `chore(release)` version-only commits. A documentation commit that also changes product code still qualifies. Dependency and packaging changes to Cargo manifests or the lockfile qualify even when committed with a `chore` prefix. Product changes are limited to workspace manifests, Rust source/build files, and the Rust toolchain/build configuration. Mixed commits still qualify when they include a product file.

When no stable tag exists yet, the first beta uses the workspace version as its base (currently `0.1.0-beta.1`). Until the first stable release, subsequent qualifying updates increment the beta sequence for that same base version. After a stable tag exists, version calculation examines qualifying commits since that stable tag:
- `BREAKING CHANGE:` or a breaking Conventional Commit header increments the major version.
- `feat:` increments the minor version.
- Other qualifying product changes increment the patch version.

Each beta is tagged as `vMAJOR.MINOR.PATCH-beta.N`. The beta sequence increments for each qualifying update to that target version. Documentation-only and other excluded commits do not consume beta versions. A beta is built from the exact `main` commit that triggered it.

The workflow is serialized to prevent concurrent beta publishers from selecting the same version. The release pipeline must pass tests, binary smoke tests, package validation, and artifact checks before publishing. Failed builds do not create a release.

## Stable release

1. Update the workspace version in the root `Cargo.toml` and commit the intended release version to `main`.
2. Ensure the repository tracks a current `Cargo.lock`. The repository did not track one when this pipeline was introduced; add it before the first stable release.
3. Confirm CI is green and manually run the **Release** workflow from `main`.
4. Enter the exact workspace version without a leading `v` (for example, `0.1.0`). The stable workflow validates that it matches the CLI package version.
5. The workflow builds and verifies all artifacts, then creates the stable GitHub Release. It does not publish if any build or validation job fails.

Stable publication is intentionally an explicit maintainer action. Beta publication is automatic.

## Supported release artifacts

Every release currently builds Linux packages on native Linux runners for both supported architectures:

| Architecture | Debian package | RPM package | GNU archive | Static standalone binary and archive |
| --- | --- | --- | --- | --- |
| x86_64 | `.deb` (amd64) | `.rpm` (x86_64) | `.tar.gz` | Raw musl binary and `-musl.tar.gz` |
| aarch64 | `.deb` (arm64) | `.rpm` (aarch64) | `.tar.gz` | Raw musl binary and `-musl.tar.gz` |

Builds run on Ubuntu 22.04 x86_64 and ARM64 runners. Debian, RPM, and GNU portable artifacts use the system glibc and require glibc 2.35 or newer. The standalone binaries are built for `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` with Cross and verified to have no ELF interpreter, so they do not depend on the host glibc or a Rust toolchain. Each archive contains the executable, README, and license.

Each published release includes `SHA256SUMS`. Verify downloaded artifacts from the directory containing the downloaded files:

```sh
sha256sum -c SHA256SUMS
```

## Pipeline and safeguards

1. Detect qualifying changes and calculate a version for automatic beta releases, or validate the manually requested stable version.
2. Run formatting, Clippy, and workspace tests on native x86_64 and ARM64 Linux runners.
3. Build the CLI and smoke-test version, help, basic inspection, and machine inspection commands.
4. Build Debian and RPM packages plus a GNU archive for each architecture; verify package architecture and contents. Build and smoke-test static musl standalone binaries and matching archives for both architectures.
5. Generate SHA-256 checksums and publish the GitHub Release only after all builds succeed.

Pull requests and ordinary branch pushes never publish directly. Only the beta workflow on `main`, a manually dispatched stable release, or an explicitly pushed version tag can publish. Build jobs have read-only repository permissions; the publishing job alone receives write permission. The pipeline does not publish to external package repositories or modify the user's machine.

## Validate packaging without publishing

Use the **Release** workflow's manual dispatch only for a real stable release. For packaging experiments, use a pull request or branch CI; do not create a stable tag merely to test packaging.
