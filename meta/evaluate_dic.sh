#!/bin/bash

LANG=fren
# --file1 data/data_${LANG}_s --file2 data/data_${LANG}_t \
# --gold-index-one
# --switch-dic \

cargo run --release --bin slow_align -- \
     --file1 /tmp/data_x_s --file2 /tmp/data_x_t \
     --gold data/data_${LANG}_a --method dic \
     --test-offset 0 --dic data/dic/europarl_${LANG}.dic \
     --lowercase \
     --params "[0.1], [0.29], [0.7], [0.08, 0.0001], [0.76], [0.0000]" \
     > /tmp/data_x_v_full
head -n 37 /tmp/data_x_v_full > /tmp/data_x_v
head -n 37 data/data_${LANG}_a > /tmp/data_x_g
head -n 37 data/data_${LANG}_s > /tmp/data_x_s
head -n 37 data/data_${LANG}_t > /tmp/data_x_t
python3 meta/aer.py /tmp/data_x_g /tmp/data_x_v

# paste data/data_${LANG}_s data/data_${LANG}_t  -d"|" | sed "s/|/ ||| /g" > /tmp/data_x
# ~/bin/fast_align/build/fast_align -i /tmp/data_x -dov > /tmp/data_x_a_full
# tail -n 50 /tmp/data_x_a_full > /tmp/data_x_a
# python3 meta/aer.py /tmp/data_x_s /tmp/data_x_t /tmp/data_x_g /tmp/data_x_a