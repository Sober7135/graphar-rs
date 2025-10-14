// Currently do not support cardinality

pub use ffi::graphar::{AdjListType, Cardinality, FileType, Type};
use std::{
    fmt::{Debug, Display},
    path::Path,
};

use cxx::{CxxVector, SharedPtr, UniquePtr, let_cxx_string};

use crate::{
    cxx_string_to_string,
    ffi::{
        self, SharedPropertyGroup, SharedVertexInfo,
        graphar::{
            CreateAdjacentList, CreateEdgeInfo, CreatePropertyGroup, boolean, create_graph_info,
            create_vertex_info, date, edge_info_dump, edge_info_save, float32, float64,
            graph_info_dump, graph_info_save, int32, int64, list, load_graph_info,
            new_adjacent_list_vec, new_const_info_version, new_properties, new_property,
            new_property_group_vec, property_clone, property_get_name, property_get_type,
            push_adjacent_list, push_property, push_property_group, string, timestamp,
            to_type_name, vertex_info_dump, vertex_info_save,
        },
    },
};

#[derive(Clone)]
pub struct DataType {
    inner: SharedPtr<ffi::graphar::DataType>,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.is_null() && other.inner.is_null() {
            return true;
        }
        if self.inner.is_null() || other.inner.is_null() {
            return false;
        }

        self.inner
            .Equals(other.inner.as_ref().expect("rhs is nullptr"))
    }
}

impl Eq for DataType {}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inner.is_null() {
            write!(f, "null")
        } else {
            write!(f, "{}", to_type_name(&self.inner))
        }
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inner.is_null() {
            write!(f, "null")
        } else {
            write!(f, "{}", to_type_name(&self.inner))
        }
    }
}

impl DataType {
    pub fn value_type(&self) -> Self {
        DataType {
            inner: self.inner.value_type().clone(),
        }
    }

    pub fn id(&self) -> Type {
        self.inner.id()
    }

    pub fn bool() -> Self {
        Self {
            inner: boolean().clone(),
        }
    }

    pub fn int32() -> Self {
        Self {
            inner: int32().clone(),
        }
    }

    pub fn int64() -> Self {
        Self {
            inner: int64().clone(),
        }
    }

    pub fn float32() -> Self {
        Self {
            inner: float32().clone(),
        }
    }

    pub fn float64() -> Self {
        Self {
            inner: float64().clone(),
        }
    }

    pub fn string() -> Self {
        Self {
            inner: string().clone(),
        }
    }

    pub fn date() -> Self {
        Self {
            inner: date().clone(),
        }
    }

    pub fn timestamp() -> Self {
        Self {
            inner: timestamp().clone(),
        }
    }

    pub fn list(value_type: &DataType) -> Self {
        Self {
            inner: list(&value_type.inner),
        }
    }
}

pub struct Property {
    inner: UniquePtr<ffi::graphar::Property>,
}

impl Property {
    //
    pub fn new<S: AsRef<str>>(
        name: S,
        data_type: &DataType,
        is_primary: bool,
        is_nullable: bool,
        cardinality: Cardinality,
    ) -> Self {
        let_cxx_string!(name = name.as_ref());
        Self {
            inner: new_property(
                &name,
                &data_type.inner,
                is_primary,
                is_nullable,
                cardinality,
            ),
        }
    }

    pub fn name(&self) -> String {
        cxx_string_to_string(property_get_name(&self.inner))
    }

    pub fn data_type(&self) -> DataType {
        let ty = property_get_type(&self.inner);
        DataType { inner: ty.clone() }
    }
}

// TODO(how to design this)
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
        push_property(self.inner.pin_mut(), property.inner);
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
        let inner = CreatePropertyGroup(props, file_type, &prefix);
        Self { inner }
    }

    pub fn properties(&self) -> Vec<Property> {
        let props_cxx = self.inner.GetProperties();
        let mut props = Vec::with_capacity(props_cxx.len());
        for prop in props_cxx {
            props.push(Property {
                inner: property_clone(prop),
            });
        }

        props
    }

    pub fn has_property(&self, property_name: &str) -> bool {
        let_cxx_string!(name = property_name);

        self.inner.HasProperty(&name)
    }
}

