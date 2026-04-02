use std::io::{self, Write};
use std::path::Path;

#[cfg(feature = "trust-manifest")]
use crate::script_discovery::DiscoveryFlow;
#[cfg(feature = "trust-manifest")]
use crate::trust::{self, TrustedManifest};
use crate::{JaoResult, script_discovery};

#[cfg(feature = "trust-manifest")]
/// Lists discovered scripts with trust-state labels.
///
/// Output format: `<trust>\t<command>\t\t<resolved_path>`.
///
/// `trust` values map to [`crate::trust::ScriptTrustState`] display labels.
pub(crate) fn list_scripts_with_trust_status(root: impl AsRef<Path>, manifest: &TrustedManifest) -> JaoResult<()> {
    let mut out = io::stdout().lock();

    script_discovery::for_each_discovered_script(&root, |script| {
        let trust = trust::determine_script_trust_state(script.path, manifest)?;

        #[rustfmt::skip]
        writeln!(
            out,"{trust} \t {} \t\t {}",
            script.parts.display().to_string_lossy(),
            script.path.display()
        )?;

        Ok(DiscoveryFlow::ContinueSearching)
    })?;

    Ok(())
}

#[cfg(not(feature = "trust-manifest"))]
/// Lists discovered scripts without trust-state labels.
///
/// Output format: `<command>\t\t<resolved_path>`.
///
/// This variant is compiled when trust-manifest support is disabled.
pub(crate) fn list_scripts(root: impl AsRef<Path>) -> JaoResult<()> {
    let mut out = io::stdout().lock();

    script_discovery::for_each_discovered_script(&root, |script| {
        #[rustfmt::skip]
        writeln!(
            out, "{} \t\t {}",
            script.parts.display().to_string_lossy(),
            script.path.display()
        )?;

        Ok(DiscoveryFlow::ContinueSearching)
    })?;

    Ok(())
}
