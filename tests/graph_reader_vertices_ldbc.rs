mod common;

use graphar::graph_info::GraphInfo;
use graphar::graph_reader::Vertices;

#[test]
fn vertices_with_label_and_labels_ldbc_parquet() {
    let path = common::test_data_root()
        .join("ldbc")
        .join("parquet")
        .join("ldbc.graph.yml");
    let graph_info = GraphInfo::load(path).unwrap();

    // With label: organisation::university
    let mut org_univ = Vertices::with_label(&graph_info, "organisation", "university").unwrap();
    assert!(!org_univ.is_empty());
    let mut it = org_univ.begin();
    let org_labels = it.labels().unwrap();
    assert!(org_labels.iter().any(|l| l == "university"));

    // labels() returns the labels vector for organisation vertices as well
    let labels = it.labels().unwrap();
    assert!(labels.iter().any(|l| l == "university" || l == "company" || l == "public"));
}
