fn emscripten_sdl_flags() {
  println!("cargo:rustc-link-arg=-sWASM");
  println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH");
  println!("cargo:rustc-link-arg=-oindex.html");
  println!("cargo:rustc-link-arg=-sUSE_SDL=2");
  println!("cargo:rustc-link-arg=--preload-file=./assets");
  println!("cargo:rustc-link-arg=-sUSE_SDL_IMAGE=2");
  println!("cargo:rustc-link-arg=-sASYNCIFY");
  println!("cargo:rustc-link-arg=-sSDL2_IMAGE_FORMATS=[\"png\"]");
}

fn main() {
  if let Ok(target) = std::env::var("TARGET") {
    if target == "wasm32-unknown-emscripten" {
      emscripten_sdl_flags();
    }
  }
}
