// test.rs
// :PROPERTIES:
// :header-args: :tangle src/test.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*test.rs][test.rs:1]]
use std::os::raw::c_int;

extern "C" {
    fn run(x_array: *mut f64, n: c_int) -> c_int;
}

const N: usize = 100;
fn main() {
    let mut x = [0.0; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    let ret = unsafe {
        run(x.as_mut_ptr(), 100)
    };

    println!("L-BFGS optimization terminated with status code = {}", ret);
}
// test.rs:1 ends here
