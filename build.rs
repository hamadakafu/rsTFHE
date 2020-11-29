fn main() {
    let src = [
        "src/cpp/spqlios-fft.s",
        "src/cpp/spqlios-ifft.s",
        "src/cpp/spqlios-fft-impl.cpp",
    ];
    let include = "src/cpp";
    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .files(src.iter())
        .include(include)
        .flag("-Wno-unused-parameter")
        .flag("-v")
        .compile("libspqlios.a");
}
