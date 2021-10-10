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

