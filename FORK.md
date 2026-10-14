# Fork maintenance

This fork maintains a complete, matched komorebi executable suite.

## Stable release line

The current stable line is based on upstream `v0.1.41` and adds:

- per-window virtual-desktop filtering before window-manager event processing;
- removal of managed windows that Windows moves to another virtual desktop;
- source-workspace retiling after such a move;
- the upstream `v0.1.40` directional-focus behavior while monocled, allowing
  focus to cross to an adjacent monitor instead of cycling monocle windows;
- synchronous border positioning and redraws on size/rect changes, keeping
  borders painted with `"window_handling_behaviour": "Async"`.

Release tags use the neutral form:

```text
v0.1.41-desktop-isolation.1
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

GitHub Actions is disabled for this fork. Builds are local only; the
workflow files stay in `.github/` to keep upstream merges clean.

Validate, build, and install the complete suite:

```powershell
$Env:RUSTFLAGS = '-Ctarget-feature=+crt-static -Dwarnings'
cargo +nightly-2026-06-23 fmt --check
cargo +1.96.0 clippy --locked
cargo +1.96.0 test --locked
cargo +1.96.0 build --locked --release --target x86_64-pc-windows-msvc
```

A clean release build takes about 3 minutes; an incremental one under a minute.

Stop komorebi, copy the six executables (`komorebi`, `komorebic`,
`komorebic-no-console`, `komorebi-bar`, `komorebi-gui`, `komorebi-shortcuts`)
from `target\x86_64-pc-windows-msvc\release` over
`~\scoop\apps\komorebi-desktop-isolation\current`, then start komorebi.
`komorebic --version` shows the deployed branch or tag and commit.

Tags mark deployed commits. They no longer publish GitHub releases.

The fork does not submit releases to the official Winget package.

## Scoop

The Scoop install from `bucket/` is only a container for the local build. The
manifest stays at `0.1.41-desktop-isolation.1`, so `scoop update` leaves the
local executables alone. Do not run `scoop uninstall` or `scoop reset` on it.
