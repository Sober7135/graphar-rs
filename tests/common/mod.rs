use std::path::{Path, PathBuf};

/// Returns the root directory containing the bundled GraphAr fixtures.
pub fn test_data_root() -> PathBuf {
    std::env::var("GAR_TEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("incubator-graphar-testing"))
}
