import numpy as np
import matplotlib.pyplot as plt

dB = np.loadtxt("output/dB.txt")
metadata = np.loadtxt("output/basis_meta.txt")
n_splines = int(metadata[0])
x = metadata[1:]
n_x = len(x)

real = dB[:, 0].reshape(n_splines, n_x)
imag = dB[:, 1].reshape(n_splines, n_x)

for i in range(n_splines):
    plt.plot(x, real[i], color = "k")
    plt.plot(x, imag[i], color = "brown")

plt.show()