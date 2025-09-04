// Currently do not support cardinality

use std::{cell::UnsafeCell, path::Path};

use cxx::{CxxString, CxxVector, SharedPtr, UniquePtr, let_cxx_string};

use crate::ffi::{
    self,
    ffi::{
        Cardinality, CreateAdjacentList, CreateEdgeInfo, CreatePropertyGroup, boolean,
        create_vertex_info, date, edge_add_property_bool, edge_info_dump, edge_info_save,
        edges_dump, float32, float64, graph_info_dump, graph_info_save, int32, int64, list,
        load_graph_info, new_adjacent_list_vec, new_const_info_version, new_edges_builder,
        new_properties, new_property_group_vec, new_vertex, new_vertices_builder,
        push_adjacent_list, push_property, push_property_group, string, timestamp,
        vertex_add_property_bool, vertex_add_property_f32, vertex_add_property_f64,
        vertex_add_property_i32, vertex_add_property_i64, vertex_add_property_string,
        vertex_info_dump, vertex_info_save, vertices_dump,
    },
};

fn cxx_string_to_string(s: &CxxString) -> String {
    s.to_str()
        .map(|s| s.to_owned())
        .unwrap_or_else(|_| String::from_utf8_lossy(s.as_bytes()).into_owned())
}

#[derive(Debug, Clone)]
pub enum DataType {
    Bool,
    Int32,
    Int64,
    Float,
    Double,
    String,
    List(Box<DataType>),
    Date,
    Timestamp,
    UserDefined,
}

pub trait SupportedDataType<T> {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: T);
    fn edge_add_property(edge: &mut Edge, name: &str, val: T);
}

impl SupportedDataType<bool> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: bool) {
        let_cxx_string!(name = name);
        vertex_add_property_bool(vertex.inner.get_mut().pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: bool) {
        let_cxx_string!(name = name);
        edge_add_property_bool(edge.inner.get_mut().pin_mut(), &name, val);
    }
}

impl SupportedDataType<i32> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: i32) {
        let_cxx_string!(name = name);
        vertex_add_property_i32(vertex.inner.get_mut().pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: i32) {
        let_cxx_string!(name = name);
        ffi::ffi::edge_add_property_i32(edge.inner.get_mut().pin_mut(), &name, val);
    }
}

impl SupportedDataType<i64> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: i64) {
        let_cxx_string!(name = name);
        vertex_add_property_i64(vertex.inner.get_mut().pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: i64) {
        let_cxx_string!(name = name);
        ffi::ffi::edge_add_property_i64(edge.inner.get_mut().pin_mut(), &name, val);
    }
}

impl SupportedDataType<f32> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: f32) {
        let_cxx_string!(name = name);
        vertex_add_property_f32(vertex.inner.get_mut().pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: f32) {
        let_cxx_string!(name = name);
        ffi::ffi::edge_add_property_f32(edge.inner.get_mut().pin_mut(), &name, val);
    }
}

impl SupportedDataType<f64> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: f64) {
        let_cxx_string!(name = name);
        vertex_add_property_f64(vertex.inner.get_mut().pin_mut(), &name, val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: f64) {
        let_cxx_string!(name = name);
        ffi::ffi::edge_add_property_f64(edge.inner.get_mut().pin_mut(), &name, val);
    }
}

impl SupportedDataType<String> for () {
    fn vertex_add_property(vertex: &mut Vertex, name: &str, val: String) {
        let_cxx_string!(name = name);
        let_cxx_string!(val = val);
        vertex_add_property_string(vertex.inner.get_mut().pin_mut(), &name, &val);
    }

    fn edge_add_property(edge: &mut Edge, name: &str, val: String) {
        let_cxx_string!(name = name);
        let_cxx_string!(val = val);
        ffi::ffi::edge_add_property_string(edge.inner.get_mut().pin_mut(), &name, &val);
    }
}

