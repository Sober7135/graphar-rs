use std::ops::Range;

use anyhow::Result;
use cxx::{SharedPtr, UniquePtr, let_cxx_string};

use crate::{
    cxx_string_to_string,
    ffi::graphar::{self, vertex_iter_deref},
    graph_info::{AdjListType, GraphInfo},
};

pub struct Vertex {
    inner: UniquePtr<graphar::Vertex>,
}

impl Vertex {
    pub fn id(&self) -> i64 {
        self.inner.id()
    }

    pub fn is_valid(&self, property: &str) -> bool {
        let_cxx_string!(prop = property);
        self.inner.IsValid(&prop)
    }

    pub fn property<T>(&self, name: &str) -> Result<T>
    where
        (): SupportedPropertyType<T>,
    {
        <() as SupportedPropertyType<T>>::vertex_property(self, name)
    }
}

pub struct VertexIter {
    iter: UniquePtr<graphar::VertexIter>,
    len: usize,
}

impl VertexIter {
    pub fn id(&mut self) -> i64 {
        graphar::vertex_iter_id(self.iter.pin_mut())
    }

    pub fn has_label(&mut self, label: &str) -> Result<bool> {
        let_cxx_string!(label_cxx = label);
        Ok(graphar::vertex_iter_has_label(
            self.iter.pin_mut(),
            &label_cxx,
        )?)
    }

    pub fn labels(&mut self) -> Result<Vec<String>> {
        let labels = graphar::vertex_iter_labels(self.iter.pin_mut())?;
        let mut out = Vec::with_capacity(labels.len());
        for label in labels.iter() {
            out.push(cxx_string_to_string(label));
        }
        Ok(out)
    }

    pub fn next(&mut self) {
        graphar::vertex_iter_next(self.iter.pin_mut());
    }

    pub fn vertex(&mut self) -> Vertex {
        Vertex {
            inner: vertex_iter_deref(self.iter.pin_mut()),
        }
    }

    pub fn property<T>(&mut self, name: &str) -> Result<T>
    where
        (): SupportedPropertyType<T>,
    {
        <() as SupportedPropertyType<T>>::vertex_iter_property(self, name)
    }
}

impl PartialEq for VertexIter {
    fn eq(&self, other: &Self) -> bool {
        graphar::vertex_iter_eq(&self.iter, &other.iter)
    }
}

impl Eq for VertexIter {}

impl Iterator for VertexIter {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.id() as usize == self.len {
            None
        } else {
            let ret = self.vertex();
            self.next();
            Some(ret)
        }
    }
}

#[derive(Clone)]
pub struct Vertices {
    inner: SharedPtr<graphar::VerticesCollection>,
}

impl Vertices {
    pub fn new(graph_info: &GraphInfo, ty: &str) -> Result<Self> {
        let_cxx_string!(ty_cxx = ty);
        let inner = graphar::vertices_collection_make(&graph_info.inner, &ty_cxx)?;
        Ok(Self { inner })
    }

    pub fn with_label(graph_info: &GraphInfo, ty: &str, label: &str) -> Result<Self> {
        let_cxx_string!(ty_cxx = ty);
        let_cxx_string!(label_cxx = label);
        let inner =
            graphar::vertices_collection_with_label(&graph_info.inner, &ty_cxx, &label_cxx)?;
        Ok(Self { inner })
    }

    pub fn len(&self) -> usize {
        self.inner.size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn begin(&mut self) -> VertexIter {
        let begin = unsafe { graphar::vertices_collection_begin(self.inner.pin_mut_unchecked()) };

        VertexIter {
            iter: begin,
            len: self.len(),
        }
    }

    pub fn end(&mut self) -> VertexIter {
        let iter = unsafe { graphar::vertices_collection_end(self.inner.pin_mut_unchecked()) };
        VertexIter {
            iter,
            len: self.len(),
        }
    }

    pub fn find(&mut self, id: i64) -> VertexIter {
        let iter = unsafe { graphar::vertices_collection_find(self.inner.pin_mut_unchecked(), id) };
        VertexIter {
            iter,
            len: self.len(),
        }
    }
}

pub struct Edge {
    inner: UniquePtr<graphar::Edge>,
}

impl Edge {
    pub fn src(&self) -> i64 {
        self.inner.source()
    }

    pub fn dst(&self) -> i64 {
        self.inner.destination()
    }

    pub fn is_valid(&self, property: &str) -> bool {
        let_cxx_string!(prop = property);
        self.inner.IsValid(&prop)
    }

    pub fn property<T>(&self, name: &str) -> Result<T>
    where
        (): SupportedPropertyType<T>,
    {
        <() as SupportedPropertyType<T>>::edge_property(self, name)
    }
}

pub struct EdgeIter {
    iter: UniquePtr<graphar::EdgeIter>,
}

// TODO(wrapper for EdgeIter like VertexIter)
impl EdgeIter {
    pub fn source(&mut self) -> i64 {
        self.iter.pin_mut().source()
    }

    pub fn destination(&mut self) -> i64 {
        self.iter.pin_mut().destination()
    }

    pub fn next(&mut self) {
        graphar::edge_iter_next(self.iter.pin_mut());
    }

