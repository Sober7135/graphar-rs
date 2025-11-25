mod common;

use graphar::{graph_info::GraphInfo, graph_reader::Vertices};

#[test]
fn vertices_iterate_and_read_properties_modern_graph() {
    let path = common::test_data_root()
        .join("modern_graph")
        .join("modern_graph.graph.yml");
    let gi = GraphInfo::load(path).unwrap();

    let mut vertices = Vertices::new(&gi, "person").unwrap();
    assert!(!vertices.is_empty());
    assert_eq!(vertices.len(), 4);

    // Collect first four names using iterator
    let mut iter = vertices.begin();
    let names: Vec<String> = (&mut iter)
        .map(|v| v.property::<String>("name").unwrap())
        .take(4)
        .collect();
    assert_eq!(names, vec!["vadas", "peter", "josh", "marko"]);

    // Reset and validate additional properties on first vertex
    let mut iter = vertices.begin();
    let v0 = iter.vertex();
    assert!(v0.is_valid("age"));
    let id0 = v0.property::<i64>("id").unwrap();
    let age0 = v0.property::<i64>("age").unwrap();
    let name0 = v0.property::<String>("name").unwrap();
    assert_eq!(id0, 2);
    assert_eq!(age0, 27);
    assert_eq!(name0, "vadas");

    // Use VertexIter::property on a found iterator
    let mut found = vertices.find(2);
    let name2 = found.property::<String>("name").unwrap();
    let id2 = found.property::<i64>("id").unwrap();
    assert_eq!(name2, "josh");
    assert_eq!(id2, 4);

    // After iterating len() times, iterator equals end
    let mut it = vertices.begin();
    for _ in 0..vertices.len() {
        it.next();
    }
    let end = vertices.end();
    assert!(it == end);

    // Skip labels() due to missing label data in this fixture
}