impl<T> SupportedDataType<Vec<T>> for ()
where
    (): SupportedDataType<T>,
{
    fn vertex_add_property(_vertex: &mut Vertex, _name: &str, _val: Vec<T>) {
        todo!()
    }

    fn edge_add_property(_edge: &mut Edge, _name: &str, _val: Vec<T>) {
        todo!()
    }
}
// TODO(date, timestamp)

fn vertex_add_property<T, S: AsRef<str>>(vertex: &mut Vertex, name: S, val: T)
where
    (): SupportedDataType<T>,
{
    <() as SupportedDataType<T>>::vertex_add_property(vertex, name.as_ref(), val);
}

impl From<DataType> for SharedPtr<ffi::ffi::DataType> {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Bool => boolean().clone(),
            DataType::Int32 => int32().clone(),
            DataType::Int64 => int64().clone(),
            DataType::Float => float32().clone(),
            DataType::Double => float64().clone(),
            DataType::String => string().clone(),
            DataType::Timestamp => timestamp().clone(),
            DataType::Date => date().clone(),
            DataType::List(inner) => {
                let inner = (*inner).clone().into();
                list(&inner)
            }
            DataType::UserDefined => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Csv,
    Parquet,
    Orc,
    Json,
}

impl From<FileType> for ffi::ffi::FileType {
    fn from(value: FileType) -> Self {
        match value {
            FileType::Csv => ffi::ffi::FileType::CSV,
            FileType::Parquet => ffi::ffi::FileType::PARQUET,
            FileType::Orc => ffi::ffi::FileType::ORC,
            FileType::Json => ffi::ffi::FileType::JSON,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AdjListType {
    UnorderedBySource,
    UnorderedByDest,
    OrderedBySource,
    OrderedByDest,
}

impl From<AdjListType> for ffi::ffi::AdjListType {
    fn from(value: AdjListType) -> Self {
        match value {
            AdjListType::UnorderedBySource => ffi::ffi::AdjListType::unordered_by_source,
            AdjListType::UnorderedByDest => ffi::ffi::AdjListType::unordered_by_dest,
            AdjListType::OrderedBySource => ffi::ffi::AdjListType::ordered_by_source,
            AdjListType::OrderedByDest => ffi::ffi::AdjListType::ordered_by_dest,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub data_type: DataType,
    pub is_primary: bool,
    pub is_nullable: bool,
}

#[derive(Debug)]
pub struct PropertyVec {
    inner: UnsafeCell<UniquePtr<CxxVector<ffi::ffi::Property>>>,
}

impl Default for PropertyVec {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyVec {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(new_properties()),
        }
    }

    pub fn add_property(&mut self, property: Property) {
        let_cxx_string!(name = property.name);
        push_property(
            self.inner.get_mut().pin_mut(),
            &name,
            &property.data_type.into(),
            property.is_primary,
            property.is_nullable,
            Cardinality::SINGLE,
        );
    }
}

#[derive(Debug)]
pub struct PropertyGroup {
    inner: UnsafeCell<SharedPtr<ffi::ffi::PropertyGroup>>,
}

impl PropertyGroup {
    pub fn new<P: AsRef<Path>>(properties: PropertyVec, file_type: FileType, prefix: P) -> Self {
        let prefix_string = prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let properties_vec = properties.inner.into_inner();
        let props = properties_vec
            .as_ref()
            .expect("properties vec should be valid");
        let inner = CreatePropertyGroup(props, file_type.into(), &prefix);
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    // TODO(get_properties)

    pub fn has_property(&self, property_name: &str) -> bool {
        let_cxx_string!(name = property_name);

        unsafe { (&*self.inner.get()).HasProperty(&name) }
    }
}

#[derive(Debug)]
pub struct PropertyGroupVector {
    inner: UnsafeCell<UniquePtr<ffi::ffi::PropertyGroupVector>>,
}

impl Default for PropertyGroupVector {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyGroupVector {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(new_property_group_vec()),
        }
    }

    pub fn add_property_group(&mut self, property_group: PropertyGroup) {
        push_property_group(
            self.inner.get_mut().pin_mut(),
            property_group.inner.into_inner(),
        )
    }
}

#[derive(Debug)]
pub struct InfoVersion {
    inner: UnsafeCell<SharedPtr<ffi::ffi::ConstInfoVersion>>,
}

impl Clone for InfoVersion {
    fn clone(&self) -> Self {
        Self {
            inner: UnsafeCell::new(unsafe { (*self.inner.get()).clone() }),
        }
    }
}

impl InfoVersion {
    pub fn new(version: i32) -> anyhow::Result<Self> {
        Ok(Self {
            inner: UnsafeCell::new(new_const_info_version(version)?),
        })
    }
}

#[derive(Debug)]
pub struct VertexInfo {
    inner: UnsafeCell<SharedPtr<ffi::ffi::VertexInfo>>,
}

impl VertexInfo {
    // TODO(use &str for prefix)
    pub fn new<P: AsRef<Path>>(
        r#type: String,
        chunk_size: i64,
        property_groups: PropertyGroupVector,
        labels: Vec<String>,
        prefix: P,
        version: InfoVersion,
    ) -> Self {
        let inner = {
            let groups = property_groups.inner.into_inner();
            let groups_ref = groups.as_ref().expect("property group vec should be valid");
            let prefix_string = prefix.as_ref().to_string_lossy().into_owned();
            create_vertex_info(
                &r#type,
                chunk_size,
                groups_ref,
                &labels,
                &prefix_string,
                version.inner.into_inner(),
            )
        };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(path = path_string);
        vertex_info_save(unsafe { &*self.inner.get() }, &path)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(vertex_info_dump(unsafe { &*self.inner.get() }).map(|inner| inner.to_string())?)
    }
}

#[derive(Debug)]
pub struct Vertex {
    inner: UnsafeCell<UniquePtr<ffi::ffi::Vertex>>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(new_vertex()),
        }
    }

    pub fn add_property<T>(&mut self, name: String, property: T)
    where
        (): SupportedDataType<T>,
    {
        vertex_add_property(self, &name, property);
    }
}

#[derive(Debug)]
pub struct VerticesBuilder {
    inner: UnsafeCell<SharedPtr<ffi::ffi::VerticesBuilder>>,
}

impl VerticesBuilder {
    pub fn new<P: AsRef<Path>>(
        vertex_info: &VertexInfo,
        path_prefix: P,
        start_idx: i64,
    ) -> anyhow::Result<Self> {
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let inner = unsafe { &*vertex_info.inner.get() };
        Ok(Self {
            inner: UnsafeCell::new(new_vertices_builder(inner, &prefix, start_idx)?),
        })
    }

    pub fn add_vertex(&mut self, mut vertex: Vertex) -> anyhow::Result<()> {
        let builder = self.inner.get_mut();
        let v = vertex.inner.get_mut();
        unsafe { ffi::ffi::add_vertex(builder.pin_mut_unchecked(), v.pin_mut())? };
        Ok(())
    }

    pub fn dump(&mut self) -> anyhow::Result<()> {
        let builder = self.inner.get_mut();
        unsafe { vertices_dump(builder.pin_mut_unchecked())? };
        Ok(())
    }
}

#[derive(Debug)]
pub struct GraphInfo {
    inner: UnsafeCell<SharedPtr<ffi::ffi::GraphInfo>>,
}

impl GraphInfo {
    pub fn new(
        _name: String,
        _vertex_infos: &Vec<VertexInfo>,
        _edge_infos: &Vec<EdgeInfo>,
        _labels: &Vec<String>,
        _prefix: &String,
    ) -> Self {
        todo!()
    }

    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(p = path_string);
        let inner = load_graph_info(&p)?;
        Ok(Self {
            inner: UnsafeCell::new(inner),
        })
    }

    pub fn name(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetName() })
    }

    pub fn labels(&self) -> Vec<String> {
        let v = unsafe { (&*self.inner.get()).GetLabels() };
        let mut out = Vec::with_capacity(v.len());
        for cxx_string in v.iter() {
            out.push(cxx_string_to_string(cxx_string));
        }
        out
    }

    pub fn prefix(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetPrefix() })
    }

    pub fn version(&self) -> InfoVersion {
        let sp = unsafe { (&*self.inner.get()).version() };
        InfoVersion {
            inner: UnsafeCell::new(sp.clone()),
        }
    }

    pub fn vertex_info_num(&self) -> i32 {
        unsafe { (&*self.inner.get()).VertexInfoNum() }
    }

    pub fn edge_info_num(&self) -> i32 {
        unsafe { (&*self.inner.get()).EdgeInfoNum() }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(p = path_string);
        graph_info_save(unsafe { &*self.inner.get() }, &p)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(graph_info_dump(unsafe { &*self.inner.get() }).map(|u| u.to_string())?)
    }
}

