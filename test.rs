// base

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*base][base:1]]
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};

pub type lbfgsfloatval_t = f64;

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

// progress

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*progress][progress:1]]
/// Callback interface to receive the progress of the optimization process.
///
///  The lbfgs() function call this function for each iteration. Implementing
///  this function, a client program can store or display the current progress
///  of the optimization process.
///
///  @param  instance    The user data sent for lbfgs() function by the client.
///  @param  x           The current values of variables.
///  @param  g           The current gradient values of variables.
///  @param  fx          The current value of the objective function.
///  @param  xnorm       The Euclidean norm of the variables.
///  @param  gnorm       The Euclidean norm of the gradients.
///  @param  step        The line-search step used for this iteration.
///  @param  n           The number of variables.
///  @param  k           The iteration count.
///  @param  ls          The number of evaluations called for this iteration.
///  @retval int         Zero to continue the optimization process. Returning a
///                      non-zero value will cancel the optimization process.
pub type CbProgress = ::std::option::Option<
    unsafe extern "C" fn(
        instance: *mut ::std::os::raw::c_void,
        x: *const lbfgsfloatval_t,
        g: *const lbfgsfloatval_t,
        fx: lbfgsfloatval_t,
        xnorm: lbfgsfloatval_t,
        gnorm: lbfgsfloatval_t,
        step: lbfgsfloatval_t,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        ls: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;

#[no_mangle]
pub unsafe extern "C" fn progress(
        instance : *mut c_void,
        x        : *const lbfgsfloatval_t,
        g        : *const lbfgsfloatval_t,
        fx       : lbfgsfloatval_t,
        xnorm    : lbfgsfloatval_t,
        gnorm    : lbfgsfloatval_t,
        step     : lbfgsfloatval_t,
        n        : c_int,
        k        : c_int,
        ls       : c_int) -> c_int
{
    assert!(!x.is_null(), "Null pointer in progress()");
    assert!(!g.is_null(), "Null pointer in progress()");

    let n = n as usize;
    // convert pointer to native data type
    let x = ::std::slice::from_raw_parts(x, n);

    // convert pointer to native data type
    let g = ::std::slice::from_raw_parts(g, n);

    println!("Iteration {}:", k);
    println!("  fx = {}, x[0] = {}, x[1] = {}", fx, x[0], x[1]);
    println!("  xnorm = {}, gnorm = {}, step = {}", xnorm, gnorm, step);
    println!("");
    if gnorm < 7. {
        return 0
    } else {
        return 0
    }
}
// progress:1 ends here

// evalulate

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*evalulate][evalulate:1]]
/// Callback interface to provide objective function and gradient evaluations.
///
///  The lbfgs() function call this function to obtain the values of objective
///  function and its gradients when needed. A client program must implement
///  this function to evaluate the values of the objective function and its
///  gradients, given current values of variables.
///
///  @param  instance    The user data sent for lbfgs() function by the client.
///  @param  x           The current values of variables.
///  @param  g           The gradient vector. The callback function must compute
///                      the gradient values for the current variables.
///  @param  n           The number of variables.
///  @param  step        The current step of the line search routine.
///  @retval lbfgsfloatval_t The value of the objective function for the current
///                          variables.
pub type CbEvaluate = ::std::option::Option<
    unsafe extern "C" fn(
        instance: *mut ::std::os::raw::c_void,
        x: *const lbfgsfloatval_t,
        g: *mut lbfgsfloatval_t,
        n: ::std::os::raw::c_int,
        step: lbfgsfloatval_t,
    ) -> lbfgsfloatval_t,
>;

#[no_mangle]
pub unsafe extern "C" fn evaluate(instance: *mut c_void,
                                  x: *const lbfgsfloatval_t,
                                  g: *mut lbfgsfloatval_t,
                                  n: c_int,
                                  step: lbfgsfloatval_t) -> lbfgsfloatval_t
{
    assert!(!x.is_null(), "Null pointer in evaluate()");
    assert!(!g.is_null(), "Null pointer in evaluate()");

    let n = n as usize;
    // convert pointer to native data type
    let x = ::std::slice::from_raw_parts(x, n);

    // convert pointer to native data type
    let mut g = ::std::slice::from_raw_parts_mut(g, n);

    let mut fx: lbfgsfloatval_t = 0.0;
    for i in (0..n).step_by(2) {
        let t1 = 1.0 - x[i];
        let t2 = 10.0 * (x[i+1] - x[i] * x[i]);
        g[i+1] = 20.0 * t2;
        g[i] = -2.0 * (x[i] * g[i+1] + t1);
        fx += t1 * t1 + t2 * t2;
    }

    // ::std::mem::forget(g);

    fx
}
// evalulate:1 ends here

// main

// [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*main][main:1]]
extern "C" {
    fn lbfgs(n: c_int,
           x_array: *mut lbfgsfloatval_t,
           ptr_fx: *mut lbfgsfloatval_t,
           proc_evaluate: CbEvaluate,
           proc_progress: CbProgress,
           instance: *mut c_void,
           param: *mut LBFGSParameter) -> c_int;
}

fn main() {
    let mut x = [0.0; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    let mut fx: lbfgsfloatval_t = 0.0;
    let mut param = LBFGSParameter::default();
    let ret = unsafe {
        lbfgs(N as i32,
            x.as_mut_ptr(),
            &mut fx,
            Some(evaluate),
            Some(progress),
            null_mut(),
            &mut param,
        )
    };

    // Report the result.
    println!("rust  fx = {}, x[0] = {}, x[1] = {}\n", fx, x[0], x[1]);
    println!("L-BFGS optimization terminated with status code = {:?}", ret);
}
// main:1 ends here
