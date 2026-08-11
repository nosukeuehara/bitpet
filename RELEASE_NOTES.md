# BitPet v1.1.0

BitPet v1.1.0 adds Native CLI self-update support.

## What's New

- `bitpet update --check` checks GitHub Releases for the latest stable BitPet release.
- `bitpet update` downloads, verifies, and installs the matching Native CLI asset.
- Updates use SHA-256 checksum files before replacing the current executable.
- Save data is not touched by the updater.
- Wasm builds remain separate and do not include self-update.

## Self Update

Once you have installed the standalone Native CLI from GitHub Releases, v1.1.0
and newer can usually update themselves:

```bash
bitpet update --check
bitpet update
```

`bitpet update --check` only reports whether an update is available. It does not
modify files.

`bitpet update` uses the official GitHub Release assets for:

- macOS Apple Silicon
- macOS Intel
- Linux 64-bit Intel / AMD
- Windows 64-bit

If you installed BitPet through a package manager, prefer that package manager's
update command.

## Upgrade

Existing save data remains compatible. The updater does not read, write, or move
`save.json`; normal save migration still happens when BitPet starts.

## Known Limitations

- Self-update is for Native CLI builds only.
- Prerelease versions such as beta or rc releases are not installed by default.
- Expedition currently supports only `Explore`.
- No Web UI is included.

## License

BitPet is distributed under the MIT License. Release archives include `LICENSE`
and `THIRD_PARTY_LICENSES.txt`.
