# SlowAlign

The main functionalities of SlowAlign are (1) heuristic parameter estimation in a supervised fashion using gridsearch, (2) combination of multiple soft alignments and (3) data-less alignment based on diagonal alignment, levenstein distance and bluring.

This report is split into the following sections:
- [System Description](#System-Description): introduction to different components of SlowAlign together with the heuristics
- [Baseline Evaluation](#Baseline-Evaluation): dataset overview and evaluation, esp. in comparison to `fast_align`
- [Future Work](#Future-Work): list of further improvements
- [Appendix A](#Appendix-A): technical details including building this project

## System Description

TODO

## Baseline Evaluation

TODO

## Train Data Size

TODO

## Future Work

Built-in http server: TODO

Gridsearch multithreading: TODO

Custom alignment score input: TODO

## Appendix A

SlowAlign is written in Rust. For installation, please refer to the [official resources](https://www.rust-lang.org/tools/install). Assuming that `cargo` is in the path, the main binary can be run as: `cargo run --release --bin slow_align -- <arguments>`. The target binary is going to be stored in `target/release/slow_align`. To just build the binary and not run it, replace `run` with `build`. 