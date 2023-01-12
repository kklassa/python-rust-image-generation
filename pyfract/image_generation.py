import numpy as np


def generate_mandelbrot(size: int) -> np.ndarray:
    '''
    Generates an image of the Mandelbrot set as a numpy.ndarray of shape (size, size, 3)
    '''

    mandelbrot = np.zeros((size, size, 3), dtype=np.uint8)
    xmin = -2.0
    xmax = 1.0
    ymin = -1.5
    ymax = 1.5

    brightness = 2.0
    start_color = [0, 64, 64]
    end_color = [0, 255, 255]
    gradient = [end_color[0] - start_color[0], end_color[1] - start_color[1], end_color[2] - start_color[2]]

    for i in range(size):
        for j in range(size):
            x = xmin + (xmax - xmin) * i / size
            y = ymin + (ymax - ymin) * j / size
            c = complex(x, y)
            z = complex(0, 0)
            count = 0
            while count < 255 and abs(z) <= 2.0:
                z = z * z + c
                count += 1
            if count == 255:
                for k in range(3):
                    mandelbrot[i][j][k] = 0
            else:
                for k in range(3):
                    mandelbrot[i][j][k] = ((count / 255) * gradient[k] + start_color[k]) * brightness
    
    return mandelbrot


def generate_noise(size: int) -> np.ndarray:
    '''
    Generates an image of random noise as a numpy.ndarray of shape (size, size, 3)
    '''
    image = np.zeros((size, size, 3), dtype=np.uint8)

    for i in range(size):
        for j in range(size):
            for k in range(3):
                image[i][j][k] = np.random.randint(0, 256)

    return image