mod common;

use graphar::graph_info::GraphInfo;
use graphar::graph_reader::Vertices;

#[test]
fn parquet_vertices_loader_smoke() {
    let path = common::test_data_root()
        .join("ldbc_sample")
        .join("parquet")
        .join("ldbc_sample.graph.yml");
    let graph_info = GraphInfo::load(path).unwrap();

    let vertices = Vertices::new(&graph_info, "person").unwrap();
    assert!(!vertices.is_empty());
}

#[test]
fn modern_graph_indices() {
    let path = common::test_data_root()
        .join("modern_graph")
        .join("modern_graph.graph.yml");
    let graph_info = GraphInfo::load(path).unwrap();

    // Check expected indices from file order
    assert_eq!(graph_info.vertex_info_index("person"), 0);
    assert_eq!(graph_info.vertex_info_index("software"), 1);

    // Edge indices (order as listed in the YAML)
    assert_eq!(graph_info.edge_info_index("person", "knows", "person"), 0);
    assert_eq!(
        graph_info.edge_info_index("person", "created", "software"),
        1
    );
}
