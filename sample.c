/* sample.c */
/* :PROPERTIES: */
/* :header-args: :tangle src/sample.c */
/* :END: */

/* [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*sample.c][sample.c:1]] */
#include <stdio.h>
#include <lbfgs.h>

static lbfgsfloatval_t evaluate(
    void *instance,
    const lbfgsfloatval_t *x,
    lbfgsfloatval_t *g,
    const int n,
    const lbfgsfloatval_t step
    )
{
  int i;
  lbfgsfloatval_t fx = 0.0;

  for (i = 0;i < n;i += 2) {
    lbfgsfloatval_t t1 = 1.0 - x[i];
    lbfgsfloatval_t t2 = 10.0 * (x[i+1] - x[i] * x[i]);
    g[i+1] = 20.0 * t2;
    g[i] = -2.0 * (x[i] * g[i+1] + t1);
    fx += t1 * t1 + t2 * t2;
  }
  return fx;
}

static int progress(
    void *instance,
    const lbfgsfloatval_t *x,
    const lbfgsfloatval_t *g,
    const lbfgsfloatval_t fx,
    const lbfgsfloatval_t xnorm,
    const lbfgsfloatval_t gnorm,
    const lbfgsfloatval_t step,
    int n,
    int k,
    int ls
    )
{
  printf("Iteration %d:\n", k);
  printf("  fx = %f, x[0] = %f, x[1] = %f\n", fx, x[0], x[1]);
  printf("  xnorm = %f, gnorm = %f, step = %f\n", xnorm, gnorm, step);
  printf("\n");
  return 0;
}

int run(int n, lbfgsfloatval_t *x, lbfgs_parameter_t *param)
{
  if (x == NULL) {
    printf("ERROR: Failed to allocate a memory block for variables.\n");
    return 1;
  }

  /*
    Start the L-BFGS optimization; this will invoke the callback functions
    evaluate() and progress() when necessary.
  */
  lbfgsfloatval_t fx;
  int ret = lbfgs(n, x, &fx, evaluate, progress, NULL, param);

  /* Report the result. */
  printf("  fx = %f, x[0] = %f, x[1] = %f\n", fx, x[0], x[1]);

  return ret;
}
/* sample.c:1 ends here */
