use std::{env::current_dir, fs::exists, path::Path};

use graphar::{
    graph_info::{GraphInfo, Type},
    graph_reader::Vertices,
};

const PATH: &str = "/workspace/incubator-graphar-testing/modern_graph";

fn read() {
    let path = Path::new(PATH).join("modern_graph.graph.yml");
    assert!(exists(&path).unwrap());
    let graph_info = GraphInfo::load(path).unwrap();
    let vertex_infos = graph_info.vertex_infos();

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
            print!("{}", iter.id());
            for p in props.iter() {
                let ty = p.data_type();
                let name = p.name();
                let value = match ty.id() {
                    Type::Bool => iter.property::<bool>(&name).unwrap().to_string(),
                    Type::Int32 => iter.property::<i32>(&name).unwrap().to_string(),
                    Type::Int64 => iter.property::<i64>(&name).unwrap().to_string(),
                    Type::Float => iter.property::<f32>(&name).unwrap().to_string(),
                    Type::Double => iter.property::<f64>(&name).unwrap().to_string(),
                    _ => unimplemented!(),
                };
                print!("{}({}): {},", name, value, ty);
            }
            println!();
            count += 1;
            iter.next();
        }

        assert_eq!(len, count);
    }
}

fn main() {
    println!("{}", current_dir().unwrap().display());
    read();
}
