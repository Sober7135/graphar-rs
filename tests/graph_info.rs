use std::{fs, path::Path};

use graphar::graph_info::*;
use tempfile::tempdir;

#[test]
fn test_graph_info_load_from_fixture_and_dump_save() -> anyhow::Result<()> {
    let path = Path::new("incubator-graphar-testing/modern_graph/modern_graph.graph.yml");
    assert!(path.exists());
    let abs = fs::canonicalize(path)?;
    let info = GraphInfo::load(&abs)?;
    assert_eq!(info.name(), "modern_graph");
    assert_eq!(info.vertex_info_num(), 2);
    assert_eq!(info.edge_info_num(), 2);

    let dump = info.dump()?;
    assert!(dump.contains("name: modern_graph"));
    assert!(dump.contains("version: gar/v1"));

    let dir = tempdir()?;
    let out = dir.path().join("roundtrip.graph.yml");
    info.save(&out)?;
    assert!(out.exists());

    Ok(())
}
