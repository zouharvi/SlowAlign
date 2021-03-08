#!/bin/bash

# Run dictionary configuration, pass params through the arguments

LANG=encs

cargo run --release --bin slow_align -- \
     --file1 /tmp/data_x_s --file2 /tmp/data_x_t \
     --gold data/data_${LANG}_a --method dic \
     --test-offset 0 --dic data/dic/europarl_${LANG}.dic \
     --lowercase \
     --params "$1" \
     > /tmp/data_x_v_full
tail -n 50 /tmp/data_x_v_full > /tmp/data_x_v
tail -n 50 data/data_${LANG}_a > /tmp/data_x_g
tail -n 50 data/data_${LANG}_s > /tmp/data_x_s
tail -n 50 data/data_${LANG}_t > /tmp/data_x_t
python3 meta/aer.py /tmp/data_x_g /tmp/data_x_v