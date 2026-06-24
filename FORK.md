# Fork maintenance

This fork maintains a complete, matched komorebi executable suite.

## Stable release line

The current stable line is based on upstream `v0.1.40` and adds:

- per-window virtual-desktop filtering before window-manager event processing;
- removal of managed windows that Windows moves to another virtual desktop;
- source-workspace retiling after such a move.

Release tags use the neutral form:

```text
v0.1.40-desktop-isolation.1
```

## Upstream

The local repository should use:

```text
origin    public fork
upstream  https://github.com/LGUG2Z/komorebi.git
```

Upstream releases are integrated on a temporary branch and merged into `main`
only after the fork-specific behavior and layout modes have been tested.

## Releases

Pushing a `v*` tag runs the Windows workflow and publishes matching binaries,
MSI installers, ZIP archives, and SHA-256 checksums from the tagged commit.
The release workflow then updates the Scoop manifest in `bucket/`.

The fork does not submit releases to the official Winget package.

## Actions workflow

The Windows workflow uses a staged release pipeline:

1. validate formatting, Clippy, and tests on pinned Rust toolchains;
2. build x86_64 and ARM64 artifacts independently;
3. package both architectures and calculate checksums on every run;
4. publish a GitHub release only for a `v*` tag;
5. update the Scoop manifest only after the release succeeds.

Pull requests and manual workflow runs are dry runs: they build and package
artifacts but cannot publish a release or modify the Scoop manifest.

Before merging workflow changes into `main`:

1. run the pinned formatting, Clippy, and test commands locally;
2. push an isolated `ci/*` branch;
3. open a pull request to exercise both hosted Windows architectures;
4. inspect the packaged dry-run artifact;
5. merge only after all jobs pass.

## Scoop

Add this repository as a Scoop bucket and install the complete suite:

```powershell
scoop bucket add komorebi-fork https://github.com/chixing/komorebi
scoop install komorebi-fork/komorebi-desktop-isolation
```

Update through Scoop:

```powershell
scoop update
scoop update komorebi-desktop-isolation
```
