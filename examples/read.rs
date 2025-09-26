use std::{fs::exists, path::Path};

use graphar::{
    graph_info::{AdjListType, EdgeInfo, GraphInfo, Property, Type},
    graph_reader::{Edges, Vertices},
};

const PATH: &str = "incubator-graphar-testing/modern_graph";

fn read() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = root.join(PATH).join("modern_graph.graph.yml");

    assert!(exists(&path).unwrap());

    let graph_info = GraphInfo::load(path).unwrap();
    let vertex_infos = graph_info.vertex_infos();
    let edge_infos = graph_info.edge_infos();

    for info in &vertex_infos {
        let pgs = info.property_groups();
        let props = pgs
            .iter()
            .flat_map(|pg| pg.properties())
            .collect::<Vec<_>>();
        let ty = info.ty();
        let mut vertices = Vertices::new(&graph_info, &ty).unwrap();
        let mut iter = vertices.begin();
        let len = vertices.len();
        let mut count = 0_usize;

        while iter != vertices.end() {
            print!("{} ", iter.id());
            for p in props.iter() {
                let ty = p.data_type();
                let name = p.name();
                let value = match ty.id() {
                    Type::Bool => iter.property::<bool>(&name).unwrap().to_string(),
                    Type::Int32 => iter.property::<i32>(&name).unwrap().to_string(),
                    Type::Int64 => iter.property::<i64>(&name).unwrap().to_string(),
                    Type::Float => iter.property::<f32>(&name).unwrap().to_string(),
                    Type::Double => iter.property::<f64>(&name).unwrap().to_string(),
                    Type::String => iter.property::<String>(&name).unwrap(),
                    _ => unimplemented!(),
                };
                print!("{}({}): {}, ", name, value, ty);
            }
            println!();
            count += 1;
            iter.next();
        }
        println!("");

        assert_eq!(len, count);
    }

    let iter =
        |graph_info: &GraphInfo, info: &EdgeInfo, props: &Vec<Property>, adj_type: AdjListType| {
            let mut edges = Edges::new(
                &graph_info,
                &info.src_type(),
                &info.edge_type(),
                &info.dst_type(),
                adj_type,
                None,
            )
            .unwrap();

            let len = edges.len();
            let mut iter = edges.begin();
            let mut count = 0_usize;

            while iter != edges.end() {
                print!("{} -> {}: ", iter.source(), iter.destination());
                for p in props.iter() {
                    let ty = p.data_type();
                    let name = p.name();
                    let value = match ty.id() {
                        Type::Bool => iter.property::<bool>(&name).unwrap().to_string(),
                        Type::Int32 => iter.property::<i32>(&name).unwrap().to_string(),
                        Type::Int64 => iter.property::<i64>(&name).unwrap().to_string(),
                        Type::Float => iter.property::<f32>(&name).unwrap().to_string(),
                        Type::Double => iter.property::<f64>(&name).unwrap().to_string(),
                        Type::String => iter.property::<String>(&name).unwrap(),
                        _ => unimplemented!(),
                    };
                    print!("{}({}): {}, ", name, value, ty);
                }
                println!();
                count += 1;
                iter.next();
            }
            println!();

            assert_eq!(len, count)
        };

    for info in &edge_infos {
        let pgs = info.property_groups();
        let props = pgs
            .iter()
            .flat_map(|pg| pg.properties())
            .collect::<Vec<_>>();

        if info.has_adjacent_list_type(AdjListType::OrderedBySource) {
            iter(&graph_info, &info, &props, AdjListType::OrderedBySource);
        }
        if info.has_adjacent_list_type(AdjListType::OrderedByDest) {
            iter(&graph_info, &info, &props, AdjListType::OrderedByDest);
        }
        if info.has_adjacent_list_type(AdjListType::UnorderedBySource) {
            iter(&graph_info, &info, &props, AdjListType::UnorderedBySource);
        }
        if info.has_adjacent_list_type(AdjListType::UnorderedByDest) {
            iter(&graph_info, &info, &props, AdjListType::UnorderedByDest);
        }

        // let edges =  Edges::new(&graph_info, info.src_type(), info.edge_type(), info.dst_type(), info, vertex_chunks)
    }
}

fn main() {
    read();
}
