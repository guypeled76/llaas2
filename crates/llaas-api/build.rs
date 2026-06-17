fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/content.proto"], &["proto"])
        .unwrap();
}
