import numpy as np
import matplotlib.pyplot as plt
from PIL import Image
from time import perf_counter

import rsfract

start = perf_counter()
np_image = np.random.random((1024, 1024, 3))
end = perf_counter()
print('numpy execution time: %s' % (end - start))

start = perf_counter ()
rs_image = rsfract.generate_noise(1024)
end = perf_counter()
print('rust execution time: %s' % (end - start))

start = perf_counter ()
rs_image = rsfract.generate_noise_threaded(1024, 4)
end = perf_counter()
print('rust threaded execution time: %s' % (end - start))

fig, axs = plt.subplots(1, 2)

img1 = axs[0].imshow(np_image)
axs[0].set_title('numpy')

img2 = axs[1].imshow(rs_image)
axs[1].set_title('rust')

for ax in axs:
    ax.axis('off')
plt.show()

image = Image.fromarray(rs_image)
image.save('images/out/image.png')
