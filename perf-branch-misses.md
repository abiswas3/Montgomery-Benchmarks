# How `branch-misses` Is Measured

## Counting, Not Sampling

`perf stat` uses **counting mode**: it programs a hardware register and lets it
increment for the entire run. At the end it reads the total. There are no
interrupts, no per-event records, and effectively zero overhead on the measured
workload. This is distinct from `perf record`, which uses interrupt-driven
**sampling** (the counter overflows every N events, firing a PMI that captures
the instruction pointer).

## The Hardware Counter

Modern x86 CPUs have a Performance Monitoring Unit (PMU) with a small number of
programmable counters (typically 4 general-purpose + 3 fixed per logical core on
Intel). Each general-purpose counter has an associated event-select MSR
(`IA32_PERFEVTSELx`) that you program with an event code + umask to choose what
to count.

`branch-misses` maps to the Intel architectural event
**`BR_MISP_RETIRED.ALL_BRANCHES`** (event `0xC5`, umask `0x00`). The hardware
increments this counter by 1 each time a mispredicted branch micro-op retires
(commits architecturally). Detection happens in the pipeline: the branch
execution unit compares the predicted direction/target with the resolved one and
flags the branch; the flag propagates to the retirement unit where the counter
increments. Because this is an architectural event (Intel SDM Vol 3, Table 19-1),
it is stable across every Intel Core microarchitecture from Core 2 onward.

In the Linux kernel (`arch/x86/events/intel/core.c`):

```c
[PERF_COUNT_HW_BRANCH_MISSES] = 0x00c5,
```

## Multiplexing (Why You See Percentages)

If you request more events than available counters, the kernel **time-multiplexes**:
it round-robins event groups onto the hardware counters on each timer tick
(default 1 ms). Each event tracks `time_enabled` (wall time it was enabled) and
`time_running` (time it was actually on a counter). The final reported count is:

```
final_count = raw_count × (time_enabled / time_running)
```

This is an **extrapolation**, not a measurement. `perf stat` shows the duty cycle
as a percentage, e.g.:

```
831,549,345  cpu_core/branch-misses/  (86.36%)
```

means the counter was only physically scheduled 86% of the time; the reported
value is scaled up. To avoid scaling errors when computing ratios, group events
with `{}` so they are always co-scheduled:

```bash
perf stat -e '{branches,branch-misses}' ./program
```

## Hybrid CPUs (Alder Lake / Raptor Lake)

On Intel hybrid processors, the kernel exposes two PMU devices: `cpu_core`
(P-cores) and `cpu_atom` (E-cores). These are separate PMU implementations with
different counter counts and event constraints. Running `perf stat -e
branch-misses` implicitly creates events on both PMUs. To see them separately:

```bash
perf stat -e cpu_core/branch-misses/,cpu_atom/branch-misses/ ./program
```

Events from different PMUs cannot be grouped together.

## References

1. **Intel SDM, Volume 3, Chapter 19** — defines the architectural PMU and the
   seven guaranteed events (Table 19-1), including `BR_MISP_RETIRED.ALL_BRANCHES`
   (`0xC5/0x00`).
   [PDF](https://cdrdv2-public.intel.com/812391/325384-sdm-vol-3abcd.pdf)

2. **Intel perfmon event database** — searchable per-microarchitecture event
   definitions.
   [perfmon-events.intel.com](https://perfmon-events.intel.com/)

3. **Linux kernel source, `arch/x86/events/intel/core.c`** — contains
   `intel_perfmon_event_map[]` and per-microarchitecture event constraints.
   [GitHub](https://github.com/torvalds/linux/blob/master/arch/x86/events/intel/core.c)

4. **`perf_event_open(2)` man page** — documents `time_enabled`, `time_running`,
   and the multiplexing/scaling mechanism.
   [man7.org](https://man7.org/linux/man-pages/man2/perf_event_open.2.html)

5. **Brendan Gregg, "perf Examples"** — explains counting vs sampling modes.
   [brendangregg.com/perf.html](https://www.brendangregg.com/perf.html)

6. **Hadi Brais, "The Linux perf Event Scheduling Algorithm"** — deep dive into
   how perf schedules events across PMU counters.
   [hadibrais.wordpress.com](https://hadibrais.wordpress.com/2019/09/06/the-linux-perf-event-scheduling-algorithm/)