#[derive(Debug)]
pub struct AdjacentList {
    inner: UnsafeCell<SharedPtr<ffi::ffi::AdjacentList>>,
}

impl AdjacentList {
    pub fn new<P: AsRef<Path>>(ty: AdjListType, file_type: FileType, path_prefix: P) -> Self {
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let inner = CreateAdjacentList(ty.into(), file_type.into(), &prefix);
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn list_type(&self) -> AdjListType {
        match unsafe { (&*self.inner.get()).GetType() } {
            ffi::ffi::AdjListType::unordered_by_source => AdjListType::UnorderedBySource,
            ffi::ffi::AdjListType::unordered_by_dest => AdjListType::UnorderedByDest,
            ffi::ffi::AdjListType::ordered_by_source => AdjListType::OrderedBySource,
            ffi::ffi::AdjListType::ordered_by_dest => AdjListType::OrderedByDest,
            _ => unreachable!(),
        }
    }

    pub fn file_type(&self) -> FileType {
        match unsafe { (&*self.inner.get()).GetFileType() } {
            ffi::ffi::FileType::CSV => FileType::Csv,
            ffi::ffi::FileType::PARQUET => FileType::Parquet,
            ffi::ffi::FileType::ORC => FileType::Orc,
            ffi::ffi::FileType::JSON => FileType::Json,
            _ => unreachable!(),
        }
    }

    pub fn prefix(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetPrefix() })
    }
}

