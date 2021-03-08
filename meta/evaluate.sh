#!/bin/bash

LANG=deen

# Evaluate performance of SlowAlign and fast_align methods.

cargo run --release --bin slow_align -- \
     --file1 data/data_${LANG}_s --file2 data/data_${LANG}_t \
     --gold data/data_${LANG}_a --method fixed \
     --lowercase --gold-index-one \
     --params "[0.95], [0.90], [0.7], [0.21, 0.0000], [0.70], [0.0050]" \
     --test-offset 0 --dev-count 50 > /tmp/data_x_v_full
#  --gold-index-one
tail -n 50 /tmp/data_x_v_full > /tmp/data_x_v
tail -n 50 data/data_${LANG}_a > /tmp/data_x_g
tail -n 50 data/data_${LANG}_s > /tmp/data_x_s
tail -n 50 data/data_${LANG}_t > /tmp/data_x_t
python3 meta/aer.py /tmp/data_x_g /tmp/data_x_v

paste data/data_${LANG}_s data/data_${LANG}_t  -d"|" | sed "s/|/ ||| /g" > /tmp/data_x
~/bin/fast_align/build/fast_align -i /tmp/data_x -dov > /tmp/data_x_a_full
tail -n 50 /tmp/data_x_a_full > /tmp/data_x_a
python3 meta/aer.py /tmp/data_x_g /tmp/data_x_a