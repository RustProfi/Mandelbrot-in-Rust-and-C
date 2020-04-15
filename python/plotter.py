from matplotlib import pyplot as plt
import sys
import numpy as np

if not sys.argv:
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


plt.plot(cresultsx, cresultsy, label='C')
plt.plot(rustresultsx, rustresultsy, label='Rust')
plt.legend()
plt.title(sys.argv[3])
plt.xlabel('Threads')
plt.ylabel('Time(ms)')
plt.show()