#[derive(Debug)]
pub struct AdjacentListVector {
    inner: UnsafeCell<UniquePtr<ffi::ffi::AdjacentListVector>>,
}

impl Default for AdjacentListVector {
    fn default() -> Self {
        Self::new()
    }
}

impl AdjacentListVector {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(new_adjacent_list_vec()),
        }
    }

    pub fn add_adjacent_list(&mut self, adj_list: AdjacentList) {
        let vec = self.inner.get_mut();
        let adj = adj_list.inner.into_inner();
        push_adjacent_list(vec.pin_mut(), adj);
    }
}

#[derive(Debug)]
pub struct EdgeInfo {
    inner: UnsafeCell<SharedPtr<ffi::ffi::EdgeInfo>>,
}

impl EdgeInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new<P: AsRef<Path>>(
        src_type: &str,
        edge_type: &str,
        dst_type: &str,
        chunk_size: i64,
        src_chunk_size: i64,
        dst_chunk_size: i64,
        directed: bool,
        adjacent_lists: AdjacentListVector,
        property_groups: PropertyGroupVector,
        path_prefix: P,
        version: InfoVersion,
    ) -> Self {
        let_cxx_string!(src = src_type);
        let_cxx_string!(edge = edge_type);
        let_cxx_string!(dst = dst_type);
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);

        let adj_vec = adjacent_lists.inner.into_inner();
        let adj_ref = adj_vec.as_ref().expect("adjacent list vec should be valid");
        let prop_vec = property_groups.inner.into_inner();
        let prop_ref = prop_vec
            .as_ref()
            .expect("property group vec should be valid");

        let inner = CreateEdgeInfo(
            &src,
            &edge,
            &dst,
            chunk_size,
            src_chunk_size,
            dst_chunk_size,
            directed,
            adj_ref,
            prop_ref,
            &prefix,
            version.inner.into_inner(),
        );
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(p = path_string);
        edge_info_save(unsafe { &*self.inner.get() }, &p)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(edge_info_dump(unsafe { &*self.inner.get() }).map(|u| u.to_string())?)
    }

    pub fn src_type(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetSrcType() })
    }
    pub fn dst_type(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetDstType() })
    }
    pub fn edge_type(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetEdgeType() })
    }
    pub fn chunk_size(&self) -> i64 {
        unsafe { (&*self.inner.get()).GetChunkSize() }
    }
    pub fn src_chunk_size(&self) -> i64 {
        unsafe { (&*self.inner.get()).GetSrcChunkSize() }
    }
    pub fn dst_chunk_size(&self) -> i64 {
        unsafe { (&*self.inner.get()).GetDstChunkSize() }
    }
    pub fn prefix(&self) -> String {
        cxx_string_to_string(unsafe { (&*self.inner.get()).GetPrefix() })
    }
    pub fn directed(&self) -> bool {
        unsafe { (&*self.inner.get()).IsDirected() }
    }
}

