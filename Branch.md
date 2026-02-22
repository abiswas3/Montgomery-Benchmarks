# Branch Misprediction Analysis — Linux perf Guide

## Context

This repo benchmarks two Montgomery multiplication algorithms for the BN-254 scalar field
(256-bit prime, 4 × 64-bit limbs):

- **c-mul** (`src/cios.rs`): CIOS algorithm. Output exceeds p ~5% of the time. Needs 1 conditional subtraction.
- **h-mul** (`src/logjumps.rs`): Log-jumps algorithm (Yuval Domb). Output exceeds p ~68% of the time. Needs up to 2 conditional subtractions.

**Hypothesis**: h-mul saves a few multiplications vs c-mul, but the high overflow rate (~68%) causes
severe branch misprediction on the conditional subtraction, erasing the gain. We want `perf` to
confirm this with hardware counters.

## Toolchain

Requires nightly Rust (`rust-toolchain.toml` is present). Build with:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --no-run
```

Bench binaries land in `target/release/deps/`.

## Bench Binaries

Two bench files:

| Binary pattern                    | Groups                                                      |
|-----------------------------------|-------------------------------------------------------------|
| `bench_sumcheck_bind-*`           | `bind_small_coeffs`, `bind_random_coeffs`                   |
| `bench_mont_mult_barebones-*`     | `no_reduce`, `one_reduce`, `correct_reduce`                  |

Within each group, functions are: `no_reduce/c_mul`, `no_reduce/h_mul`, `no_reduce/h_mul_jit`,
`correct_reduce/c_mul`, `correct_reduce/h_mul`, `correct_reduce/h_mul_jit`.

## What We Want to Measure

Run `perf stat` on the correct_reduce variants and compare branch-miss rates:

```bash
BINARY=./target/release/deps/bench_sumcheck_bind-<hash>

# c-mul — expect ~5% branch miss rate
perf stat -e branches,branch-misses,instructions,cycles -r 5 \
  $BINARY --bench "bind_random_coeffs/correct_reduce/c_mul"

# h-mul — expect ~68% branch miss rate
perf stat -e branches,branch-misses,instructions,cycles -r 5 \
  $BINARY --bench "bind_random_coeffs/correct_reduce/h_mul"

# h-mul-jit (low register pressure variant) — same ~68% miss rate as h-mul
perf stat -e branches,branch-misses,instructions,cycles -r 5 \
  $BINARY --bench "bind_random_coeffs/correct_reduce/h_mul_jit"
```

Also run the no_reduce variants as a baseline (both should have ~0% branch misses):

```bash
perf stat -e branches,branch-misses,instructions,cycles -r 5 \
  $BINARY --bench "bind_random_coeffs/no_reduce/c_mul"

perf stat -e branches,branch-misses,instructions,cycles -r 5 \
  $BINARY --bench "bind_random_coeffs/no_reduce/h_mul"
```

Repeat the above for `bind_small_coeffs` (coefficients are 32-bit values near zero rather than
full 256-bit field elements — interesting to see if the distribution changes the misprediction rate).

## Expected Results (from macOS wall-time benchmarks)

| Variant                              | vs c-mul    |
|--------------------------------------|-------------|
| no_reduce/h_mul                      | ~10% slower |
| correct_reduce/h_mul                 | ~34% slower |

The `perf` numbers should show that `correct_reduce/h_mul` has dramatically more `branch-misses`
than `correct_reduce/c_mul`, matching the ~68% overflow rate.

## Suggested perf Script

Save output for the paper. Run everything in one shot:

```bash
#!/usr/bin/env bash
set -e

RUSTFLAGS="-C target-cpu=native" cargo bench --no-run 2>/dev/null
BIND_BIN=$(ls target/release/deps/bench_sumcheck_bind-* | grep -v '\.d' | head -1)

GROUPS=("bind_small_coeffs" "bind_random_coeffs")
VARIANTS=("no_reduce/c_mul" "no_reduce/h_mul" "no_reduce/h_mul_jit" \
          "correct_reduce/c_mul" "correct_reduce/h_mul" "correct_reduce/h_mul_jit")

for group in "${GROUPS[@]}"; do
  for variant in "${VARIANTS[@]}"; do
    echo "========================================"
    echo "  $group / $variant"
    echo "========================================"
    perf stat -e branches,branch-misses,instructions,cycles -r 5 \
      "$BIND_BIN" --bench "${group}/${variant}" 2>&1
    echo ""
  done
done
```

## Key Files

```
src/cios.rs           — c-mul (CIOS): cios / cios_no_reduce
src/logjumps.rs       — h-mul: mul_logjumps_unr_2 / _no_reduce / _one_reduce
src/logjumps_jit.rs   — h-mul-jit (low register pressure): mul_logjumps_jit / _no_reduce
src/fa.rs             — primitives: mult, wadd, ge_p, reduce_once/twice_if_needed
src/constants.rs      — BN-254 constants: P, 2P, I1, I2, MU0
benches/bench_sumcheck_bind.rs       — independent mult benchmark (primary)
benches/bench_mont_mult_barebones.rs — chained mult benchmark (secondary)
```

## Notes

- All inputs are rejection-sampled to lie in `[0, p)` — the algorithms require this.
- `h_mul_jit` is a low-register-pressure variant of h_mul (avoids stack spills on ARM's 31 GP
  registers by computing products just-in-time). It does NOT change the overflow rate — branch
  misprediction is the same. The JIT variant isolates register pressure from branch prediction.
- `perf` may need `sudo` or `/proc/sys/kernel/perf_event_paranoid` set to 1:
  ```bash
  echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
  ```
