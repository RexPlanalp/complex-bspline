import numpy as np  
import matplotlib.pyplot as plt
import sys

if len(sys.argv) < 2:
    print(f"Usage: python {sys.argv[0]} <file.txt>")
    sys.exit(1)

file = sys.argv[1]

knots = np.loadtxt("output/" + file)

real = knots[:,0]
imag = knots[:,1]


plt.scatter(real,imag, color = "k", s = 2, zorder = 3)
plt.grid(zorder = 0)
plt.title("Knot Vector")
plt.ylabel("Imaginary")
plt.xlabel("Real")
plt.show()