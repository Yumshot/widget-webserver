fn main() {
    // Instruct cargo to set the correct linker argument for a GUI application
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:windows");
    }
}
