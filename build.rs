fn main() {
    tonic_build::configure()
        .out_dir("src/routeguide")
        .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .out_dir("src/helloworld")
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();
}
