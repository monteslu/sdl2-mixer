extern crate napi_build;

fn main() {
  napi_build::setup();

  // For macOS, use the Homebrew paths
  if cfg!(target_os = "macos") {
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_mixer");
  } else if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-search=C:/SDL2/lib");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_mixer");
  } else {
    // Linux paths
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_mixer");
  }
}
