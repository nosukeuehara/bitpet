# BitPet v1.0.0

BitPet is a small Rust CLI digital pet for quick terminal check-ins. It starts
from an Egg, hatches into a pet, grows through Monster evolutions, and updates
from saved timestamps whenever you run the command.

## What's New

- Egg / Hatching: new games now begin from an Egg and hatch deterministically.
- Monster Evolution: Baby pets can grow through Stage 1, Stage 2, and Final forms.
- Expedition: send Stage 1 or later pets exploring with `bitpet go`.
- Daily Report / Streak: track care actions, events, and consecutive check-ins.
- Evolution Effect: evolution is shown with a short CLI effect before the new form.
- Legacy Save Migration: v0.1.x saves migrate forward without resetting your pet.
- CLI Help: `bitpet --help`, `bitpet -h`, `bitpet --version`, and `bitpet -V`.
- Time handling fixes: offline progression, local-day resets, hatching, and expedition returns use stable timestamp handling.
- Native CLI: release archives are built for macOS, Linux, and Windows.
- Wasm support: game logic can be built for `wasm32-unknown-unknown`.

## Download / Which file should I use?

Choose the file for your computer:

| Platform | Download |
|---|---|
| macOS - Apple Silicon (M1 / M2 / M3 / M4...) | `bitpet-v1.0.0-aarch64-apple-darwin.tar.gz` |
| macOS - Intel | `bitpet-v1.0.0-x86_64-apple-darwin.tar.gz` |
| Windows - 64-bit | `bitpet-v1.0.0-x86_64-pc-windows-msvc.zip` |
| Linux - 64-bit Intel / AMD | `bitpet-v1.0.0-x86_64-unknown-linux-gnu.tar.gz` |

Apple Silicon includes M1, M2, M3, M4, and newer Apple Silicon Macs.
Intel Macs use the separate Intel Mac asset.

Not sure which Mac you have?

Apple menu -> About This Mac

- If it says Apple M1 / M2 / M3 / M4, choose Apple Silicon.
- If it says Intel, choose Intel.

Files ending in `.sha256` are checksum files used to verify downloads. You do
not need them just to install and play BitPet.

The "Source code" downloads are for developers. Most users should download one
of the BitPet binaries listed above.

## Install

macOS:

```bash
tar -xzf <downloaded-file>.tar.gz
cd <extracted-directory>
./bitpet
```

Linux:

```bash
tar -xzf <downloaded-file>.tar.gz
cd <extracted-directory>
./bitpet
```

Windows:

```powershell
Expand-Archive <downloaded-file>.zip
cd <extracted-directory>
.\bitpet.exe
```

Each archive includes:

- `bitpet` or `bitpet.exe`
- `README.md`
- `LICENSE`
- `THIRD_PARTY_LICENSES.txt`

## Upgrade

BitPet stores save data locally as `save.json`.

- macOS / Linux: `~/.bitpet/save.json`
- Windows: `%APPDATA%\BitPet\save.json`

v0.1.x saves are migrated at startup. Existing pets are not reset to a new Egg,
and supported old saves are not replaced with a fresh game.

## Known Limitations

- Expedition currently supports only `Explore`.
- No Web UI is included.
- No package-manager installer or self-update command is included.

## License

BitPet is distributed under the MIT License. Release archives include `LICENSE`
and `THIRD_PARTY_LICENSES.txt`.