#[derive(Debug)]
pub struct Edge {
    inner: UnsafeCell<UniquePtr<ffi::ffi::Edge>>,
}

impl Edge {
    pub fn new(src_id: i64, dst_id: i64) -> Self {
        Self {
            inner: UnsafeCell::new(ffi::ffi::new_edge(src_id, dst_id)),
        }
    }

    pub fn add_property<T>(&mut self, name: String, property: T)
    where
        (): SupportedDataType<T>,
    {
        <() as SupportedDataType<T>>::edge_add_property(self, &name, property);
    }
}

#[derive(Debug)]
pub struct EdgesBuilder {
    inner: UnsafeCell<SharedPtr<ffi::ffi::EdgesBuilder>>,
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
        let inner = new_edges_builder(
            unsafe { &*edge_info.inner.get() },
            &prefix,
            adj_list_type.into(),
            vertices_num,
        )?;
        Ok(Self {
            inner: UnsafeCell::new(inner),
        })
    }

    pub fn add_edge(&mut self, mut edge: Edge) -> anyhow::Result<()> {
        let builder = self.inner.get_mut();
        let e = edge.inner.get_mut();
        unsafe { ffi::ffi::add_edge(builder.pin_mut_unchecked(), e.pin_mut())? };
        Ok(())
    }

    pub fn dump(&mut self) -> anyhow::Result<()> {
        let builder = self.inner.get_mut();
        unsafe { edges_dump(builder.pin_mut_unchecked())? };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut vertex_props = PropertyVec::new();

        // Add id
        vertex_props.add_property(Property {
            name: "id".into(),
            data_type: DataType::Int64,
            is_primary: false,
            is_nullable: false,
        });
        // Add name
        vertex_props.add_property(Property {
            name: "name".into(),
            data_type: DataType::String,
            is_primary: false,
            is_nullable: false,
        });

        let vertex_prop_group = PropertyGroup::new(vertex_props, FileType::Csv, "");
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
        let mut alice = Vertex::new();
        alice.add_property("id".into(), 1_i64);
        alice.add_property("name".into(), "alice".to_string());

        let mut bob = Vertex::new();
        bob.add_property("id".into(), 2_i64);
        bob.add_property("name".into(), "bob".to_string());
        vb.add_vertex(alice).unwrap();
        vb.add_vertex(bob).unwrap();
        vb.dump().unwrap();

        // `EdgeInfo`
        let mut adjs = AdjacentListVector::new();
        let adj = AdjacentList::new(AdjListType::OrderedBySource, FileType::Csv, "");
        adjs.add_adjacent_list(adj);

        let mut edge_props = PropertyVec::new();
        edge_props.add_property(Property {
            name: "friend".into(),
            data_type: DataType::String,
            is_primary: false,
            is_nullable: true,
        });
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
        let mut e = Edge::new(1, 2);
        e.add_property("friend".into(), "bob".to_string());
        edge_builder.add_edge(e).unwrap();
        edge_builder.dump().unwrap();

        // GraphInfo
    }
}
