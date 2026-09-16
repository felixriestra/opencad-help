# OCS Help

An external Open CAD Studio plugin with one purpose: open the bundled
`OCS Commands.pdf` document in the user's default PDF viewer.

## Commands

| Command | Action |
|---|---|
| `HELP`, `OCS_HELP` | Open the bundled OCS Commands PDF |

The plugin intentionally handles `HELP` before the host's built-in web link, so
F1 opens the PDF while the plugin is enabled. It contains no HTML help, support
links, issue links, or other help content.

## Development build

`Cargo.toml` pins the exact OpenCADStudio revision used by the verified host,
and `rust-toolchain.toml` pins its compiler. Update the dependency revision,
toolchain, and ABI metadata together when targeting a newer host.

```sh
cargo build --release
```

Keep the `plugin.toml` `api_version`, `acadrust_source`, and `rustc_version`
aligned with the target host build.

## Local installation

Stage the plugin beside its manifest, then restart OCS:

```sh
plugin_dir="$HOME/Library/Application Support/OpenCADStudio/plugins/opencad.help"
mkdir -p "$plugin_dir"
cp target/release/libopencad_help.dylib "$plugin_dir/"
cp plugin.toml "$plugin_dir/"
```

For host verification, use a scratch `HOME` so `--mcp` or `--serve` does not
silently connect to an already-running production instance.

## Releases

Tag releases as `v<version>` (e.g. `v0.1.0`).
