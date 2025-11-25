use std::path::Path;

use cxx::{SharedPtr, UniquePtr, let_cxx_string};

use crate::{
    ffi::{
        self,
        graphar::{
            add_vertex, edge_add_property_bool, edges_dump, new_edge, new_edges_builder,
            new_vertex, new_vertices_builder, vertex_add_property_bool, vertex_add_property_f32,
            vertex_add_property_f64, vertex_add_property_i32, vertex_add_property_i64,
            vertex_add_property_string, vertices_dump,
        },
    },
    graph_info::{AdjListType, EdgeInfo, VertexInfo},
};

pub trait SupportedDataType<T> {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: T);
    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: T);
}

impl SupportedDataType<bool> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: bool) {
        let_cxx_string!(name = name);
        vertex_add_property_bool(vertex.inner.pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: bool) {
        let_cxx_string!(name = name);
        edge_add_property_bool(edge.inner.pin_mut(), &name, val);
    }
}

impl SupportedDataType<i32> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: i32) {
        let_cxx_string!(name = name);
        vertex_add_property_i32(vertex.inner.pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: i32) {
        let_cxx_string!(name = name);
        ffi::graphar::edge_add_property_i32(edge.inner.pin_mut(), &name, val);
    }
}

impl SupportedDataType<i64> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: i64) {
        let_cxx_string!(name = name);
        vertex_add_property_i64(vertex.inner.pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: i64) {
        let_cxx_string!(name = name);
        ffi::graphar::edge_add_property_i64(edge.inner.pin_mut(), &name, val);
    }
}

impl SupportedDataType<f32> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: f32) {
        let_cxx_string!(name = name);
        vertex_add_property_f32(vertex.inner.pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: f32) {
        let_cxx_string!(name = name);
        ffi::graphar::edge_add_property_f32(edge.inner.pin_mut(), &name, val);
    }
}

impl SupportedDataType<f64> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: f64) {
        let_cxx_string!(name = name);
        vertex_add_property_f64(vertex.inner.pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: f64) {
        let_cxx_string!(name = name);
        ffi::graphar::edge_add_property_f64(edge.inner.pin_mut(), &name, val);
    }
}

impl SupportedDataType<String> for () {
    fn vertex_add_property(vertex: &mut VertexBuilder, name: &str, val: String) {
        let_cxx_string!(name = name);
        let_cxx_string!(val = val);
        vertex_add_property_string(vertex.inner.pin_mut(), &name, &val);
    }

    fn edge_add_property(edge: &mut EdgeBuilder, name: &str, val: String) {
        let_cxx_string!(name = name);
        let_cxx_string!(val = val);
        ffi::graphar::edge_add_property_string(edge.inner.pin_mut(), &name, &val);
    }
}

impl<T> SupportedDataType<Vec<T>> for ()
where
    (): SupportedDataType<T>,
{
    fn vertex_add_property(_vertex: &mut VertexBuilder, _name: &str, _val: Vec<T>) {
        todo!()
    }

    fn edge_add_property(_edge: &mut EdgeBuilder, _name: &str, _val: Vec<T>) {
        todo!()
    }
}
// TODO(date, timestamp)

fn vertex_add_property<T, S: AsRef<str>>(vertex: &mut VertexBuilder, name: S, val: T)
where
    (): SupportedDataType<T>,
{
    <() as SupportedDataType<T>>::vertex_add_property(vertex, name.as_ref(), val);
}

pub struct VertexBuilder {
    inner: UniquePtr<ffi::graphar::VertexBuilder>,
}

impl Default for VertexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VertexBuilder {
    pub fn new() -> Self {
        Self {
            inner: new_vertex(),
        }
    }

    pub fn add_property<T>(&mut self, name: String, property: T)
    where
        (): SupportedDataType<T>,
    {
        vertex_add_property(self, &name, property);
    }
}

pub struct VerticesBuilder {
    inner: SharedPtr<ffi::graphar::VerticesBuilder>,
}

impl VerticesBuilder {
    pub fn new<P: AsRef<Path>>(
        vertex_info: &VertexInfo,
        path_prefix: P,
        start_idx: i64,
    ) -> anyhow::Result<Self> {
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        Ok(Self {
            inner: new_vertices_builder(&vertex_info.inner, &prefix, start_idx)?,
        })
    }

    pub fn add_vertex(&mut self, mut vertex: VertexBuilder) -> anyhow::Result<()> {
        unsafe { add_vertex(self.inner.pin_mut_unchecked(), vertex.inner.pin_mut())? };
        Ok(())
    }

    pub fn dump(&mut self) -> anyhow::Result<()> {
        unsafe { vertices_dump(self.inner.pin_mut_unchecked())? };
        Ok(())
    }
}

pub struct EdgeBuilder {
    inner: UniquePtr<ffi::graphar::EdgeBuilder>,
}

impl EdgeBuilder {
    pub fn new(src_id: i64, dst_id: i64) -> Self {
        Self {
            inner: new_edge(src_id, dst_id),
        }
    }

