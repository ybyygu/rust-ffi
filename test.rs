// test.rs
// :PROPERTIES:
// :header-args: :tangle test.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*test.rs][test.rs:1]]
use std::os::raw::c_int;

extern "C" {
    fn run() -> c_int;
}

fn main() {
    unsafe {
        run();
    }
}
// test.rs:1 ends here
