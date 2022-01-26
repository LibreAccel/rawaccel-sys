fn main() {
  cxx_build::bridge("src/lib.rs")
    .include("/rawaccel")
    .flag("/std:c++17")
    .compile("rawaccel");

  println!("cargo:rerun-if-changed=src/lib.rs");
  println!("cargo:rerun-if-changed=rawaccel");
}
