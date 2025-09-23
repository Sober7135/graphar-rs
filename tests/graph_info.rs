use std::{fs, path::Path};

use graphar::graph_info::*;
use tempfile::tempdir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn property_group_has_property() {
    let mut props = PropertyVec::new();
    props.add_property(Property::new(
        "id",
        &DataType::int64(),
        false,
        false,
        Cardinality::Single,
    ));
    props.add_property(Property::new(
        "name",
        &DataType::string(),
        false,
        false,
        Cardinality::Single,
    ));
    let pg = PropertyGroup::new(props, FileType::Csv, "");
    assert!(pg.has_property("id"));
    assert!(pg.has_property("name"));
    assert!(!pg.has_property("missing"));
}

#[test]
fn adjacent_list_new_fields() {
    let adj = AdjacentList::new(AdjListType::OrderedBySource, FileType::Csv, "adj/");
    assert!(matches!(adj.list_type(), AdjListType::OrderedBySource));
    assert!(matches!(adj.file_type(), FileType::Csv));
    assert_eq!(adj.prefix(), "adj/");
}

#[test]
fn vertex_info_new_dump_and_save() -> Result<()> {
    let mut props = PropertyVec::new();
    props.add_property(Property::new(
        "id",
        &DataType::int64(),
        false,
        false,
        Cardinality::Single,
    ));
    let pg = PropertyGroup::new(props, FileType::Csv, "");
    let mut pgv = PropertyGroupVector::new();
    pgv.add_property_group(pg);
    let ver = InfoVersion::new(1)?;

    let vi = VertexInfo::new("person".into(), 2, pgv, vec![], "", ver);
    let dump = vi.dump()?;
    assert!(dump.contains("type: person"));
    assert!(dump.contains("chunk_size: 2"));

    let dir = tempdir()?;
    let path = dir.path().join("person.vertex.yml");
    vi.save(&path)?;
    assert!(path.exists());

    Ok(())
}

#[test]
fn edge_info_new_getters_dump_and_save() -> Result<()> {
    let mut adjs = AdjacentListVector::new();
    adjs.add_adjacent_list(AdjacentList::new(
        AdjListType::OrderedBySource,
        FileType::Csv,
        "ordered_by_source/",
    ));

    let mut props = PropertyVec::new();
    props.add_property(Property::new(
        "weight",
        &DataType::float64(),
        false,
        true,
        Cardinality::Single,
    ));
    let mut pgv = PropertyGroupVector::new();
    pgv.add_property_group(PropertyGroup::new(props, FileType::Csv, "weight/"));

    let ver = InfoVersion::new(1)?;
    let ei = EdgeInfo::new(
        "person", "knows", "person", 10, 2, 2, true, adjs, pgv, "", ver,
    );
    assert_eq!(ei.src_type(), "person");
    assert_eq!(ei.dst_type(), "person");
    assert_eq!(ei.edge_type(), "knows");
    assert_eq!(ei.chunk_size(), 10);
    assert!(ei.directed());

    let dump = ei.dump()?;
    assert!(dump.contains("edge_type: knows"));

    let dir = tempdir()?;
    let path = dir.path().join("person_knows_person.edge.yml");
    ei.save(&path)?;
    assert!(path.exists());

    Ok(())
}

#[test]
fn graph_info_load_from_fixture_and_dump_save() -> Result<()> {
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

#[test]
fn graph_info_new_build_and_dump() -> Result<()> {
    // VertexInfo
    let mut vp = PropertyVec::new();
    vp.add_property(Property::new(
        "id",
        &DataType::int64(),
        true,
        false,
        Cardinality::Single,
    ));
    let pg = PropertyGroup::new(vp, FileType::Csv, "");
    let mut pgv = PropertyGroupVector::new();
    pgv.add_property_group(pg);
    let ver = InfoVersion::new(1)?;
    let vi = VertexInfo::new(
        "person".into(),
        2,
        pgv,
        vec!["Person".into()],
        "",
        ver.clone(),
    );

    // EdgeInfo
    let mut adjs = AdjacentListVector::new();
    adjs.add_adjacent_list(AdjacentList::new(
        AdjListType::OrderedBySource,
        FileType::Csv,
        "ordered_by_source/",
    ));
    let mut ep = PropertyVec::new();
    ep.add_property(Property::new(
        "w",
        &DataType::float64(),
        false,
        true,
        Cardinality::Single,
    ));
    let mut epg = PropertyGroupVector::new();
    epg.add_property_group(PropertyGroup::new(ep, FileType::Csv, "w/"));
    let ei = EdgeInfo::new(
        "person", "knows", "person", 10, 2, 2, true, adjs, epg, "", ver,
    );

    // GraphInfo::new
    let name = "my_graph".to_string();
    let labels = vec!["Person".to_string()];
    let prefix = "test_graph/".to_string();
    let g = GraphInfo::new(&name, &vec![vi], &vec![ei], &labels, &prefix, None);

    assert_eq!(g.name(), name);
    assert_eq!(g.vertex_info_num(), 1);
    assert_eq!(g.edge_info_num(), 1);
    assert_eq!(g.prefix(), prefix);
    assert!(g.labels().contains(&"Person".to_string()));

    let dump = g.dump()?;
    assert!(dump.contains("name: my_graph"));
    println!("{}", dump);

    let dir = tempdir()?;
    let out = dir.path().join("graph.graph.yml");
    g.save(&out)?;
    assert!(out.exists());

    Ok(())
}
