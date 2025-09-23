// use std::path::Path;

// use anyhow::Result;

// use graphar::graph_info::GraphInfo;
// use graphar::graph_reader::{AdjListType, EdgesCollection, Expression, VerticesCollection};

// fn modern_graph_path() -> &'static Path {
//     Path::new("incubator-graphar-testing/modern_graph/modern_graph.graph.yml")
// }

// #[test]
// fn iterate_person_vertices() -> Result<()> {
//     let graph = GraphInfo::load(modern_graph_path())?;
//     let collection = VerticesCollection::make(&graph, "person")?;
//     assert_eq!(collection.len(), 4);

//     let mut iter = collection.iter()?;
//     assert!(!iter.is_end());
//     assert_eq!(iter.property_i64("id")?, 2);
//     assert_eq!(iter.property_string("name")?, "vadas");
//     assert_eq!(iter.property_i64("age")?, 27);

//     iter.next()?;
//     assert_eq!(iter.property_string("name")?, "peter");
//     assert_eq!(iter.property_i64("age")?, 35);

//     Ok(())
// }

// #[test]
// fn filter_person_by_age() -> Result<()> {
//     let graph = GraphInfo::load(modern_graph_path())?;
//     let age_prop = Expression::property("age");
//     let thirty = Expression::literal_i64(30);
//     let filter = Expression::greater_than(&age_prop, &thirty);

//     let collection = VerticesCollection::with_property(&graph, "person", "age", &filter)?;
//     let mut names = Vec::new();
//     let mut iter = collection.iter()?;
//     while !iter.is_end() {
//         names.push(iter.property_string("name")?);
//         iter.next()?;
//     }
//     names.sort();
//     assert_eq!(names, vec!["josh", "peter"]);
//     Ok(())
// }

// #[test]
// fn iterate_person_knows_edges() -> Result<()> {
//     let graph = GraphInfo::load(modern_graph_path())?;
//     let edges = EdgesCollection::make(
//         &graph,
//         "person",
//         "knows",
//         "person",
//         AdjListType::ordered_by_source,
//         0,
//         i64::MAX,
//     )?;
//     assert_eq!(edges.len(), 2);

//     let mut iter = edges.iter()?;
//     assert!(!iter.is_end());
//     assert_eq!(iter.source()?, 3);
//     assert_eq!(iter.destination()?, 0);
//     assert!((iter.property_f64("weight")? - 0.5).abs() < 1e-6);

//     iter.next()?;
//     assert_eq!(iter.source()?, 3);
//     assert_eq!(iter.destination()?, 2);
//     assert!((iter.property_f64("weight")? - 1.0).abs() < 1e-6);

//     Ok(())
// }

// #[test]
// fn find_edges_from_vertex() -> Result<()> {
//     let graph = GraphInfo::load(modern_graph_path())?;
//     let edges = EdgesCollection::make(
//         &graph,
//         "person",
//         "knows",
//         "person",
//         AdjListType::ordered_by_source,
//         0,
//         i64::MAX,
//     )?;

//     let iter_anchor = edges.iter()?;
//     let mut found = edges.find_src(3, &iter_anchor)?;
//     assert!(!found.is_end());
//     assert_eq!(found.source()?, 3);
//     assert_eq!(found.destination()?, 0);

//     Ok(())
// }
