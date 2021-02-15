---
title: "SlowAlign Report"
author: |
        | Vilém Zouhar
        | <zouhar@ufal.mff.cuni.cz>
date: February 2021
geometry: margin=2.5cm
output: pdf_document
---


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

**Built-in http server:** Word alignment can be used in multiple scenarios. It can be useful for research purposes as a binary installed on a computer, but it is also necessary in some deployment scenarios. Furthermore, sharing an alignment service in a collaboration is easier than to manually send around parallel data to align or to learn to use a specific word alignment tool.
The simplest solution would just accept the request and call the appropriate method from the main binary. This would be resource wasteful, since e.g. the word translation dictionary would have to be loaded from the filesystem (or cached in the memory by the OS) and parsed for every request (usually a pair of sentences). A solution to this would be to specify list of dictionaries to load when starting the server (more secure, but also more restrictive) or keep explicit cache of one most recently used dictionary. 

**Gridsearch multithreading:** Currently, only the IBM soft alignment implementation is multithreaded (fixed to 4 threads), even though it is problematic and bottlenecked between the Expectation and Maximization steps. Inducing hard alignment using the recipes, is however a pure function which only needs a read access to  the soft alignment package. At the end, only the argmax needs to be extracted (parameters) together with the AER. Even though this does not influence all the use-cases and especially the inference scenario, it is possible to gain a multiplicative speedup determined solely by the number of cores (e.g. 8x).

**Custom alignment score input:** It has been shown, that attention scores can be used for word alignment, especially if one of the attention heads is trained for this explicitly. Further improvements can be made if these scores are joined with other soft alignments. Main advantage is also the more complex hard alignment induction scheme. An MT practicioner who wishes to e.g. also present the word alignments next to the MT output may choose to send a request to SlowAlign together with their attention scores (and possibly other parameters) to get a better hard alignment.

## Appendix A

SlowAlign is written in Rust (1.50 in this report). For installation, please refer to the [official resources](https://www.rust-lang.org/tools/install). Assuming that `cargo` is in the path, the main binary can be run as: `cargo run --release --bin slow_align -- <arguments>`. The target binary is going to be stored in `target/release/slow_align`. To just build the binary and not run it, replace `run` with `build`. 

Output of `slow_align --help`:

```
USAGE:
    slow_align [FLAGS] [OPTIONS]

FLAGS:
        --gold-one-index
            Treat gold alignments as if they are 1-indexed (default is 0-indexed)

    -h, --help
            Prints help information

        --lowercase
            Treat everything case-insensitive (default is case-sensitive, even though that
            provides slightly worse performance).

    -V, --version
            Prints version information


OPTIONS:
    -d, --dic <dic>
            OPUS-like dictionary of word translation probabilities

    -f, --file1 <file1>
            Path to the source file to align. (If both files and sentences are provided,
            only files are used).

    -f, --file2 <file2>
            Path to the target file to align. (If both files and sentences are provided,
            only files are used).

    -g, --gold <gold>
            Path to the file with alignments (single space separated, x-y for sure alignments,
            x?y for possible). `x` and `y` are (by default) 0-indexed token indicies.

    -m, --method <method>
            Which alignment method pipeline to use (static, dic, levenstein, ibm1, search)
            [default: static]

    -p, --params <params>
            Comma-separated arrays of parameters to the estimator recipe
            [default: [0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]]

    -s, --sent1 <sent1>
            List of source sentences (separated by \n) to align.

    -s, --sent2 <sent2>
            List of target sentences (separated by \n) to align.
```

&nbsp;

Output of `slow_align_dic --help`:

```
USAGE:
    slow_align_dic [FLAGS] [OPTIONS] <file1> <file2> <out>

ARGS:
    <file1>
            Path to the source file to train word translation probabilities on.
    <file2>
            Path to the target file to train word translation probabilities on.
    <out>
            Path to the output word translation probabilities dictionary.

FLAGS:
    -h, --help
            Prints help information

        --lowercase
            Treat everything case-insensitive (default is case-sensitive, even though
            that provides slightly worse performance).

    -V, --version
            Prints version information


OPTIONS:
    -t, --threshold <threshold>
            Threshold under which translation probabilities will be omitted. Lower values
            lead to better approximation, but also larger file size. [default: 0.2]
```