//! PDF help plugin for Open CAD Studio.
//!
//! The OCS Commands PDF is embedded in the dynamic library because OCS's plugin
//! installer currently downloads one native library plus `plugin.toml`. On
//! first use it is materialized into a versioned temporary folder and opened
//! with the operating system's default PDF viewer.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

const OCS_COMMANDS_PDF: &[u8] = include_bytes!("../assets/ocs-commands.pdf");

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.help",
    name: "OCS Help",
    version: env!("CARGO_PKG_VERSION"),
    description: "Opens the bundled OCS Commands PDF.",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 900,
    xdata_apps: &[],
    command_prefixes: &["OCS_HELP"],
};

struct HelpModule;

impl CadModule for HelpModule {
    fn id(&self) -> &'static str {
        MANIFEST.id
    }

    fn title(&self) -> &'static str {
        "Help"
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![RibbonGroup {
                title: "Reference",
                tools: vec![RibbonItem::LargeTool(ToolDef {
                    id: "OCS_HELP",
                    label: "OCS Commands",
                    icon: IconKind::Glyph("?"),
                    event: ModuleEvent::Command("OCS_HELP".to_string()),
                })],
            }]
        })
    }
}

struct HelpPlugin;

impl BuiltinPlugin for HelpPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }

    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(HelpModule)
    }

    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        match cmd.trim() {
            // HELP intentionally overrides the host's built-in web link.
            "HELP" | "OCS_HELP" => {
                open_embedded_pdf(host);
                true
            }
            _ => false,
        }
    }
}

fn open_embedded_pdf(host: &mut dyn HostApi) {
    match materialize_pdf() {
        Ok(path) => open_pdf(host, &path),
        Err(error) => host.push_error(&format!("Could not prepare OCS help: {error}")),
    }
}

fn materialize_pdf() -> std::io::Result<PathBuf> {
    let root = std::env::temp_dir()
        .join("opencad-help")
        .join(env!("CARGO_PKG_VERSION"));
    fs::create_dir_all(&root)?;

    let path = root.join("OCS Commands.pdf");
    write_if_changed(&path, OCS_COMMANDS_PDF)?;
    Ok(path)
}

fn write_if_changed(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    if fs::read(path).ok().as_deref() == Some(contents) {
        return Ok(());
    }

    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)
}

fn open_pdf(host: &mut dyn HostApi, path: &Path) {
    match launch(&path.to_string_lossy()) {
        Ok(()) => host.push_info("Opening OCS Commands PDF..."),
        Err(error) => host.push_error(&format!("Could not open help: {error}")),
    }
}

#[cfg(target_os = "macos")]
fn launch(target: &str) -> std::io::Result<()> {
    Command::new("open").arg(target).spawn().map(|_| ())
}

#[cfg(target_os = "windows")]
fn launch(target: &str) -> std::io::Result<()> {
    Command::new("cmd")
        .args(["/C", "start", "", target])
        .spawn()
        .map(|_| ())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn launch(target: &str) -> std::io::Result<()> {
    Command::new("xdg-open").arg(target).spawn().map(|_| ())
}

ocs_plugin_api::export_plugin!(HelpPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_document_is_the_ocs_commands_pdf() {
        assert_eq!(OCS_COMMANDS_PDF.len(), 4_253_366);
        assert!(OCS_COMMANDS_PDF.starts_with(b"%PDF-"));
    }

    #[test]
    fn pdf_materializes_with_exact_contents() {
        let path = materialize_pdf().expect("PDF materializes");
        assert_eq!(
            fs::read(path).expect("materialized PDF reads"),
            OCS_COMMANDS_PDF
        );
    }
}
