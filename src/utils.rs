use std::path::{PathBuf, Path};
use once_cell::sync::Lazy;

pub static APP_ICON: Lazy<PathBuf> = Lazy::new(|| {Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets/images")
        .join("alarm-app-icon.ico")});
