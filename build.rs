fn main() {
    prost_build::Config::new()
        .compile_protos(&["src/proto/events.proto"], &["src/proto", "src/proto/google/protobuf"])
        .unwrap();
}