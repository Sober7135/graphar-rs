use graphar::{
    graph_builder::{self, EdgesBuilder, VerticesBuilder},
    graph_info::{
        AdjListType, AdjacentList, AdjacentListVector, Cardinality, DataType, EdgeInfo, FileType,
        GraphInfo, InfoVersion, Property, PropertyGroup, PropertyGroupVector, PropertyVec,
        VertexInfo,
    },
};

fn dump() {
    let mut vertex_props = PropertyVec::new();

    // Add id
    vertex_props.add_property(Property::new(
        "id",
        &DataType::int64(),
        false,
        false,
        Cardinality::Single,
    ));
    // Add name
    vertex_props.add_property(Property::new(
        "name",
        &DataType::string(),
        false,
        false,
        Cardinality::Single,
    ));

    let vertex_prop_group = PropertyGroup::new(vertex_props, FileType::Parquet, "");
    let mut vertex_prop_groups = PropertyGroupVector::new();
    vertex_prop_groups.add_property_group(vertex_prop_group);

    // `VertexInfo` save
    let version = InfoVersion::new(1).unwrap();
    let vertex_info = VertexInfo::new(
        "person".into(),
        1024,
        vertex_prop_groups,
        vec![],
        "",
        version.clone(),
    );
    vertex_info
        .save("/tmp/test_graphar/person.vertex.yml")
        .unwrap();

    // `VerticesBuilder` dump
    let mut vb = VerticesBuilder::new(&vertex_info, "/tmp/test_graphar/vertex/", 0).unwrap();
    let mut alice = graph_builder::VertexBuilder::new();
    alice.add_property("id".into(), 1_i64);
    alice.add_property("name".into(), "alice".to_string());

    let mut bob = graph_builder::VertexBuilder::new();
    bob.add_property("id".into(), 2_i64);
    bob.add_property("name".into(), "bob".to_string());
    vb.add_vertex(alice).unwrap();
    vb.add_vertex(bob).unwrap();
    vb.dump().unwrap();

    // `EdgeInfo`
    let mut adjs = AdjacentListVector::new();
    let adj = AdjacentList::new(AdjListType::OrderedBySource, FileType::Orc, "");
    adjs.add_adjacent_list(adj);

    let mut edge_props = PropertyVec::new();
    edge_props.add_property(Property::new(
        "friend",
        &DataType::string(),
        false,
        true,
        Cardinality::Single,
    ));
    let mut edge_prop_groups = PropertyGroupVector::new();
    edge_prop_groups.add_property_group(PropertyGroup::new(
        edge_props,
        FileType::Csv,
        "knows/props",
    ));

    let edge_info = EdgeInfo::new(
        "person",
        "knows",
        "person",
        1024,
        1024,
        1024,
        true,
        adjs,
        edge_prop_groups,
        "",
        version.clone(),
    );
    edge_info
        .save("/tmp/test_graphar/person_knows_person.edge.yml")
        .unwrap();

    // EdgesBuilder
    let mut edge_builder = EdgesBuilder::new(
        &edge_info,
        "/tmp/test_graphar/edge/",
        AdjListType::OrderedBySource,
        2,
    )
    .unwrap();
    let mut e = graph_builder::EdgeBuilder::new(1, 2);
    e.add_property("friend".into(), "bob".to_string());
    edge_builder.add_edge(e).unwrap();
    edge_builder.dump().unwrap();

    let graph_info = GraphInfo::new(
        "test_graphar",
        &vec![vertex_info],
        &vec![edge_info],
        &vec![],
        "./",
        Some(version.clone()),
    );
    graph_info.save("/tmp/test_graphar/test.graph.yml").unwrap();
}

fn main() {
    dump();
}
