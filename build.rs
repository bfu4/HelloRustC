extern crate cc;

fn main() {
   println!("cargo:rustc-link-search=native=./");
   println!("cargo:rustc-link-lib-static=hello");
   // cc::Build::new().file("hello.c").compile("Hello");
}
