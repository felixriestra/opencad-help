# OCS Help plugin notes

## Packaging

OCS currently installs one platform library plus `plugin.toml`. The sole help
document, `OCS Commands.pdf`, is therefore embedded with `include_bytes!` and
materialized to a versioned temporary folder when first opened.

## Host interaction

This plugin does not read or mutate drawings and requires no new `HostApi`
methods. Opening the PDF is performed by the plugin process through the
platform launcher (`open`, `cmd /C start`, or `xdg-open`). Failures are reported
through `HostApi::push_error`.

## Known limits

- The bundled PDF is a release snapshot. Updating the documentation
  requires rebuilding the plugin.
- Browser-mode OCS does not load native plugins.
- Linux requires an `xdg-open` provider in the desktop environment.
