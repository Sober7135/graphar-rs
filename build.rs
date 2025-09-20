use std::path::{Path, PathBuf};

fn link_libraries() {
    // build from source should enable this
    // println!("cargo:rustc-link-lib=static=graphar_bundled_dependencies");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=dylib=arrow_compute");
    println!("cargo:rustc-link-lib=dylib=arrow");

    println!("cargo:rustc-link-lib=graphar");
}

fn build_ffi(bridge_file: &str, out_name: &str, source_file: &str, include_paths: &Vec<PathBuf>) {
    let mut build = cxx_build::bridge(bridge_file);
    build.file(source_file);

    build.includes(include_paths);
    build.flag("-std=c++17");
    build.flag("-fdiagnostics-color=always");
    // We should define ARROW_ORC, because we built arrow from source,
    // in this case, CMakeLists.txt will add this definition, so we need to add manually
    build.define("ARROW_ORC", None);

    build.compile(out_name);
}

fn build_bundled_cmake() -> Vec<PathBuf> {
    let graph_ar_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("incubator-graphar")
        .join("cpp");
    let mut build = cmake::Config::new(&graph_ar_root);
    build
        .no_build_target(true)
        // .define("BUILD_ARROW_FROM_SOURCE", "on")
        .define("CMAKE_BUILD_TYPE", "Release");
    let build_dir = build.build();

    let graphar_lib_path = build_dir.join("build");
    println!(
        "cargo:rustc-link-search=native={}",
        graphar_lib_path.display()
    );

    println!("cargo:rerun-if-changed=include/graphar_rs.h");
    println!("cargo:rerun-if-changed=src/graphar_rs.cc");
    println!("cargo:rerun-if-changed=incubator-graphar");

    vec![
        graph_ar_root.join("src/"),
        graph_ar_root.join("thirdparty/"),
        graphar_lib_path.join("arrow_ep-prefix/include"),
    ]
}

fn main() {
    let mut include_paths = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("include")];
    include_paths.extend(build_bundled_cmake());

    link_libraries();

    build_ffi(
        "src/ffi.rs",
        "graphar_cxx",
        "src/graphar_rs.cc",
        &include_paths,
    );
}
