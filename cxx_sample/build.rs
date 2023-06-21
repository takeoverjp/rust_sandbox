use cmake;

fn main() {
  let dst = cmake::build("src");
  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=static=wrapper");
  println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
  println!("cargo:rustc-link-lib=dylib=stdc++");
}
