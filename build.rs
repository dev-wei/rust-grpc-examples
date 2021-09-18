fn main() {
    tonic_build::configure()
        .out_dir("src/helloworld")
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();
}
