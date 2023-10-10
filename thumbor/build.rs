fn main() {
    // 指定当某些文件发生变化时才重新编译
    println!("cargo:rerun-if-change=abi.proto");
    println!("cargo:rerun-if-change=build.rs");

    prost_build::Config::new()
        .out_dir("./src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}