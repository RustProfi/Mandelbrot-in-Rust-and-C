from matplotlib import pyplot as plt
import sys
import numpy as np

if len(sys.argv) == 1:
    print("usage: plotter.py <filetocresults> <filetorustresults> <title>")
    exit(1)

file1 = open(sys.argv[1], 'r')
lines1 = file1.readlines()
file2 = open(sys.argv[2], 'r')
lines2 = file2.readlines()

cresultsx = []
cresultsy= []
rustresultsx=[]
rustresultsy=[]

for line in lines1:
    x = line.split(',')
    cresultsx.append(int(x[0]))
    cresultsy.append(float(x[1]))

for line in lines2:
    x = line.split(',')
    rustresultsx.append(int(x[0]))
    rustresultsy.append(float(x[1]))

#points for annotation
ymin1 = min(cresultsy)
xpos1 = cresultsy.index(ymin1)
xmin1 = cresultsx[xpos1]

ymin2 = min(rustresultsy)
xpos2 = rustresultsy.index(ymin2)
xmin2 = rustresultsx[xpos2]

bbox = dict(boxstyle="round", fc="0.8")
arrowprops = dict(
    arrowstyle = "->",
    connectionstyle = "angle,angleA=0,angleB=90,rad=10")

plt.plot(cresultsx, cresultsy, label='C')
plt.plot(rustresultsx, rustresultsy, label='Rust')
plt.legend()
plt.title(sys.argv[3])
plt.xlabel('Threads')
plt.ylabel('Time(ms)')
plt.annotate('Min C: %d, %.1fms'%(xmin1, ymin1), (xmin1, ymin1), xytext=(xmin1, ymin1 + 1000), bbox=bbox, arrowprops=arrowprops)
plt.annotate('Min Rust: %d, %.1fms'%(xmin2, ymin2), (xmin2, ymin2), xytext=(xmin2, ymin2 + 500), bbox=bbox, arrowprops=arrowprops)
plt.show()
