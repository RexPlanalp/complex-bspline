import numpy as np
import matplotlib.pyplot as plt

dB = np.loadtxt("dB.txt")

n_splines =  30
n_x = dB.shape[0] // n_splines

real = dB[:, 0].reshape(n_splines, n_x)
imag = dB[:, 1].reshape(n_splines, n_x)

for i in range(n_splines):
    plt.plot(real[i], color = "k")
    plt.plot(imag[i], color = "brown")

plt.show()