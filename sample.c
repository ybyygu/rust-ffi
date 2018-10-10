/* sample.c */
/* :PROPERTIES: */
/* :header-args: :tangle src/sample.c */
/* :END: */

/* [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*sample.c][sample.c:1]] */
#include <stdio.h>
#include <lbfgs.h>

/* static lbfgsfloatval_t evaluate( */
/*     void *instance, */
/*     const lbfgsfloatval_t *x, */
/*     lbfgsfloatval_t *g, */
/*     const int n, */
/*     const lbfgsfloatval_t step */
/*     ) */
/* { */
/*   int i; */
/*   lbfgsfloatval_t fx = 0.0; */

/*   for (i = 0;i < n;i += 2) { */
/*     lbfgsfloatval_t t1 = 1.0 - x[i]; */
/*     lbfgsfloatval_t t2 = 10.0 * (x[i+1] - x[i] * x[i]); */
/*     g[i+1] = 20.0 * t2; */
/*     g[i] = -2.0 * (x[i] * g[i+1] + t1); */
/*     fx += t1 * t1 + t2 * t2; */
/*   } */
/*   return fx; */
/* } */

int run(int n,
        lbfgsfloatval_t *x,
        lbfgsfloatval_t *fx,
        lbfgs_evaluate_t evaluate,
        lbfgs_progress_t progress,
        void *instance,
        lbfgs_parameter_t *param)
{
  if (x == NULL) {
    printf("ERROR: Null pointer of x.\n");
    return 1;
  }

  if (fx == NULL) {
    printf("ERROR: Null pointer of fx.\n");
    return 1;
  }

  /*
    Start the L-BFGS optimization; this will invoke the callback functions
    evaluate() and progress() when necessary.
  */
  int ret = lbfgs(n,
                  x,
                  fx,
                  evaluate,
                  progress,
                  instance,
                  param);

  return ret;
}
/* sample.c:1 ends here */
