fn main() {
  prost_build::compile_protos(
    &["proto/telemetry.proto"],
    &["proto"],
  ).expect("Error compiling protobufs");

  tauri_build::build();
}
