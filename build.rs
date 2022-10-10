// use protobuf_codegen::Codegen;
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     Codegen::new()
//         .protoc()
//         .cargo_out_dir("generated_with_native")
//         .input("src/protos/hello_world.proto")
//         .include("src/protos")
//         .run_from_script();
//
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/protos/hello_world.proto")?;
    tonic_build::compile_protos("src/protos/tag_service.proto")?;
    tonic_build::compile_protos("src/protos/tag_search_service.proto")?;

    Ok(())
}