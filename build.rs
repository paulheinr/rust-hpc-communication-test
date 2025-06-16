fn main() {
    // prost_build::Config::new()
    //     .compile_protos(&["src/proto/events.proto"], &["src/proto", "src/proto/google/protobuf"])
    //     .unwrap();
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["src/proto/events.proto"], &["src/proto", "src/proto/google/protobuf"])
        .unwrap();
}