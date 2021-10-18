## Project-X (TODO: Change me)

You like AWS?
You like privacy?
You love _Project-X_

## Benchmarking

Run
```
cargo bench
```

View CPU clock speeds
```
watch -n.1 "grep \"^[c]pu MHz\" /proc/cpuinfo"
```

View results
```
firefox target/criterion/report/index.html
```

Some compiled version results can be found in `whitepaper/benchmarks/`

### Flamegraph

This will generate a flamegraph that visualizes the time spent by the CPU in a given stack frame.

Some dependencies:

- `perf`. E.g. for Debian-based distros: `apt install linux-tools-generic`
- `cargo install flamegraph`

Run the profiler for everything:
```
cargo flamegraph --bench benchmark  -- --bench
```

or for a specific benchmark:
```
cargo flamegraph --bench benchmark  -- multiplication/Multiplication/4096 --bench
```

or for a specific test:
```
cd rlwe
cargo flamegraph --test test --dev -- mnist
```


Then look at the flamegraph:
```
firefox flamegraph.svg
```
