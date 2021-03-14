
use libc::c_char;
use std::ffi::CStr;


/**
 * During compile time, rustc complains about
 * the type String not being FFI-safe. However,
 * when using &str or &'static str, it complains as well.
 * What does this mean?
 *
 * FFI is Rust's (F)oreign (F)unction (I)nterface. Rust has
 * C bindings, life is great!
 *
 * On a sidenote, this code compiles to a mach-O (FEEDFACF), which is quite nice!
 *
 * For str vs String, we can look at https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
 * (Credits to @fuwn's profile on Github)
 *
 */

#[link (name = "hello")]
extern "C" {
   fn getMessage() -> * const c_char;
}

fn main() {
   let cbuf : * const c_char = unsafe { getMessage() };
   let cstr : & CStr = unsafe { CStr::from_ptr(cbuf) };
   println!("{}", cstr.to_str().unwrap() );
}
