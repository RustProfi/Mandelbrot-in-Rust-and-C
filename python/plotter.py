from matplotlib import pyplot as plt
import sys
import numpy as np

if len(sys.argv) == 1:
    print("usage: plotter.py <filetogccresults> <filetoclangresults> <filetorustresults> <title>")
    exit(1)
#gcc
file1 = open(sys.argv[1], 'r')
gccresults = file1.readlines()
gccx = []
gccy = []
for line in gccresults:
    x = line.split(',')
    gccx.append(int(x[0]))
    gccy.append(float(x[1]))

#x,y min value
y_min_gcc = min(gccy)
xpos = gccy.index(y_min_gcc)
x_min_gcc = gccx[xpos]

#clang
file2 = open(sys.argv[2], 'r')
clangresults = file2.readlines()
clangx = []
clangy = []
for line in clangresults:
    x = line.split(',')
    clangx.append(int(x[0]))
    clangy.append(float(x[1]))

#x,y min value
y_min_clang = min(clangy)
xpos = clangy.index(y_min_clang)
x_min_clang = clangx[xpos]

#rust
file3 = open(sys.argv[3], 'r')
rustresults = file3.readlines()
rustresultsx = []
rustresultsy = []
for line in rustresults:
    x = line.split(',')
    rustresultsx.append(int(x[0]))
    rustresultsy.append(float(x[1]))

y_min_rust = min(rustresultsy)
xpos = rustresultsy.index(y_min_rust)
x_min_rust = rustresultsx[xpos]

#plot
bbox = dict(boxstyle="round", fc="0.8")
arrowprops = dict(
    arrowstyle = "->",
    connectionstyle = "angle,angleA=0,angleB=90,rad=10")

plt.plot(gccx, gccy, label='C with gcc')
plt.plot(clangx, clangy, label='C with clang')
plt.plot(rustresultsx, rustresultsy, label='Rust')
plt.legend()
plt.title(sys.argv[4])
plt.xlabel('Threads')
plt.ylabel('Time(ms)')
#gcc
plt.annotate('Min. value C with gcc\n%d Threads\n%.2f ms'%(x_min_gcc, y_min_gcc), (x_min_gcc, y_min_gcc), xytext=(x_min_gcc-40, y_min_gcc + 300), bbox=bbox, arrowprops=arrowprops)
#clang
plt.annotate('Min. value C with clang\n%d Threads\n%.2f ms'%(x_min_clang, y_min_clang), (x_min_clang, y_min_clang), xytext=(x_min_clang-10, y_min_clang + 300), bbox=bbox, arrowprops=arrowprops)
#Rust
plt.annotate('Min. value Rust\n%d Threads\n%.2f ms'%(x_min_rust, y_min_rust), (x_min_rust, y_min_rust), xytext=(x_min_rust+10, y_min_rust + 300), bbox=bbox, arrowprops=arrowprops)
plt.show()
