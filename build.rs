fn main() {
    prost_build::compile_protos(
        &[
            "src/http.proto",
            "src/core.proto",
            "src/messaging.proto",
            "src/kv.proto",
            "src/blob.proto",
            "src/extras.proto",
        ],
        &["src/"],
    )
    .unwrap();
}
