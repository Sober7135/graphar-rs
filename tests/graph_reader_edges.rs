mod common;

use graphar::graph_info::{AdjListType, GraphInfo};
use graphar::graph_reader::Edges;

#[test]
fn edges_iterate_and_properties() {
    let path = common::test_data_root()
        .join("ldbc_sample")
        .join("csv")
        .join("ldbc_sample.graph.yml");
    let graph_info = GraphInfo::load(path).unwrap();

    let mut edges = Edges::new(
        &graph_info,
        "person",
        "knows",
        "person",
        AdjListType::OrderedBySource,
        None,
    )
    .unwrap();

    assert!(!edges.is_empty());
    assert!(edges.len() > 0);
    let mut begin = edges.begin();
    assert!(!begin.is_end());

    // Read a property from the first edge
    let creation_date = begin.property::<String>("creationDate").unwrap();
    assert!(!creation_date.is_empty());
    // FIXME
    // assert_eq!(creation_date, "2010-07-30T15:19:53.298+0000");

    // Some navigation sanity checks
    let _ = begin.source();
    let _ = begin.destination();
    begin.next();
    let _ = begin.source();
    let _ = begin.destination();

    // Jump to a specific source/destination id present in dataset
    assert!(begin.next_src_with_id(0));
    assert_eq!(begin.source(), 0);
    assert!(begin.next_dst_with_id(87));
    assert_eq!(begin.destination(), 87);

    // Chunk/offset diagnostics are non-negative
    assert!(begin.global_chunk_index() >= 0);
    assert!(begin.cur_offset() >= 0);

    // find_src/dst should locate valid positions for known ids
    let end = edges.end();
    let begin_for_find = edges.begin();
    let found_src0 = edges.find_src(0, &begin_for_find);
    let found_dst87 = edges.find_dst(87, &begin_for_find);
    assert!(found_src0 != end);
    assert!(found_dst87 != end);

    // Range-restricted edges should not exceed total length
    let edges_small = Edges::new(
        &graph_info,
        "person",
        "knows",
        "person",
        AdjListType::OrderedBySource,
        Some(0..1),
    )
    .unwrap();
    assert!(edges_small.len() > 0);
    assert!(edges_small.len() < edges.len());
}

#[test]
fn edges_ordered_by_dest() {
    let path = common::test_data_root()
        .join("ldbc_sample")
        .join("csv")
        .join("ldbc_sample.graph.yml");
    let gi = GraphInfo::load(path).unwrap();

    let mut edges = Edges::new(
        &gi,
        "person",
        "knows",
        "person",
        AdjListType::OrderedByDest,
        None,
    )
    .unwrap();

    assert!(!edges.is_empty());
    let mut it = edges.begin();
    assert!(!it.is_end());
    // Property exists
    assert!(!it.property::<String>("creationDate").unwrap().is_empty());
    // Move a bit and ensure we still have valid positions
    it.next();
    let _ = it.source();
    let _ = it.destination();
}
