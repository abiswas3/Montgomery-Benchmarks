# Measurement Methodology

## Machine

| | |
|---|---|
| CPU | Intel Core i7-14700K (Raptor Lake, 8P + 12E cores, 28 threads) |
| P-cores | CPUs 0–15, max 5.6 GHz, Raptor Cove (HT enabled) |
| E-cores | CPUs 16–27, max 4.3 GHz, Gracemont |
| Kernel | Linux 6.18.5-arch1-1 (Arch Linux) |
| Rust | rustc 1.95.0-nightly (d940e5684 2026-01-19) |
| perf | 6.19-2 |

## Reproducing

```bash
# 1. Enable hardware counters (resets on reboot)
echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid

# 2. Build bench binaries
RUSTFLAGS="-C target-cpu=native" cargo bench --no-run

# 3. Find the binary (hash changes on rebuild)
BIND_BIN=$(ls target/release/deps/bench_sumcheck_bind-* | grep -v '\.d' | head -1)

# 4. Run all variants
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

Requires nightly Rust (see `rust-toolchain.toml`) and Linux with `perf`
installed. On non-hybrid CPUs the output will show plain `branches` /
`branch-misses` instead of `cpu_core/…` / `cpu_atom/…`.

## Build

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --no-run
```

This produces optimized bench binaries in `target/release/deps/`. The binary
used was `bench_sumcheck_bind-5164c6e343c7b8b7`.

## perf Configuration

```bash
echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

This lowers the paranoid level from 2 (default, restricted) to 1, allowing
unprivileged users to read hardware performance counters.

## Script

The following script was run once, measuring 12 variant/group combinations:

```bash
BIND_BIN=./target/release/deps/bench_sumcheck_bind-5164c6e343c7b8b7

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

### What each flag does

- **`-e branches,branch-misses,instructions,cycles`** — requests four hardware
  events. On this hybrid CPU, each generic event expands to two PMU-specific
  events (`cpu_core/…/` and `cpu_atom/…/`), so 8 events are actually created.
  With 4 general-purpose counters per PMU, this requires time-multiplexing
  (visible as the ~85–90% duty-cycle percentages in the output).

- **`-r 5`** — runs the benchmark binary 5 times and reports the mean ± stddev
  across runs. Each run internally executes Criterion's own warmup + 100-sample
  collection, so the total per-variant is 5 × (warmup + 100 iterations).

- **`--bench "${group}/${variant}"`** — Criterion's filter flag. Only benchmarks
  whose name contains the given substring are executed.

## Which Numbers We Report

The raw `perf stat` output reports counters for both `cpu_core` and `cpu_atom`.
The tables report only the **`cpu_core` (P-core)** counters because:

1. The benchmark threads predominantly ran on P-cores (the OS scheduler favors
   them for CPU-bound work).
2. P-core and E-core branch-miss counts are not directly comparable due to
   different microarchitectures (Raptor Cove vs Gracemont).
3. The `cpu_atom` counters showed proportionally similar trends but with higher
   variance due to lower duty cycles (~15–17%).

### Multiplexing Scaling

All reported values are **scaled estimates**, not raw counts. Because 8 events
competed for 4 counters per PMU, each event was physically measured ~85–90% of
the wall time. `perf stat` extrapolates:

```
reported = raw_count × (time_enabled / time_running)
```

The duty cycle is shown in parentheses in the raw output, e.g. `(86.36%)`.
Cross-run variance (± percentages) incorporates both natural variation and
scaling noise.

## Results

### bind_small_coeffs

| Variant | Branches | Branch Misses | Miss Rate | Instructions | Cycles | IPC |
|---|---|---|---|---|---|---|
| no_reduce/c_mul | 9.16B | 118M | 1.29% | 280B | 88.5B | 3.16 |
| no_reduce/h_mul | 18.6B | 243M | 1.31% | 535B | 180B | 2.97 |
| no_reduce/h_mul_jit | 9.57B | 126M | 1.32% | 273B | 92.3B | 2.95 |
| correct_reduce/c_mul | 11.2B | 120M | 1.07% | 298B | 91.0B | 3.27 |
| correct_reduce/h_mul | 20.3B | 843M | 4.16% | 408B | 163B | 2.51 |
| correct_reduce/h_mul_jit | 10.6B | 435M | 4.11% | 212B | 84.7B | 2.50 |

### bind_random_coeffs

| Variant | Branches | Branch Misses | Miss Rate | Instructions | Cycles | IPC |
|---|---|---|---|---|---|---|
| no_reduce/c_mul | 9.15B | 116M | 1.27% | 279B | 85.8B | 3.25 |
| no_reduce/h_mul | 18.4B | 236M | 1.28% | 529B | 195B | 2.72 |
| no_reduce/h_mul_jit | 9.77B | 129M | 1.32% | 277B | 89.5B | 3.10 |
| correct_reduce/c_mul | 11.4B | 134M | 1.18% | 301B | 89.7B | 3.36 |
| correct_reduce/h_mul | 20.7B | 832M | 4.03% | 417B | 162B | 2.57 |
| correct_reduce/h_mul_jit | 10.4B | 415M | 4.00% | 208B | 82.2B | 2.53 |

### Reading the Tables

- **Branches / Branch Misses** — total retired branch instructions and
  mispredictions across the full `perf stat -r 5` run (5 invocations of the
  bench binary). These are aggregate totals, not per-iteration.
- **Miss Rate** — `branch_misses / branches`. This is the program-wide rate
  across all branch sites, not the per-site misprediction probability of the
  conditional subtraction alone.
- **IPC** — `instructions / cycles`. Higher is better; mispredicted branches
  cause pipeline flushes that reduce IPC.
