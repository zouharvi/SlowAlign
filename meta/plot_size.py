#!/usr/bin/env python3

import matplotlib.pyplot as plt
import argparse

parser = argparse.ArgumentParser(description='Plot AER w.r.t. train size')
parser.add_argument('aers_file', help='Path to the AERs dump')
parser.add_argument('--lang', default="cs", help='Lang to use (either cs or de)')
args = parser.parse_args()
RARROW = "$\\rightarrow$"

if args.lang == "cs":
    title = "EN" + RARROW + "CS"
    transfer = [
        ("Transfer FR"+RARROW+"CS", 42.0, ":"),
        ("Transfer DE"+RARROW+"CS", 42.6, "-."),
    ]
elif args.lang == "de":
    title = "DE" + RARROW + "EN"
    transfer = [
        ("Transfer FR"+RARROW+"DE", 48.4, ":"),
        ("Transfer CS"+RARROW+"DE", 47.0, "-."),
    ]
else:
    raise Exception("Unknown language")

with open(args.aers_file, 'r') as f:
    aers = [line.split() for line in f.readlines()]
    aers = [(int(x[0]), float(x[1])*100, float(x[2])*100) for x in aers]
    aers.sort(key=lambda x: x[0])
    aers = [(5*i-200 if i >= 50 else i, x[0], x[1], x[2]) for i,x in enumerate(aers)]

points_tr = [x[2] for x in aers]
points_ts = [x[3] for x in aers]
min_tr = min(points_tr)
min_ts = min(points_ts)
points_tr_min = [x for x in aers if x[2] == min_tr]
points_ts_min = [x for x in aers if x[3] == min_ts]

fig = plt.figure(figsize=(6, 4.5))
plt.plot([x[0] for x in aers], points_tr, label="Train AER")
plt.plot([x[0] for x in aers], points_ts, label="Test AER")
plt.scatter([x[0] for x in points_tr_min], [x[2] for x in points_tr_min], color="black", marker=".", s=40, alpha=1, zorder=10)
plt.scatter([x[0] for x in points_ts_min], [x[3] for x in points_ts_min], color="black", marker=".", s=40, alpha=1, zorder=10)
for (label, val, style) in transfer:
    plt.axhline(y = val, color = 'gray', linestyle = style, alpha=0.5, label=label) 
plt.xlabel("Train data size")
plt.ylabel("AER (lower is better)")
plt.xticks(list(range(0,51,5))+[x[0] for x in aers if x[0] > 50], [max(i,1) for i in range(0,51,5)]+[x[1] for x in aers if x[0] > 50],rotation=45)
plt.plot
plt.legend()
plt.title(title)
fig.tight_layout()
plt.show()