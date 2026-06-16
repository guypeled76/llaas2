fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["src/messages/content.proto"], &["src/messages"])
        .unwrap();
}
