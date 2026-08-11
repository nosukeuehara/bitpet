# BitPet v0.1.0

BitPet is a small Rust CLI digital pet for quick terminal check-ins during work.
It does not run a background process. Instead, it updates the pet state from
saved timestamps whenever you run the command.

## Highlights

- New game creation and save/load through `save.json`
- Status display with compact ASCII art
- Offline time progression for hunger and energy
- `feed` and `play` care actions with daily limits
- Experience, level-up, and first evolution from Baby to Stage 1
- `go` Explore expeditions with away state, return time, and rewards
- Daily Report with action and event history
- Login streak tracking
- Native CLI release archives for macOS, Linux, and Windows
- WebAssembly build support for using BitPet game logic from web apps

## Supported Platforms

Native release artifacts are built for:

- macOS Apple Silicon: `aarch64-apple-darwin`
- macOS Intel: `x86_64-apple-darwin`
- Linux x86_64: `x86_64-unknown-linux-gnu`
- Windows x86_64: `x86_64-pc-windows-msvc`

## Installation

Download the archive for your platform from this release, extract it, and run
the `bitpet` executable. Each archive includes:

- `bitpet` or `bitpet.exe`
- `README.md`
- `LICENSE`
- `THIRD_PARTY_LICENSES.txt`

## Save Data

BitPet stores save data locally as `save.json`.

- macOS / Linux: `~/.bitpet/save.json`
- Windows: `%APPDATA%\BitPet\save.json`

Old save versions are migrated at startup where supported. Broken save data is
reported as a user-facing read error instead of panicking.

## WebAssembly

The `wasm` feature builds a WebAssembly adapter. v0.1.0 does not include a Web
UI. Web apps can use the generated Wasm package and store the returned save JSON
in browser storage such as `localStorage` or IndexedDB.

## Known Limitations

- Only the first growth stage and first evolution are implemented.
- Expedition currently supports only `Explore`.
- No Web UI is included.
- No self-update or package-manager installation is included.
