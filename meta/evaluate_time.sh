#!/bin/bash

# Measure runtimes of SlowAlign and fast_align.

LANG=deen

# ./target/release/slow_align
# cargo run --release --bin slow_align --
time ./target/release/slow_align \
     --file1 data/data_${LANG}_s --file2 data/data_${LANG}_t \
     --gold data/data_${LANG}_a --method ibm1 \
     --lowercase --gold-index-one \
     --params "[0.95], [0.90], [0.7], [0.21, 0.0000], [0.70], [0.0050]" \
     --test-offset 0 --dev-count 2500 > /tmp/data_x_v_full

paste data/data_${LANG}_s data/data_${LANG}_t  -d"|" | sed "s/|/ ||| /g" > /tmp/data_x
time ~/bin/fast_align/build/fast_align -i /tmp/data_x -dov > /tmp/data_x_a_full