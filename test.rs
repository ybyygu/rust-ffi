// base

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*base][base:1]]
use std::os::raw::c_int;

pub type lbfgsfloatval_t = f64;

extern "C" {
    fn run(n: c_int, x_array: *mut f64, param: *mut LBFGSParameter) -> c_int;
}

#[repr(C)]
#[derive(Debug)]
pub struct LBFGSParameter {
    pub m: ::std::os::raw::c_int,
    pub epsilon: lbfgsfloatval_t,
    pub past: ::std::os::raw::c_int,
    pub delta: lbfgsfloatval_t,
    pub max_iterations: ::std::os::raw::c_int,
    pub linesearch: ::std::os::raw::c_int,
    pub max_linesearch: ::std::os::raw::c_int,
    pub min_step: lbfgsfloatval_t,
    pub max_step: lbfgsfloatval_t,
    pub ftol: lbfgsfloatval_t,
    pub wolfe: lbfgsfloatval_t,
    pub gtol: lbfgsfloatval_t,
    pub xtol: lbfgsfloatval_t,
    pub orthantwise_c: lbfgsfloatval_t,
    pub orthantwise_start: ::std::os::raw::c_int,
    pub orthantwise_end: ::std::os::raw::c_int,
}

impl Default for LBFGSParameter {
    fn default() -> Self {
        LBFGSParameter {
            // The number of corrections to approximate the inverse hessian matrix.
            m: 6,
            // Epsilon for convergence test.
            epsilon: 1e-5,
            // Distance for delta-based convergence test.
            past: 0,
            // The maximum number of iterations.
            delta: 1e-5,
            // The maximum number of iterations.
            max_iterations: 0,
            // The line search algorithm.
            linesearch: 0,
            // The maximum number of trials for the line search.
            max_linesearch: 40,
            // The minimum step of the line search routine.
            min_step: 1e-20,
            // The maximum step of the line search.
            max_step: 1e20,
            // A parameter to control the accuracy of the line search routine.
            ftol: 1e-4,
            // A coefficient for the Wolfe condition.
            wolfe: 0.9,
            // A parameter to control the accuracy of the line search routine.
            gtol: 0.9,
            // The machine precision for floating-point values.
            xtol: 1.0e-16,
            // Coeefficient for the L1 norm of variables.
            orthantwise_c: 0.0,
            // Start index for computing L1 norm of the variables.
            orthantwise_start: 0,
            // End index for computing L1 norm of the variables.
            orthantwise_end: -1,
        }
    }
}

const N: usize = 100;
// base:1 ends here

// main

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*main][main:1]]
fn main() {
    let mut x = [0.0; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    let mut param = LBFGSParameter::default();
    let ret = unsafe {
        run(100, x.as_mut_ptr(), &mut param)
    };

    // println!("  fx = {}, x[0] = {}, x[1] = {}\n", fx, x[0], x[1]);
    println!(" rust ret = {}, x[0] = {}, x[1] = {}\n", ret, x[0], x[1]);
}
// main:1 ends here
