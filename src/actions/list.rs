use jwalk::WalkDir;
use std::path::Path;
use std::path::PathBuf;

pub fn list_scripts(root: PathBuf) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let ext = Path::new(entry.file_name()).extension()?.to_str()?;

            if is_supported_script_extension(ext) {
                Some(entry.path())
            } else {
                None
            }
        })
}

fn is_supported_script_extension(ext: &str) -> bool {
    if cfg!(windows) {
        ext.eq_ignore_ascii_case("bat")
    } else {
        ext.eq_ignore_ascii_case("sh")
    }
}
