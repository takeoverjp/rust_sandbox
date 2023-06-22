use cc;

fn main() {
  cc::Build::new().file("src/wrapper.cc").compile("wrapper");
  println!("cargo:rustc-link-lib=static=wrapper");
  println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
  println!("cargo:rustc-link-lib=dylib=stdc++");
}
