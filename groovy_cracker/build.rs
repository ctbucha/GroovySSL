use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../groovy_cracker_cuda")
        .copy_to("./resources/groovy_cracker_cuda.ptx")
        .build()
        .unwrap();
}
