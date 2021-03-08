#!/usr/bin/env python
import sys
import numpy as np
import argparse
"""
Adapted from hw2 of https://github.com/xutaima/jhu-mt-hw
"""

parser = argparse.ArgumentParser(description='Process some integers.')
parser.add_argument('gold', help='Gold alignments file')
parser.add_argument('hypothesis', help='Hypothesis alignments file')
parser.add_argument('--gold-index-one', action='store_true', help="Shift hypothesis by one.")
args = parser.parse_args()

aers = []
for (i, (algn_gold, algn_hypothesis)) in enumerate(zip(open(args.gold), open(args.hypothesis))):
    sure = set([tuple(map(int, x.split("-"))) for x in algn_gold.strip().split() if x.find("-") > -1])
    possible = set([tuple(map(int, x.split("?"))) for x in algn_gold.strip().split() if x.find("?") > -1])
    alignment = set([tuple(map(int, x.split("-"))) for x in algn_hypothesis.strip().split()])
    if args.gold_index_one:
        alignment = {(x+1,y+1) for x,y in alignment}

    size_a = len(alignment)
    size_s = len(sure)
    size_a_and_s = len(alignment & sure)
    size_a_and_p = len(alignment & possible) + len(alignment & sure)
    aers.append(1-(size_a_and_s + size_a_and_p) / (size_a + size_s))

print("AER", np.average(aers))
print(len(aers))