// No imports needed for PathBuf at the top if using std::path::PathBuf

fn main() {
    let project_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = project_dir.join("target").join(std::env::var("PROFILE").unwrap());
    
    // 1. Link to the local vosk library
    println!("cargo:rustc-link-search=native={}/libs", project_dir.display());
    println!("cargo:rustc-link-lib=dylib=vosk");
    
    // 2. Ensure the dylib is in the same folder as the binary for development
    // This is the most reliable way for macOS to find it at runtime
    let libs_dir = project_dir.join("libs");
    let dylib_name = "libvosk.dylib";
    let source = libs_dir.join(dylib_name);
    
    // We try to copy it to the target dir (best effort)
    if source.exists() {
        // Create target dir if it doesn't exist (e.g. initial cargo check)
        std::fs::create_dir_all(&target_dir).ok();
        let destination = target_dir.join(dylib_name);
        std::fs::copy(&source, &destination).ok();
        
        // Also copy to deps as cargo check/test often look there
        let deps_dir = target_dir.join("deps");
        std::fs::create_dir_all(&deps_dir).ok();
        std::fs::copy(&source, deps_dir.join(dylib_name)).ok();
    }
    
    // 3. Set rpath for portability in the final bundle
    // @executable_path is the directory containing the executable
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/libs");
    
    tauri_build::build();
}
