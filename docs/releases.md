# Release process

## Supported release artifacts

Every release builds Linux packages on native Linux runners for both supported architectures:

| Architecture | Debian package | RPM package | Portable archive |
| --- | --- | --- | --- |
| x86_64 | `.deb` (amd64) | `.rpm` (x86_64) | `.tar.gz` |
| aarch64 | `.deb` (arm64) | `.rpm` (aarch64) | `.tar.gz` |

Builds run on Ubuntu 22.04 x86_64 and ARM64 runners. The GNU/Linux binaries use the system glibc and are intended for glibc 2.35 or newer. The packages install the executable at `/usr/bin/zashikura`. The portable archive contains the executable, README, and license.

Each published release also includes `SHA256SUMS`. Verify downloaded artifacts from the directory containing the downloaded files:

```sh
sha256sum -c SHA256SUMS
```

## Creating a release

1. Ensure the intended changes are merged into `main` and CI is green.
2. Update the workspace version in the root `Cargo.toml`. All workspace crates inherit this version.
3. Update and commit `Cargo.lock` when dependency resolution changes.
4. Merge the version change into `main`.
5. Create and push a matching version tag from the release commit:

   ```sh
   git tag -a v0.1.0 -m "Zashikura v0.1.0"
   git push origin v0.1.0
   ```

The tag must exactly match the CLI package version. The workflow rejects mismatches and verifies that each runner is building the expected CPU architecture.

## Validate packaging without publishing

After this workflow has been merged, use GitHub Actions' **Release** workflow with the `workflow_dispatch` event and enter the workspace version (for example, `0.1.0`). It builds and uploads test artifacts for both architectures but does not create a GitHub Release.

## Release pipeline

A version-tag push runs the following steps:

1. Checks out the tagged source and installs stable Rust.
2. Runs formatting, Clippy, and workspace tests on native x86_64 and ARM64 Linux runners.
3. Builds the CLI and smoke-tests its version, help, basic inspection, and machine inspection commands.
4. Builds Debian and RPM packages plus a portable tarball for each architecture, then checks package and archive contents.
5. Collects all artifacts, generates SHA-256 checksums, and publishes a GitHub Release with generated release notes.

A tag containing a hyphen (for example, `v0.2.0-rc.1`) is published as a prerelease. Other matching tags are published as stable releases.

Pull requests, ordinary branch pushes, and manual packaging validations never publish releases. The release workflow does not publish to external package repositories or modify the user's machine.
