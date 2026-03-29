import numpy as np  
import matplotlib.pyplot as plt

knots = np.loadtxt("knots.txt")

real = knots[:,0]
imag = knots[:,1]


plt.scatter(real,imag, color = "k", s = 2, zorder = 3)
plt.grid(zorder = 0)
plt.title("Knot Vector")
plt.ylabel("Imaginary")
plt.xlabel("Real")
plt.show()