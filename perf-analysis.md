# Branch Misprediction Analysis: c_mul vs h_mul

## Hypothesis

h_mul (log-jumps) saves a few multiplications over c_mul (CIOS), but its output
exceeds p ~68% of the time (vs ~5% for c_mul). The conditional subtraction
needed to reduce the result back into [0, p) is poorly predicted at 68%,
erasing the arithmetic savings.

## Evidence

### The reduction step is free for c_mul, expensive for h_mul

Using `bind_random_coeffs` on an i7-14700K (P-core counters):

| | Miss Rate | IPC |
|---|---|---|
| **c_mul** no_reduce | 1.27% | 3.25 |
| **c_mul** correct_reduce | 1.18% | 3.36 |
| | | |
| **h_mul_jit** no_reduce | 1.32% | 3.10 |
| **h_mul_jit** correct_reduce | 4.00% | 2.50 |

For c_mul, adding the reduction changes nothing — the branch is predicted
correctly ~95% of the time. IPC even ticks up slightly (the extra instructions
have good ILP).

For h_mul, adding the reduction **triples the miss rate** (1.3% → 4.0%) and
**drops IPC by 20%** (3.10 → 2.50). Each misprediction flushes ~15–17 cycles of
pipeline on Raptor Cove, and at 68% overflow the predictor can never settle on a
stable pattern.

### Baselines confirm the gap comes from reduction

All no_reduce variants have nearly identical miss rates (~1.3%) regardless of
algorithm. The arithmetic cores of c_mul and h_mul are equally predictable. The
divergence appears only when the conditional subtraction is present.

### Absolute branch misses

| Variant (correct_reduce) | Branch Misses | Ratio |
|---|---|---|
| c_mul | 134M | 1.0× |
| h_mul | 832M | 6.2× |
| h_mul_jit | 415M | 3.1× |

h_mul has 6× more mispredictions than c_mul. h_mul_jit has half the total
branches of h_mul (fewer register-spill branches), but the same miss rate and
same IPC — confirming the bottleneck is the reduction branch, not register
pressure.

### Wall-clock times match

Criterion reports h_mul correct_reduce at ~1,065 µs vs c_mul at ~810 µs — about
**31% slower**. This is consistent with the 20% IPC drop plus the extra
instructions in h_mul's double-reduction path.

## Conclusion

The branch misprediction hypothesis is confirmed. h_mul's ~68% overflow rate
makes the conditional subtraction unpredictable, costing ~20% IPC and ~31%
wall-clock time. c_mul's ~5% overflow rate is well-predicted and the reduction
is effectively free.
