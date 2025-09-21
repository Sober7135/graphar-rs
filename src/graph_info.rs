// Currently do not support cardinality

use std::path::Path;

use cxx::{CxxString, CxxVector, SharedPtr, UniquePtr, let_cxx_string};

use crate::ffi::{
    self,
    graphar::{
        Cardinality, CreateAdjacentList, CreateEdgeInfo, CreatePropertyGroup, boolean,
        create_vertex_info, date, edge_info_dump, edge_info_save, float32, float64,
        graph_info_dump, graph_info_save, int32, int64, list, load_graph_info,
        new_adjacent_list_vec, new_const_info_version, new_properties, new_property_group_vec,
        push_adjacent_list, push_property, push_property_group, string, timestamp,
        vertex_info_dump, vertex_info_save,
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

impl From<DataType> for SharedPtr<ffi::graphar::DataType> {
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

impl From<FileType> for ffi::graphar::FileType {
    fn from(value: FileType) -> Self {
        match value {
            FileType::Csv => ffi::graphar::FileType::CSV,
            FileType::Parquet => ffi::graphar::FileType::PARQUET,
            FileType::Orc => ffi::graphar::FileType::ORC,
            FileType::Json => ffi::graphar::FileType::JSON,
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

impl From<AdjListType> for ffi::graphar::AdjListType {
    fn from(value: AdjListType) -> Self {
        match value {
            AdjListType::UnorderedBySource => ffi::graphar::AdjListType::unordered_by_source,
            AdjListType::UnorderedByDest => ffi::graphar::AdjListType::unordered_by_dest,
            AdjListType::OrderedBySource => ffi::graphar::AdjListType::ordered_by_source,
            AdjListType::OrderedByDest => ffi::graphar::AdjListType::ordered_by_dest,
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

pub struct PropertyVec {
    inner: UniquePtr<CxxVector<ffi::graphar::Property>>,
}

impl Default for PropertyVec {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyVec {
    pub fn new() -> Self {
        Self {
            inner: new_properties(),
        }
    }

    pub fn add_property(&mut self, property: Property) {
        let_cxx_string!(name = property.name);
        push_property(
            self.inner.pin_mut(),
            &name,
            &property.data_type.into(),
            property.is_primary,
            property.is_nullable,
            Cardinality::SINGLE,
        );
    }
}

pub struct PropertyGroup {
    inner: SharedPtr<ffi::graphar::PropertyGroup>,
}

impl PropertyGroup {
    pub fn new<P: AsRef<Path>>(properties: PropertyVec, file_type: FileType, prefix: P) -> Self {
        let prefix_string = prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let properties_vec = properties.inner;
        let props = properties_vec
            .as_ref()
            .expect("properties vec should be valid");
        let inner = CreatePropertyGroup(props, file_type.into(), &prefix);
        Self { inner }
    }

    // TODO(get_properties)

    pub fn has_property(&self, property_name: &str) -> bool {
        let_cxx_string!(name = property_name);

        self.inner.HasProperty(&name)
    }
}

pub struct PropertyGroupVector {
    inner: UniquePtr<ffi::graphar::PropertyGroupVector>,
}

impl Default for PropertyGroupVector {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyGroupVector {
    pub fn new() -> Self {
        Self {
            inner: new_property_group_vec(),
        }
    }

    pub fn add_property_group(&mut self, property_group: PropertyGroup) {
        push_property_group(self.inner.pin_mut(), property_group.inner)
    }
}

pub struct InfoVersion {
    inner: SharedPtr<ffi::graphar::ConstInfoVersion>,
}

impl Clone for InfoVersion {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl InfoVersion {
    pub fn new(version: i32) -> anyhow::Result<Self> {
        Ok(Self {
            inner: new_const_info_version(version)?,
        })
    }
}

pub struct VertexInfo {
    pub(crate) inner: SharedPtr<ffi::graphar::VertexInfo>,
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
            let groups = property_groups.inner;
            let groups_ref = groups.as_ref().expect("property group vec should be valid");
            let prefix_string = prefix.as_ref().to_string_lossy().into_owned();
            create_vertex_info(
                &r#type,
                chunk_size,
                groups_ref,
                &labels,
                &prefix_string,
                version.inner,
            )
        };
        Self { inner }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(path = path_string);
        vertex_info_save(&self.inner, &path)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(vertex_info_dump(&self.inner).map(|inner| inner.to_string())?)
    }
}

pub struct GraphInfo {
    inner: SharedPtr<ffi::graphar::GraphInfo>,
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
        Ok(Self { inner })
    }

    pub fn name(&self) -> String {
        cxx_string_to_string(self.inner.GetName())
    }

    pub fn labels(&self) -> Vec<String> {
        let v = self.inner.GetLabels();
        let mut out = Vec::with_capacity(v.len());
        for cxx_string in v.iter() {
            out.push(cxx_string_to_string(cxx_string));
        }
        out
    }

    pub fn prefix(&self) -> String {
        cxx_string_to_string(self.inner.GetPrefix())
    }

    pub fn version(&self) -> InfoVersion {
        let sp = self.inner.version();
        InfoVersion { inner: sp.clone() }
    }

    pub fn vertex_info_num(&self) -> i32 {
        self.inner.VertexInfoNum()
    }

    pub fn edge_info_num(&self) -> i32 {
        self.inner.EdgeInfoNum()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(p = path_string);
        graph_info_save(&self.inner, &p)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(graph_info_dump(&self.inner).map(|u| u.to_string())?)
    }
}

pub struct AdjacentList {
    inner: SharedPtr<ffi::graphar::AdjacentList>,
}

impl AdjacentList {
    pub fn new<P: AsRef<Path>>(ty: AdjListType, file_type: FileType, path_prefix: P) -> Self {
        let prefix_string = path_prefix.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(prefix = prefix_string);
        let inner = CreateAdjacentList(ty.into(), file_type.into(), &prefix);
        Self { inner }
    }

    pub fn list_type(&self) -> AdjListType {
        match self.inner.GetType() {
            ffi::graphar::AdjListType::unordered_by_source => AdjListType::UnorderedBySource,
            ffi::graphar::AdjListType::unordered_by_dest => AdjListType::UnorderedByDest,
            ffi::graphar::AdjListType::ordered_by_source => AdjListType::OrderedBySource,
            ffi::graphar::AdjListType::ordered_by_dest => AdjListType::OrderedByDest,
            _ => unreachable!(),
        }
    }

    pub fn file_type(&self) -> FileType {
        match self.inner.GetFileType() {
            ffi::graphar::FileType::CSV => FileType::Csv,
            ffi::graphar::FileType::PARQUET => FileType::Parquet,
            ffi::graphar::FileType::ORC => FileType::Orc,
            ffi::graphar::FileType::JSON => FileType::Json,
            _ => unreachable!(),
        }
    }

    pub fn prefix(&self) -> String {
        cxx_string_to_string(self.inner.GetPrefix())
    }
}

pub struct AdjacentListVector {
    inner: UniquePtr<ffi::graphar::AdjacentListVector>,
}

impl Default for AdjacentListVector {
    fn default() -> Self {
        Self::new()
    }
}

impl AdjacentListVector {
    pub fn new() -> Self {
        Self {
            inner: new_adjacent_list_vec(),
        }
    }

    pub fn add_adjacent_list(&mut self, adj_list: AdjacentList) {
        let adj = adj_list.inner;
        push_adjacent_list(self.inner.pin_mut(), adj);
    }
}

pub struct EdgeInfo {
    pub(crate) inner: SharedPtr<ffi::graphar::EdgeInfo>,
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

        let adj_vec = adjacent_lists.inner;
        let adj_ref = adj_vec.as_ref().expect("adjacent list vec should be valid");
        let prop_vec = property_groups.inner;
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
            version.inner,
        );
        Self { inner }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path_string = path.as_ref().to_string_lossy().into_owned();
        let_cxx_string!(p = path_string);
        edge_info_save(&self.inner, &p)?;
        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<String> {
        Ok(edge_info_dump(&self.inner).map(|u| u.to_string())?)
    }

    pub fn src_type(&self) -> String {
        cxx_string_to_string(self.inner.GetSrcType())
    }
    pub fn dst_type(&self) -> String {
        cxx_string_to_string(self.inner.GetDstType())
    }
    pub fn edge_type(&self) -> String {
        cxx_string_to_string(self.inner.GetEdgeType())
    }
    pub fn chunk_size(&self) -> i64 {
        self.inner.GetChunkSize()
    }
    pub fn src_chunk_size(&self) -> i64 {
        self.inner.GetSrcChunkSize()
    }
    pub fn dst_chunk_size(&self) -> i64 {
        self.inner.GetDstChunkSize()
    }
    pub fn prefix(&self) -> String {
        cxx_string_to_string(self.inner.GetPrefix())
    }
    pub fn directed(&self) -> bool {
        self.inner.IsDirected()
    }
}

#[cfg(test)]
mod tests {

    use crate::graph_builder::{self, EdgesBuilder, VerticesBuilder};

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
        let mut alice = graph_builder::Vertex::new();
        alice.add_property("id".into(), 1_i64);
        alice.add_property("name".into(), "alice".to_string());

        let mut bob = graph_builder::Vertex::new();
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
        let mut e = graph_builder::Edge::new(1, 2);
        e.add_property("friend".into(), "bob".to_string());
        edge_builder.add_edge(e).unwrap();
        edge_builder.dump().unwrap();

        // GraphInfo
    }
}
