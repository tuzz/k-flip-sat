## A k-flip local search algorithm for SAT and MAX SAT

[Read the paper](https://github.com/tuzz/k-flip-sat/blob/master/report/report.pdf)

This repository contains all the code and resources relating to the above
paper. The algorithm explained in the paper is written in Rust. You can run it
for `k=5` on one of the benchmark problems with:

```sh
$ cargo run 5 < evaluate/benchmarks/uf100-01.cnf
```

The code is organised such that `src/main.rs` is high-level, very legible and
directly corresponds to the pseudocode in the paper. The remaining files
implement the encoding and utility functions.

Parts of the encoding include unit tests that you can run with:

```sh
$ cargo test -- --test-threads=1
```

The `evaluate/` directory contains all of the scripts I used to measure the
performance of the algorithm and collate the data. The `evaluate/measurements/`
directory contains all 105,000 files generated and discussed in the paper.
