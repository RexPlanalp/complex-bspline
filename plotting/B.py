import numpy as np
import matplotlib.pyplot as plt

B = np.loadtxt("B.txt")

n_splines =  30
n_x = B.shape[0] // n_splines

real = B[:, 0].reshape(n_splines, n_x)
imag = B[:, 1].reshape(n_splines, n_x)

for i in range(n_splines):
    plt.plot(real[i])

plt.show()