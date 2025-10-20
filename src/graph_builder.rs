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