    pub fn add_property<T>(&mut self, name: String, property: T)
    where
        (): SupportedDataType<T>,
    {
        <() as SupportedDataType<T>>::edge_add_property(self, &name, property);
    }
}

pub struct EdgesBuilder {
    inner: SharedPtr<ffi::graphar::EdgesBuilder>,
}

impl EdgesBuilder {
    pub fn new<P: AsRef<Path>>(
        edge_info: &EdgeInfo,
        path_prefix: P,
        adj_list_type: AdjListType,
        vertices_num: i64,
    ) -> anyhow::Result<Self> {
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let inner = new_edges_builder(&edge_info.inner, &prefix, adj_list_type, vertices_num)?;
        Ok(Self { inner })
    }

    pub fn add_edge(&mut self, mut edge: EdgeBuilder) -> anyhow::Result<()> {
        unsafe { ffi::graphar::add_edge(self.inner.pin_mut_unchecked(), edge.inner.pin_mut())? };
        Ok(())
    }

    pub fn dump(&mut self) -> anyhow::Result<()> {
        unsafe { edges_dump(self.inner.pin_mut_unchecked())? };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_info::*;
    use tempfile::tempdir;

    fn make_vertex_info() -> VertexInfo {
        let mut props = PropertyVec::new();
        props.add_property(Property::new(
            "id",
            &DataType::int64(),
            true,
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
        let mut pgv = PropertyGroupVector::new();
        pgv.add_property_group(PropertyGroup::new(props, FileType::Parquet, ""));
        let ver = InfoVersion::new(1).unwrap();
        VertexInfo::new("person".into(), 4, pgv, vec![], "", ver)
    }

    fn make_edge_info() -> EdgeInfo {
        let mut adjs = AdjacentListVector::new();
        adjs.add_adjacent_list(AdjacentList::new(
            AdjListType::OrderedBySource,
            FileType::Orc,
            "",
        ));
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
            "props/",
        ));
        let ver = InfoVersion::new(1).unwrap();
        EdgeInfo::new(
            "person",
            "knows",
            "person",
            4,
            4,
            4,
            true,
            adjs,
            edge_prop_groups,
            "",
            ver,
        )
    }

    #[test]
    fn test_vertices_builder_add_and_dump() {
        let vi = make_vertex_info();
        let tmp = tempdir().unwrap();
        let mut vb = VerticesBuilder::new(&vi, tmp.path().join("vertex/"), 0).unwrap();

        let mut alice = VertexBuilder::new();
        alice.add_property("id".into(), 1_i64);
        alice.add_property("name".into(), "alice".to_string());
        vb.add_vertex(alice).unwrap();

        let mut bob = VertexBuilder::new();
        bob.add_property("id".into(), 2_i64);
        bob.add_property("name".into(), "bob".to_string());
        vb.add_vertex(bob).unwrap();

        vb.dump().unwrap();
        // success means FFI calls completed without error
    }

    #[test]
    fn test_edges_builder_add_and_dump() {
        let ei = make_edge_info();
        let tmp = tempdir().unwrap();
        let mut eb = EdgesBuilder::new(
            &ei,
            tmp.path().join("edge/"),
            AdjListType::OrderedBySource,
            2,
        )
        .unwrap();

        let mut e = EdgeBuilder::new(1, 2);
        e.add_property("friend".into(), "bob".to_string());
        eb.add_edge(e).unwrap();
    }

    #[test]
    fn test_vertices_builder_various_property_types() {
        // Build a VertexInfo with multiple property types
        let mut props = PropertyVec::new();
        props.add_property(Property::new(
            "id",
            &DataType::int64(),
            true,
            false,
            Cardinality::Single,
        ));
        props.add_property(Property::new(
            "active",
            &DataType::bool(),
            false,
            false,
            Cardinality::Single,
        ));
        props.add_property(Property::new(
            "age_i32",
            &DataType::int32(),
            false,
            false,
            Cardinality::Single,
        ));
        props.add_property(Property::new(
            "score_f32",
            &DataType::float32(),
            false,
            false,
            Cardinality::Single,
        ));
        props.add_property(Property::new(
            "rating_f64",
            &DataType::float64(),
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
        let mut pgv = PropertyGroupVector::new();
        pgv.add_property_group(PropertyGroup::new(props, FileType::Parquet, ""));
        let ver = InfoVersion::new(1).unwrap();
        let vi = VertexInfo::new("person".into(), 4, pgv, vec![], "", ver);

        let tmp = tempdir().unwrap();
        let mut vb = VerticesBuilder::new(&vi, tmp.path().join("vertex/"), 0).unwrap();

        let mut alice = VertexBuilder::new();
        alice.add_property("id".into(), 1_i64);
        alice.add_property("active".into(), true);
        alice.add_property("age_i32".into(), 30_i32);
        alice.add_property("score_f32".into(), 0.9_f32);
        alice.add_property("rating_f64".into(), 4.5_f64);
        alice.add_property("name".into(), "alice".to_string());
        vb.add_vertex(alice).unwrap();
        vb.dump().unwrap();
    }
}