    pub fn to_begin(&mut self) {
        graphar::edge_iter_to_begin(self.iter.pin_mut());
    }

    pub fn next_src(&mut self) -> bool {
        graphar::edge_iter_next_src(self.iter.pin_mut())
    }

    pub fn next_dst(&mut self) -> bool {
        graphar::edge_iter_next_dst(self.iter.pin_mut())
    }

    pub fn next_src_with_id(&mut self, id: i64) -> bool {
        graphar::edge_iter_next_src_with_id(self.iter.pin_mut(), id)
    }

    pub fn next_dst_with_id(&mut self, id: i64) -> bool {
        graphar::edge_iter_next_dst_with_id(self.iter.pin_mut(), id)
    }

    pub fn global_chunk_index(&self) -> i64 {
        self.iter.global_chunk_index()
    }

    pub fn cur_offset(&self) -> i64 {
        self.iter.cur_offset()
    }

    pub fn is_end(&self) -> bool {
        self.iter.is_end()
    }

    pub fn property<T>(&mut self, name: &str) -> Result<T>
    where
        (): SupportedPropertyType<T>,
    {
        <() as SupportedPropertyType<T>>::edge_iter_property(self, name)
    }
}

impl PartialEq for EdgeIter {
    fn eq(&self, other: &Self) -> bool {
        graphar::edge_iter_eq(&self.iter, &other.iter)
    }
}

impl Eq for EdgeIter {}

#[derive(Clone)]
pub struct Edges {
    inner: SharedPtr<graphar::EdgesCollection>,
}

impl Edges {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph_info: &GraphInfo,
        src_type: &str,
        edge_type: &str,
        dst_type: &str,
        adj_list_type: AdjListType,
        vertex_chunks: Option<Range<i64>>,
    ) -> Result<Self> {
        let (chunk_begin, chunk_end) = vertex_chunks
            .map(|range| (range.start, range.end))
            .unwrap_or((0, i64::MAX));

        let_cxx_string!(src_cxx = src_type);
        let_cxx_string!(edge_cxx = edge_type);
        let_cxx_string!(dst_cxx = dst_type);

        let inner = graphar::edges_collection_make(
            &graph_info.inner,
            &src_cxx,
            &edge_cxx,
            &dst_cxx,
            adj_list_type,
            chunk_begin,
            chunk_end,
        )?;

        Ok(Self { inner })
    }

    pub fn len(&self) -> usize {
        self.inner.size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn begin(&mut self) -> EdgeIter {
        let iter = unsafe { graphar::edges_collection_begin(self.inner.pin_mut_unchecked()) };
        EdgeIter { iter }
    }

    pub fn end(&mut self) -> EdgeIter {
        let iter = unsafe { graphar::edges_collection_end(self.inner.pin_mut_unchecked()) };
        EdgeIter { iter }
    }

    pub fn find_src(&mut self, id: i64, from: &EdgeIter) -> EdgeIter {
        let iter = unsafe {
            graphar::edges_collection_find_src(self.inner.pin_mut_unchecked(), id, &from.iter)
        };
        EdgeIter { iter }
    }

    pub fn find_dst(&mut self, id: i64, from: &EdgeIter) -> EdgeIter {
        let iter = unsafe {
            graphar::edges_collection_find_dst(self.inner.pin_mut_unchecked(), id, &from.iter)
        };
        EdgeIter { iter }
    }
}

pub trait SupportedPropertyType<T> {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<T>;
    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<T>;
    fn edge_property(edge: &Edge, name: &str) -> Result<T>;
    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<T>;
}

impl SupportedPropertyType<bool> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<bool> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_bool(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<bool> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_bool(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<bool> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_bool(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<bool> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_bool(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }
}

impl SupportedPropertyType<i32> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<i32> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_i32(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<i32> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_i32(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<i32> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_i32(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<i32> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_i32(iter.iter.pin_mut(), &prop)?)
    }
}

impl SupportedPropertyType<i64> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<i64> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_i64(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<i64> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_i64(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<i64> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_i64(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<i64> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_i64(iter.iter.pin_mut(), &prop)?)
    }
}

impl SupportedPropertyType<f32> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<f32> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_f32(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<f32> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_f32(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<f32> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_f32(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<f32> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_f32(iter.iter.pin_mut(), &prop)?)
    }
}

impl SupportedPropertyType<f64> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<f64> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_f64(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<f64> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_f64(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<f64> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_f64(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<f64> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_f64(iter.iter.pin_mut(), &prop)?)
    }
}

impl SupportedPropertyType<String> for () {
    fn vertex_property(vertex: &Vertex, name: &str) -> Result<String> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_property_string(&vertex.inner, &prop)?)
    }

    fn vertex_iter_property(iter: &mut VertexIter, name: &str) -> Result<String> {
        let_cxx_string!(prop = name);
        Ok(graphar::vertex_iter_property_string(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }

    fn edge_property(edge: &Edge, name: &str) -> Result<String> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_property_string(&edge.inner, &prop)?)
    }

    fn edge_iter_property(iter: &mut EdgeIter, name: &str) -> Result<String> {
        let_cxx_string!(prop = name);
        Ok(graphar::edge_iter_property_string(
            iter.iter.pin_mut(),
            &prop,
        )?)
    }
}