pub struct PropertyGroupVector {
    inner: UniquePtr<CxxVector<SharedPropertyGroup>>,
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

    pub fn ty(&self) -> String {
        cxx_string_to_string(self.inner.GetType())
    }

    pub fn chunk_size(&self) -> i64 {
        self.inner.GetChunkSize()
    }

    pub fn prefix(&self) -> String {
        cxx_string_to_string(self.inner.GetPrefix())
    }

    pub fn version(&self) -> InfoVersion {
        InfoVersion {
            inner: self.inner.version().clone(),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let labels_cxx = self.inner.GetLabels();
        let mut labels = Vec::with_capacity(labels_cxx.len());
        for label in labels_cxx {
            labels.push(cxx_string_to_string(label));
        }

        labels
    }

    pub fn property_group_num(&self) -> i32 {
        self.inner.PropertyGroupNum()
    }

    pub fn property_groups(&self) -> Vec<PropertyGroup> {
        let pgs_cxx = self.inner.GetPropertyGroups();
        let mut pgs = Vec::with_capacity(pgs_cxx.len());
        for pg in pgs_cxx.iter() {
            pgs.push(PropertyGroup {
                inner: pg.inner.clone(),
            });
        }

        pgs
    }

    pub fn property_group<S: AsRef<str>>(&self, property_name: S) -> PropertyGroup {
        let_cxx_string!(name = property_name.as_ref());
        PropertyGroup {
            inner: self.inner.GetPropertyGroup(&name),
        }
    }

    pub fn property_group_by_index(&self, index: i32) -> PropertyGroup {
        PropertyGroup {
            inner: self.inner.GetPropertyGroupByIndex(index),
        }
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
    pub(crate) inner: SharedPtr<ffi::graphar::GraphInfo>,
}

impl GraphInfo {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(
        name: S,
        vertex_infos: &Vec<VertexInfo>,
        edge_infos: &Vec<EdgeInfo>,
        labels: &Vec<String>,
        prefix: P,
        version: Option<InfoVersion>,
        // TODO(extra_info)
    ) -> Self {
        let_cxx_string!(name = name.as_ref());
        let_cxx_string!(prefix = prefix.as_ref().to_string_lossy().into_owned());

        let mut v_infos = CxxVector::new();
        v_infos.pin_mut().reserve(vertex_infos.len());
        for info in vertex_infos {
            v_infos.pin_mut().push(SharedVertexInfo {
                inner: info.inner.clone(),
            });
        }

        let mut e_infos = CxxVector::new();
        e_infos.pin_mut().reserve(edge_infos.len());
        for info in edge_infos {
            e_infos.pin_mut().push(ffi::SharedEdgeInfo {
                inner: info.inner.clone(),
            });
        }

        let version = if let Some(version) = version {
            version.inner
        } else {
            SharedPtr::null()
        };

        Self {
            inner: create_graph_info(
                &name,
                v_infos.as_ref().expect("vertex_infos is null"),
                e_infos.as_ref().expect("edge_infos is null"),
                labels,
                &prefix,
                version,
            ),
        }
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

    pub fn vertex_info<S: AsRef<str>>(&self, r#type: S) -> VertexInfo {
        let_cxx_string!(ty = r#type.as_ref());
        VertexInfo {
            inner: self.inner.GetVertexInfo(&ty),
        }
    }

    pub fn edge_info<S: AsRef<str>>(&self, src_type: S, edge_type: S, dst_type: S) -> EdgeInfo {
        let_cxx_string!(src_type = src_type.as_ref());
        let_cxx_string!(edge_type = edge_type.as_ref());
        let_cxx_string!(dst_type = dst_type.as_ref());

        EdgeInfo {
            inner: self.inner.GetEdgeInfo(&src_type, &edge_type, &dst_type),
        }
    }

    pub fn vertex_info_index<S: AsRef<str>>(&self, r#type: S) -> i32 {
        let_cxx_string!(ty = r#type.as_ref());
        self.inner.GetVertexInfoIndex(&ty)
    }

    pub fn edge_info_index<S: AsRef<str>>(&self, src_type: S, edge_type: S, dst_type: S) -> i32 {
        let_cxx_string!(src_type = src_type.as_ref());
        let_cxx_string!(edge_type = edge_type.as_ref());
        let_cxx_string!(dst_type = dst_type.as_ref());

        self.inner
            .GetEdgeInfoIndex(&src_type, &edge_type, &dst_type)
    }

    pub fn vertex_infos(&self) -> Vec<VertexInfo> {
        let vec = self.inner.GetVertexInfos();
        let mut out = Vec::with_capacity(vec.len());
        for item in vec {
            out.push(VertexInfo {
                inner: item.inner.clone(),
            });
        }

        out
    }

    pub fn edge_infos(&self) -> Vec<EdgeInfo> {
        let vec = self.inner.GetEdgeInfos();
        let mut out = Vec::with_capacity(vec.len());
        for item in vec {
            out.push(EdgeInfo {
                inner: item.inner.clone(),
            });
        }

        out
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
        let inner = CreateAdjacentList(ty, file_type, &prefix);
        Self { inner }
    }

    pub fn list_type(&self) -> AdjListType {
        self.inner.GetType()
    }

    pub fn file_type(&self) -> FileType {
        self.inner.GetFileType()
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

    pub fn is_directed(&self) -> bool {
        self.inner.IsDirected()
    }

    pub fn version(&self) -> InfoVersion {
        InfoVersion {
            inner: self.inner.version().clone(),
        }
    }

    pub fn has_adjacent_list_type(&self, adj_list_type: AdjListType) -> bool {
        self.inner.HasAdjacentListType(adj_list_type)
    }

    pub fn adjacent_list(&self, adj_list_type: AdjListType) -> AdjacentList {
        AdjacentList {
            inner: self.inner.GetAdjacentList(adj_list_type),
        }
    }

    pub fn property_group_num(&self) -> i32 {
        self.inner.PropertyGroupNum()
    }

    pub fn property_groups(&self) -> Vec<PropertyGroup> {
        let pgs_cxx = self.inner.GetPropertyGroups();
        let mut pgs = Vec::with_capacity(pgs_cxx.len());
        for pg in pgs_cxx.iter() {
            pgs.push(PropertyGroup {
                inner: pg.inner.clone(),
            });
        }

        pgs
    }

    pub fn property_group<S: AsRef<str>>(&self, property_name: S) -> PropertyGroup {
        let_cxx_string!(name = property_name.as_ref());
        PropertyGroup {
            inner: self.inner.GetPropertyGroup(&name),
        }
    }

    pub fn property_group_by_index(&self, index: i32) -> PropertyGroup {
        PropertyGroup {
            inner: self.inner.GetPropertyGroupByIndex(index),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    impl DataType {
        fn null() -> Self {
            Self {
                inner: SharedPtr::null(),
            }
        }
    }

    #[test]
    fn test_data_type_equality() {
        let float = DataType::float32();
        let float1 = DataType::float32();
        assert_eq!(float, float1);

        let double = DataType::float64();
        assert_ne!(float, double);

        let list_of_float_1 = DataType::list(&float);
        let list_of_float_2 = DataType::list(&float);
        assert_eq!(list_of_float_1, list_of_float_2);

        let list_of_double = DataType::list(&double);
        assert_ne!(list_of_float_1, list_of_double);
    }

    #[test]
    fn test_data_type_display() {
        let bool_type = DataType::bool();
        assert_eq!(format!("{}", bool_type), "bool");

        let int32 = DataType::int32();
        assert_eq!(format!("{}", int32), "int32");

        let int64 = DataType::int64();
        assert_eq!(format!("{}", int64), "int64");

        let float32 = DataType::float32();
        assert_eq!(format!("{}", float32), "float");

        let float64 = DataType::float64();
        assert_eq!(format!("{}", float64), "double");

        let string = DataType::string();
        assert_eq!(format!("{}", string), "string");

        let date = DataType::date();
        assert_eq!(format!("{}", date), "date");

        let timestamp = DataType::timestamp();
        assert_eq!(format!("{}", timestamp), "timestamp");

        let list_of_int32 = DataType::list(&int32);
        assert_eq!(format!("{}", list_of_int32), "list<int32>");

        let nested_list = DataType::list(&list_of_int32);
        assert_eq!(format!("{}", nested_list), "list<list<int32>>");

        assert_eq!(format!("{}", int32.value_type()), "null");
    }

    #[test]
    fn test_data_type_debug() {
        let bool_type = DataType::bool();
        assert_eq!(format!("{:?}", bool_type), "bool");

        let int32 = DataType::int32();
        assert_eq!(format!("{:?}", int32), "int32");

        let int64 = DataType::int64();
        assert_eq!(format!("{:?}", int64), "int64");

        let float32 = DataType::float32();
        assert_eq!(format!("{:?}", float32), "float");

        let float64 = DataType::float64();
        assert_eq!(format!("{:?}", float64), "double");

        let string = DataType::string();
        assert_eq!(format!("{:?}", string), "string");

        let date = DataType::date();
        assert_eq!(format!("{:?}", date), "date");

        let timestamp = DataType::timestamp();
        assert_eq!(format!("{:?}", timestamp), "timestamp");

        let list_of_int32 = DataType::list(&int32);
        assert_eq!(format!("{:?}", list_of_int32), "list<int32>");

        let nested_list = DataType::list(&list_of_int32);
        assert_eq!(format!("{:?}", nested_list), "list<list<int32>>");

        assert_eq!(format!("{:?}", int32.value_type()), "null");
    }

    #[test]
    fn test_data_type_value_type() {
        let bool_type = DataType::bool();
        assert_eq!(bool_type.value_type(), DataType::null());

        let int32 = DataType::int32();
        assert_eq!(int32.value_type(), DataType::null());

        let int64 = DataType::int64();
        assert_eq!(int64.value_type(), DataType::null());

        let float32 = DataType::float32();
        assert_eq!(float32.value_type(), DataType::null());

        let float64 = DataType::float64();
        assert_eq!(float64.value_type(), DataType::null());

        let string = DataType::string();
        assert_eq!(string.value_type(), DataType::null());

        let date = DataType::date();
        assert_eq!(date.value_type(), DataType::null());

        let timestamp = DataType::timestamp();
        assert_eq!(timestamp.value_type(), DataType::null());

        let list_of_int32 = DataType::list(&int32);
        assert_eq!(list_of_int32.value_type(), int32);

        let list_of_float32 = DataType::list(&float32);
        assert_eq!(list_of_float32.value_type(), float32);

        let list_of_string = DataType::list(&string);
        assert_eq!(list_of_string.value_type(), string);

        let nested_list = DataType::list(&list_of_int32);
        assert_eq!(nested_list.value_type(), list_of_int32);
    }

    #[test]
    fn test_data_type_id() {
        let bool_type = DataType::bool();
        assert_eq!(bool_type.id(), Type::Bool);

        let int32 = DataType::int32();
        assert_eq!(int32.id(), Type::Int32);

        let int64 = DataType::int64();
        assert_eq!(int64.id(), Type::Int64);

        let float32 = DataType::float32();
        assert_eq!(float32.id(), Type::Float);

        let float64 = DataType::float64();
        assert_eq!(float64.id(), Type::Double);

        let string = DataType::string();
        assert_eq!(string.id(), Type::String);

        let date = DataType::date();
        assert_eq!(date.id(), Type::Date);

        let timestamp = DataType::timestamp();
        assert_eq!(timestamp.id(), Type::Timestamp);

        let list_of_int32 = DataType::list(&int32);
        assert_eq!(list_of_int32.id(), Type::List);
        assert_eq!(list_of_int32.value_type().id(), Type::Int32);

        let list_of_lists = DataType::list(&list_of_int32);
        assert_eq!(list_of_lists.id(), Type::List);
        assert_eq!(list_of_lists.value_type().id(), Type::List);
    }
}
