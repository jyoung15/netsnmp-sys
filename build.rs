fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=dylib=netsnmp");
    println!("cargo:rustc-link-lib=dylib=crypto");
}
