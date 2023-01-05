fn main() {
    winres::WindowsResource::new().compile().unwrap();
    static_vcruntime::metabuild();
}
