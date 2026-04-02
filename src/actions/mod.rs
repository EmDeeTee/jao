#![doc(hidden)]

mod completion;
mod fingerprint;
mod info;
mod list;
mod run;

/// Completion request model and completion entry points.
///
/// Re-exports shell completion script output and dynamic completion protocol
/// handling.
pub(crate) use completion::{CompletionRequest, Shell, complete, print_shell_completion};
/// Fingerprint printing action.
pub(crate) use fingerprint::fingerprint_script;
/// Help text rendering action.
pub(crate) use info::print_help;
#[cfg(not(feature = "trust-manifest"))]
/// Script listing action without trust labels.
///
/// Used when trust-manifest support is disabled.
pub(crate) use list::list_scripts;
#[cfg(feature = "trust-manifest")]
/// Script listing action with trust labels.
pub(crate) use list::list_scripts_with_trust_status;
/// Script execution with explicit fingerprint verification.
pub(crate) use run::run_script_with_fingerprint;
#[cfg(feature = "trust-manifest")]
/// Script execution with interactive trust workflow.
pub(crate) use run::run_script_with_trust;
