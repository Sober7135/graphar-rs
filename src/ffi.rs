#![allow(unused)]

#[allow(clippy::module_inception)]
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "graphar"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    enum FileType {
        CSV = 0,
        PARQUET = 1,
        ORC = 2,
        JSON = 3,
    }

    #[namespace = "graphar"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    enum Cardinality {
        SINGLE = 0,
        LIST = 1,
        SET = 2,
    }

    #[namespace = "graphar"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    enum AdjListType {
        unordered_by_source = 0b00000001,
        unordered_by_dest = 0b00000010,
        ordered_by_source = 0b00000100,
        ordered_by_dest = 0b00001000,
    }

    // Enum
    #[namespace = "graphar"]
    unsafe extern "C++" {
        type FileType;
        type Cardinality;
        type AdjListType;
    }

    // `DataType`
    #[namespace = "graphar"]
    unsafe extern "C++" {
        type DataType;

        fn int32() -> &'static SharedPtr<DataType>;
        fn boolean() -> &'static SharedPtr<DataType>;
        fn int64() -> &'static SharedPtr<DataType>;
        fn float32() -> &'static SharedPtr<DataType>;
        fn float64() -> &'static SharedPtr<DataType>;
        fn string() -> &'static SharedPtr<DataType>;
        fn date() -> &'static SharedPtr<DataType>;
        fn timestamp() -> &'static SharedPtr<DataType>;
        fn list(inner: &SharedPtr<DataType>) -> SharedPtr<DataType>;
    }

    // `InfoVersion`
    #[namespace = "graphar"]
    unsafe extern "C++" {
        #[namespace = "graphar"]
        type InfoVersion;

        // see https://github.com/dtolnay/cxx/issues/850
        #[namespace = "graphar"]
        type ConstInfoVersion;

        #[namespace = "graphar_rs"]
        fn new_info_version(version: i32) -> Result<SharedPtr<InfoVersion>>;

        #[namespace = "graphar_rs"]
        fn new_const_info_version(version: i32) -> Result<SharedPtr<ConstInfoVersion>>;

    }

    // `GraphInfo`
    unsafe extern "C++" {
        include!("graphar/include/graphar_rs.h");

        #[namespace = "graphar"]
        type GraphInfo;

        fn GetName(&self) -> &CxxString;
        fn GetLabels(&self) -> &CxxVector<CxxString>;
        fn GetPrefix(&self) -> &CxxString;
        fn version(&self) -> &SharedPtr<ConstInfoVersion>;

        fn VertexInfoNum(&self) -> i32;
        fn EdgeInfoNum(&self) -> i32;

        #[namespace = "graphar_rs"]
        fn load_graph_info(path: &CxxString) -> Result<SharedPtr<GraphInfo>>;

        #[namespace = "graphar_rs"]
        fn graph_info_save(graph_info: &GraphInfo, path: &CxxString) -> Result<()>;
        #[namespace = "graphar_rs"]
        fn graph_info_dump(graph_info: &GraphInfo) -> Result<UniquePtr<CxxString>>;
    }

    // `Property`
    unsafe extern "C++" {
        #[namespace = "graphar"]
        type Property;

        #[namespace = "graphar_rs"]
        fn new_properties() -> UniquePtr<CxxVector<Property>>;
        #[namespace = "graphar_rs"]
        fn push_property(
            properties: Pin<&mut CxxVector<Property>>,
            name: &CxxString,
            type_: &SharedPtr<DataType>,
            is_primary: bool,
            is_nullable: bool,
            cardinality: Cardinality,
        );
    }

    // `PropertyGroup`
    #[namespace = "graphar"]
    unsafe extern "C++" {

        type PropertyGroup;

        fn GetProperties(&self) -> &CxxVector<Property>;
        fn HasProperty(&self, property_name: &CxxString) -> bool;

        fn CreatePropertyGroup(
            properties: &CxxVector<Property>,
            file_type: FileType,
            prefix: &CxxString,
        ) -> SharedPtr<PropertyGroup>;
    }

    // `PropertyGroupVector`
    unsafe extern "C++" {
        // `std::vector<std::shared_ptr<PropertyGroup>>`
        #[namespace = "graphar"]
        type PropertyGroupVector;

        #[namespace = "graphar_rs"]
        fn new_property_group_vec() -> UniquePtr<PropertyGroupVector>;
        #[namespace = "graphar_rs"]
        fn push_property_group(
            vec: Pin<&mut PropertyGroupVector>,
            property_group: SharedPtr<PropertyGroup>,
        );
    }

    // `VertexInfo`
    #[namespace = "graphar"]
    unsafe extern "C++" {
        type VertexInfo;

        fn PropertyGroupNum(&self) -> i32;
        fn GetPropertyGroupByIndex(&self, index: i32) -> SharedPtr<PropertyGroup>;

        fn CreateVertexInfo(
            type_: &CxxString,
            chunk_size: i64,
            property_groups: &PropertyGroupVector,
            labels: &CxxVector<CxxString>,
            prefix: &CxxString,
            version: SharedPtr<ConstInfoVersion>,
        ) -> SharedPtr<VertexInfo>;

        #[namespace = "graphar_rs"]
        fn vertex_info_save(vertex_info: &VertexInfo, path: &CxxString) -> Result<()>;
        #[namespace = "graphar_rs"]
        fn vertex_info_dump(vertex_info: &VertexInfo) -> Result<UniquePtr<CxxString>>;

        #[namespace = "graphar_rs"]
        fn create_vertex_info(
            type_: &String,
            chunk_size: i64,
            property_groups: &PropertyGroupVector,
            labels: &Vec<String>,
            prefix: &String,
            version: SharedPtr<ConstInfoVersion>,
        ) -> SharedPtr<VertexInfo>;
    }

    // `Vertex`
    #[namespace = "graphar::builder"]
    unsafe extern "C++" {
        type Vertex;

        #[namespace = "graphar_rs"]
        fn new_vertex() -> UniquePtr<Vertex>;
        #[namespace = "graphar_rs"]
        fn vertex_add_property_bool(vertex: Pin<&mut Vertex>, name: &CxxString, val: bool);
        #[namespace = "graphar_rs"]
        fn vertex_add_property_i32(vertex: Pin<&mut Vertex>, name: &CxxString, val: i32);
        #[namespace = "graphar_rs"]
        fn vertex_add_property_i64(vertex: Pin<&mut Vertex>, name: &CxxString, val: i64);
        #[namespace = "graphar_rs"]
        fn vertex_add_property_f32(vertex: Pin<&mut Vertex>, name: &CxxString, val: f32);
        #[namespace = "graphar_rs"]
        fn vertex_add_property_f64(vertex: Pin<&mut Vertex>, name: &CxxString, val: f64);
        #[namespace = "graphar_rs"]
        fn vertex_add_property_string(vertex: Pin<&mut Vertex>, name: &CxxString, val: &CxxString);
        // TODO(list date timestamp)
    }

    // `VerticesBuilder`
    unsafe extern "C++" {
        #[namespace = "graphar::builder"]
        type VerticesBuilder;

        #[namespace = "graphar_rs"]
        fn add_vertex(builder: Pin<&mut VerticesBuilder>, v: Pin<&mut Vertex>) -> Result<()>;
        #[namespace = "graphar_rs"]
        fn new_vertices_builder(
            vertex_info: &SharedPtr<VertexInfo>,
            path_prefix: &CxxString,
            start_idx: i64,
        ) -> Result<SharedPtr<VerticesBuilder>>;
        #[namespace = "graphar_rs"]
        fn vertices_dump(builder: Pin<&mut VerticesBuilder>) -> Result<()>;
    }

    // `AdjacentList`
    #[namespace = "graphar"]
    unsafe extern "C++" {
        type AdjacentList;

        fn GetType(&self) -> AdjListType;
        fn GetFileType(&self) -> FileType;
        fn GetPrefix(&self) -> &CxxString;

        fn CreateAdjacentList(
            type_: AdjListType,
            file_type: FileType,
            path_prefix: &CxxString,
        ) -> SharedPtr<AdjacentList>;
    }

    // `AdjacentListVector`
    unsafe extern "C++" {
        #[namespace = "graphar"]
        type AdjacentListVector;

        #[namespace = "graphar_rs"]
        fn new_adjacent_list_vec() -> UniquePtr<AdjacentListVector>;
        #[namespace = "graphar_rs"]
        fn push_adjacent_list(vec: Pin<&mut AdjacentListVector>, adj_list: SharedPtr<AdjacentList>);
    }

    // `EdgeInfo`
    #[namespace = "graphar"]
    unsafe extern "C++" {
        type EdgeInfo;

        fn GetSrcType(&self) -> &CxxString;
        fn GetDstType(&self) -> &CxxString;
        fn GetEdgeType(&self) -> &CxxString;
        fn GetChunkSize(&self) -> i64;
        fn GetSrcChunkSize(&self) -> i64;
        fn GetDstChunkSize(&self) -> i64;
        fn GetPrefix(&self) -> &CxxString;
        fn IsDirected(&self) -> bool;

        #[allow(clippy::too_many_arguments)]
        fn CreateEdgeInfo(
            src_type: &CxxString,
            edge_type: &CxxString,
            dst_type: &CxxString,
            chunk_size: i64,
            src_chunk_size: i64,
            dst_chunk_size: i64,
            directed: bool,
            adjacent_lists: &AdjacentListVector,
            property_groups: &PropertyGroupVector,
            path_prefix: &CxxString,
            version: SharedPtr<ConstInfoVersion>,
        ) -> SharedPtr<EdgeInfo>;

        #[namespace = "graphar_rs"]
        fn edge_info_save(edge_info: &EdgeInfo, path: &CxxString) -> Result<()>;
        #[namespace = "graphar_rs"]
        fn edge_info_dump(edge_info: &EdgeInfo) -> Result<UniquePtr<CxxString>>;
    }

    // `Edge`
    #[namespace = "graphar::builder"]
    unsafe extern "C++" {
        type Edge;

        #[namespace = "graphar_rs"]
        fn new_edge(src_id: i64, dst_id: i64) -> UniquePtr<Edge>;
        #[namespace = "graphar_rs"]
        fn edge_add_property_bool(edge: Pin<&mut Edge>, name: &CxxString, val: bool);
        #[namespace = "graphar_rs"]
        fn edge_add_property_i32(edge: Pin<&mut Edge>, name: &CxxString, val: i32);
        #[namespace = "graphar_rs"]
        fn edge_add_property_i64(edge: Pin<&mut Edge>, name: &CxxString, val: i64);
        #[namespace = "graphar_rs"]
        fn edge_add_property_f32(edge: Pin<&mut Edge>, name: &CxxString, val: f32);
        #[namespace = "graphar_rs"]
        fn edge_add_property_f64(edge: Pin<&mut Edge>, name: &CxxString, val: f64);
        #[namespace = "graphar_rs"]
        fn edge_add_property_string(edge: Pin<&mut Edge>, name: &CxxString, val: &CxxString);
        // TODO(list date timestamp)
    }

    // `EdgesBuilder`
    unsafe extern "C++" {
        #[namespace = "graphar::builder"]
        type EdgesBuilder;

        #[namespace = "graphar_rs"]
        fn add_edge(builder: Pin<&mut EdgesBuilder>, e: Pin<&mut Edge>) -> Result<()>;
        #[namespace = "graphar_rs"]
        fn new_edges_builder(
            edge_info: &SharedPtr<EdgeInfo>,
            path_prefix: &CxxString,
            adj_list_type: AdjListType,
            vertices_num: i64,
        ) -> Result<SharedPtr<EdgesBuilder>>;
        #[namespace = "graphar_rs"]
        fn edges_dump(builder: Pin<&mut EdgesBuilder>) -> Result<()>;
    }
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use cxx::{CxxVector, SharedPtr, UniquePtr, let_cxx_string};

    use crate::ffi::ffi::{
        AdjListType, AdjacentList, AdjacentListVector, Cardinality, CreateAdjacentList,
        CreateEdgeInfo, CreatePropertyGroup, CreateVertexInfo, Edge, EdgeInfo, FileType,
        PropertyGroup, PropertyGroupVector, Vertex, VertexInfo, add_edge, add_vertex, boolean,
        edge_add_property_bool, edge_add_property_f32, edge_add_property_f64,
        edge_add_property_i32, edge_add_property_i64, edge_add_property_string, edges_dump,
        float32, float64, int32, int64, load_graph_info, new_adjacent_list_vec,
        new_const_info_version, new_edge, new_edges_builder, new_properties,
        new_property_group_vec, new_vertex, new_vertices_builder, push_adjacent_list,
        push_property, push_property_group, string, vertex_add_property_bool,
        vertex_add_property_f32, vertex_add_property_f64, vertex_add_property_i32,
        vertex_add_property_i64, vertex_add_property_string, vertices_dump,
    };

    fn mock_property_group() -> SharedPtr<PropertyGroup> {
        let file_type = FileType::CSV;
        let mut properties = new_properties();
        {
            let_cxx_string!(name = "bool");
            push_property(
                properties.pin_mut(),
                &name,
                boolean(),
                false,
                false,
                Cardinality::SINGLE,
            );

            let_cxx_string!(name = "i32");
            push_property(
                properties.pin_mut(),
                &name,
                int32(),
                false,
                false,
                Cardinality::SINGLE,
            );

            let_cxx_string!(name = "i64");
            push_property(
                properties.pin_mut(),
                &name,
                int64(),
                false,
                false,
                Cardinality::SINGLE,
            );

            let_cxx_string!(name = "f32");
            push_property(
                properties.pin_mut(),
                &name,
                float32(),
                false,
                false,
                Cardinality::SINGLE,
            );

            let_cxx_string!(name = "f64");
            push_property(
                properties.pin_mut(),
                &name,
                float64(),
                false,
                false,
                Cardinality::SINGLE,
            );

            let_cxx_string!(name = "string");
            push_property(
                properties.pin_mut(),
                &name,
                string(),
                false,
                false,
                Cardinality::SINGLE,
            );
        }
        let_cxx_string!(prefix = "");

        CreatePropertyGroup(&properties, file_type, &prefix)
    }

    fn mock_property_group_vector() -> UniquePtr<PropertyGroupVector> {
        let mut pgv = new_property_group_vec();
        push_property_group(pgv.pin_mut(), mock_property_group());
        pgv
    }

    fn mock_vertex_info() -> SharedPtr<VertexInfo> {
        let_cxx_string!(type_ = "test_vertex");
        let_cxx_string!(prefix = "test_vertex");

        let pgv = mock_property_group_vector();
        let chunk_size = 100;
        let labels = CxxVector::new();
        let version = new_const_info_version(1).unwrap();
        CreateVertexInfo(&type_, chunk_size, &pgv, &labels, &prefix, version)
    }

    fn mock_vertex() -> UniquePtr<Vertex> {
        let mut v = new_vertex();
        let_cxx_string!(bool = "bool");
        let_cxx_string!(int32 = "i32");
        let_cxx_string!(int64 = "i64");
        let_cxx_string!(float = "f32");
        let_cxx_string!(double = "f64");
        let_cxx_string!(string = "string");
        let_cxx_string!(string_value = "string_value");

        vertex_add_property_bool(v.pin_mut(), &bool, true);
        vertex_add_property_i32(v.pin_mut(), &int32, 114514);
        vertex_add_property_i64(v.pin_mut(), &int64, 1919810);
        vertex_add_property_f32(v.pin_mut(), &float, 1919810.114514);
        vertex_add_property_f64(v.pin_mut(), &double, 114514.1919810);
        vertex_add_property_string(v.pin_mut(), &string, &string_value);
        v
    }

    fn mock_adjacent_list() -> SharedPtr<AdjacentList> {
        let_cxx_string!(path_prefix = "test_adjacent_list");
        CreateAdjacentList(AdjListType::ordered_by_source, FileType::CSV, &path_prefix)
    }

    fn mock_adjacent_list_vector() -> UniquePtr<AdjacentListVector> {
        let mut vec = new_adjacent_list_vec();
        push_adjacent_list(vec.pin_mut(), mock_adjacent_list());
        vec
    }

    fn mock_edge() -> UniquePtr<Edge> {
        let mut e = new_edge(1, 2);
        let_cxx_string!(bool = "bool");
        let_cxx_string!(int32 = "i32");
        let_cxx_string!(int64 = "i64");
        let_cxx_string!(float = "f32");
        let_cxx_string!(double = "f64");
        let_cxx_string!(string = "string");
        let_cxx_string!(string_value = "string_value");

        edge_add_property_bool(e.pin_mut(), &bool, true);
        edge_add_property_i32(e.pin_mut(), &int32, 114514);
        edge_add_property_i64(e.pin_mut(), &int64, 1919810);
        edge_add_property_f32(e.pin_mut(), &float, 1919810.114514);
        edge_add_property_f64(e.pin_mut(), &double, 114514.1919810);
        edge_add_property_string(e.pin_mut(), &string, &string_value);

        e
    }

    fn mock_edge_info() -> SharedPtr<EdgeInfo> {
        let_cxx_string!(src_type = "src");
        let_cxx_string!(dst_type = "dst");
        let_cxx_string!(edge_type = "to");
        let_cxx_string!(prefix = "test_edge");

        let version = new_const_info_version(1).unwrap();
        CreateEdgeInfo(
            &src_type,
            &edge_type,
            &dst_type,
            100,
            100,
            100,
            true,
            &mock_adjacent_list_vector(),
            &mock_property_group_vector(),
            &prefix,
            version,
        )
    }

    #[test]
    fn test_load_graph_info() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("incubator-graphar-testing")
            .join("modern_graph")
            .join("modern_graph.graph.yml");
        let_cxx_string!(path = root.to_str().unwrap());
        let graph_info = load_graph_info(&path).unwrap();
        println!("graph_name = {}", graph_info.GetName().to_str().unwrap());

        println!("labels:");
        for label in graph_info.GetLabels().iter() {
            println!("\t{}", label.to_str().unwrap())
        }

        println!("vertex_info_num = {}", graph_info.VertexInfoNum());
        println!("edge_info_num = {}", graph_info.EdgeInfoNum());
    }

    #[test]
    fn test_properties_group() {
        let pg = mock_property_group();

        assert_eq!(pg.GetProperties().len(), 6);
        let_cxx_string!(name = "bool");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "i32");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "i64");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "f32");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "f64");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "string");
        assert!(pg.HasProperty(&name));
    }

    #[test]
    fn test_vertex_info() {
        let vertex_info = mock_vertex_info();

        assert_eq!(vertex_info.PropertyGroupNum(), 1);

        let pg = vertex_info.GetPropertyGroupByIndex(0);
        assert_eq!(pg.GetProperties().len(), 6);

        let_cxx_string!(name = "bool");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "i32");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "i64");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "f32");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "f64");
        assert!(pg.HasProperty(&name));

        let_cxx_string!(name = "string");
        assert!(pg.HasProperty(&name));
    }

    #[test]
    fn test_vertex() {
        let mut v = new_vertex();
        let_cxx_string!(bool = "bool");
        let_cxx_string!(int32 = "i32");
        let_cxx_string!(int64 = "i64");
        let_cxx_string!(float = "f32");
        let_cxx_string!(double = "f64");
        let_cxx_string!(string = "string");
        let_cxx_string!(string_value = "string_value");

        vertex_add_property_bool(v.pin_mut(), &bool, true);
        vertex_add_property_i32(v.pin_mut(), &int32, 114514);
        vertex_add_property_i64(v.pin_mut(), &int64, 1919810);
        vertex_add_property_f32(v.pin_mut(), &float, 1919810.114514);
        vertex_add_property_f64(v.pin_mut(), &double, 114514.1919810);
        vertex_add_property_string(v.pin_mut(), &string, &string_value);
    }

    #[test]
    fn test_vertices_builder() {
        let vertex_info = mock_vertex_info();
        let_cxx_string!(prefix = "/tmp/");

        let mut builder = new_vertices_builder(&vertex_info, &prefix, 0).unwrap();
        let mut v = mock_vertex();
        add_vertex(unsafe { builder.pin_mut_unchecked() }, v.pin_mut()).unwrap();
        vertices_dump(unsafe { builder.pin_mut_unchecked() }).unwrap();
    }

    #[test]
    fn test_adj_list() {
        let adj_list = mock_adjacent_list();
        assert_eq!(adj_list.GetType(), AdjListType::ordered_by_source);
        assert_eq!(adj_list.GetFileType(), FileType::CSV);
        let_cxx_string!(prefix = "test_adjacent_list");
        assert_eq!(adj_list.GetPrefix().as_bytes(), prefix.as_bytes());
    }

    #[test]
    fn test_edge_info() {
        let edge_info = mock_edge_info();

        assert_eq!(edge_info.GetChunkSize(), 100);
        assert_eq!(edge_info.GetSrcChunkSize(), 100);
        assert_eq!(edge_info.GetDstChunkSize(), 100);
        assert_eq!(edge_info.GetSrcType(), "src");
        assert_eq!(edge_info.GetEdgeType(), "to");
        assert_eq!(edge_info.GetDstType(), "dst");
        assert!(edge_info.IsDirected());
        assert_eq!(edge_info.GetPrefix(), "test_edge");
    }

    #[test]
    fn test_edges_builder() {
        let edge_info = mock_edge_info();

        let_cxx_string!(prefix = "/tmp/");
        let mut builder =
            new_edges_builder(&edge_info, &prefix, AdjListType::ordered_by_source, 2).unwrap();
        let mut e = mock_edge();
        add_edge(unsafe { builder.pin_mut_unchecked() }, e.pin_mut()).unwrap();
        edges_dump(unsafe { builder.pin_mut_unchecked() }).unwrap();
    }
}
