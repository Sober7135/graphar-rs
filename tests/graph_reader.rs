use std::path::{Path, PathBuf};

use graphar::{graph_info::GraphInfo, graph_reader::Vertices};

fn load_graph_info() -> GraphInfo {
    let test_data_dir = std::env::var("GAR_TEST_DIR")
        .map(|path| PathBuf::from(path))
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("incubator-graphar-testing")
        });
    let path = test_data_dir
        .join("ldbc_sample")
        .join("parquet")
        .join("ldbc_sample.graph.yml");
    GraphInfo::load(path).unwrap()
}

#[test]
fn test_vertices() {
    let graph_info = load_graph_info();
    let vertex_infos = graph_info.vertex_infos();
    assert_eq!(vertex_infos.len(), 1);

    for v_info in vertex_infos.iter() {
        let vertices = Vertices::new(&graph_info, &v_info.ty()).unwrap();

        assert!(!vertices.is_empty());
    }
}
