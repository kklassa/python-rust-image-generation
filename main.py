import numpy as np
import matplotlib.pyplot as plt
from PIL import Image
from time import perf_counter
import sys

import pyfract
import rsfract


def display_mandelbrot(size: int):
    py_start = perf_counter()
    py_image = pyfract.generate_mandelbrot(size)
    py_end = perf_counter()
    py_time = py_end - py_start
    print(f'Python Mandelbrot set generation time: {py_time}')

    rs_start = perf_counter()
    rs_image = rsfract.generate_mandelbrot(size)
    rs_end = perf_counter()
    rs_time = rs_end - rs_start
    print(f'Rust Mandelbrot set generation time: {rs_time}')

    fig, axs = plt.subplots(1, 2)

    img1 = axs[0].imshow(py_image)
    axs[0].set_title('python')

    img2 = axs[1].imshow(rs_image)
    axs[1].set_title('rust')

    for ax in axs:
        ax.axis('off')
    fig.suptitle('mandelbrot set')
    plt.show()


def display_random_noise(size: int):
    py_start = perf_counter()
    py_image = pyfract.generate_noise(size)
    py_end = perf_counter()
    py_time = py_end - py_start
    print(f'Python random noise generation time: {py_time}')

    np_start = perf_counter()
    np_image = np.random.random((size, size, 3))
    np_end = perf_counter()
    np_time = np_end - np_start
    print(f'Numpy random noise generation time: {np_time}')

    rs_start = perf_counter()
    rs_image = rsfract.generate_noise(size)
    rs_end = perf_counter()
    rs_time = rs_end - rs_start
    print(f'Rust random noise generation time: {rs_time}')

    rs_start = perf_counter()
    rs_thr_image = rsfract.generate_noise_threaded(size)
    rs_end = perf_counter()
    rs_time = rs_end - rs_start
    print(f'Rust random noise generation time (multithreaded): {rs_time}')

    rs_start = perf_counter()
    rs_thr_lck_image = rsfract.generate_noise_threaded_with_locks(size)
    rs_end = perf_counter()
    rs_time = rs_end - rs_start
    print(f'Rust random noise generation time (multithreaded with locks): {rs_time}')

    rs_start = perf_counter()
    rs_thr_par_image = rsfract.generate_noise_parallel(size)
    rs_end = perf_counter()
    rs_time = rs_end - rs_start
    print(f'Rust random noise generation time (parallel): {rs_time}')

    fig, axs = plt.subplots(3, 2)

    axs[0][0].imshow(py_image)
    axs[0][0].set_title('python')

    axs[0][1].imshow(np_image)
    axs[0][1].set_title('numpy')

    axs[1][0].imshow(rs_image)
    axs[1][0].set_title('rust')

    axs[1][1].imshow(rs_thr_image)
    axs[1][1].set_title('rust (multithreaded)')

    axs[2][0].imshow(rs_thr_lck_image)
    axs[2][0].set_title('rust (multithreaded with locks)')

    axs[2][1].imshow(rs_thr_par_image)
    axs[2][1].set_title('rust (parallel)')

    for row in axs:
        for ax in row:
            ax.axis('off')
    fig.suptitle('random noise')
    plt.show()


def main():
    
    while True:
        print('''
        Do you want to:
        1. Generate Mandelbrot set image
        2. Generate random noise image
        3. Quit
        ''')
        choice = int(input('Choose an option: '))
        if choice == 1:
            size = int(input('Size of the image in pixels: '))
            while size%4 != 0:
                size = int(input('The size should be divisble by 4: '))
            display_mandelbrot(size)
            choice = 0
        elif choice == 2:
            size = int(input('Size of the image in pixels: '))
            display_random_noise(size)
            choice = 0
        elif choice == 3:
            sys.exit()
        

if __name__ == "__main__":
    main()
