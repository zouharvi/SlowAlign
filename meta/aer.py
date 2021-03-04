#!/usr/bin/env python
import sys
import numpy as np

aers = []
for (i, (f, e, g, a)) in enumerate(zip(open(sys.argv[1]), open(sys.argv[2]), open(sys.argv[3]), open(sys.argv[4]))):
    fwords = f.strip().split()
    ewords = e.strip().split()
    sure = set([tuple(map(int, x.split("-"))) for x in filter(lambda x: x.find("-") > -1, g.strip().split())])
    possible = set([tuple(map(int, x.split("?"))) for x in filter(lambda x: x.find("?") > -1, g.strip().split())])
    alignment = set([tuple(map(int, x.split("-"))) for x in a.strip().split()])
    alignment = {(x+1,y+1) for x,y in alignment}

    size_a = len(alignment)
    size_s = len(sure)
    size_a_and_s = len(alignment & sure)
    size_a_and_p = len(alignment & possible) + len(alignment & sure)
    if size_a == 0:
        aers.append(1)
    else:
        aers.append(1-(size_a_and_s + size_a_and_p) / (size_a + size_s))

print("AER", np.average(aers))
print(len(aers))