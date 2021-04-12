import matplotlib.pyplot as plt
import numpy as np
import sklearn.cluster as cl

x = np.random.rand(10000)
y = np.random.rand(10000)

pred = cl.KMeans(n_clusters=8).fit_predict(np.c_[x, y])

plt.scatter(x, y, c=pred)
plt.colorbar()
plt.savefig("kmeans.png")
