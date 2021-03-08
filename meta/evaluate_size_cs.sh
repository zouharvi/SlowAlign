#!/bin/bash

LANG=encs
echo "My ID is $2"

# Evaluate performance with varying train data size
# Can be run concurrently, as the probability of two clashing file writes is extremely low
# Use different IDs (replace with automatic PID in the future)
for SIZE in $1; do
     echo $SIZE;
     cargo run --release --bin slow_align -- \
          --file1 data/data_${LANG}_s --file2 data/data_${LANG}_t \
          --gold data/data_${LANG}_a --method search \
          --lowercase --gold-index-one \
          --test-offset 2500 --dev-count $SIZE \
          1> /dev/null 2> /tmp/alignment_err_${LANG}_$2 & wait;
     TEST_AER=$(tail -n 2 /tmp/alignment_err_${LANG}_$2 | head -n 1 | sed "s/AER: //")
     TRAIN_AER=$(tail -n 5 /tmp/alignment_err_${LANG}_$2 | head -n 1 | sed "s/Best AER: //")
     PARAMS=$(tail -n 4 /tmp/alignment_err_${LANG}_$2)
     echo $PARAMS
     echo $SIZE $TRAIN_AER $TEST_AER >> data/aers_size_${LANG};
done