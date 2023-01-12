use tonic_build::configure;

fn main() {
    configure()
        .compile(
            &[
                "protos/auth.proto",
                "protos/packet.proto",
                "protos/shared.proto",
                "protos/shredstream.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
