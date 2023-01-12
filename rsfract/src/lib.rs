use pyo3::prelude::*;
use ndarray;
use numpy::{self, IntoPyArray};
use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::sync::{Arc, Mutex};
use std::thread;
use rayon::prelude::*;
use num_complex::Complex;


#[pymodule]
fn rsfract(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn generate_noise<'py>(py: Python<'py>, size: usize) -> &'py numpy::PyArray3<u8> {
        let mut rng = rand::thread_rng();

        let mut image = ndarray::Array3::<u8>::zeros((size, size, 3));
        for i in 0..size {
            for j in 0..size {
                for k in 0..3 {
                    image[[i, j, k]] = rng.gen_range(0..255);
                }
            }
        }

        image.into_pyarray(py)
    }

    #[pyfn(m)]
    fn generate_noise_threaded<'py>(py: Python<'py>, size: usize) -> &'py numpy::PyArray3<u8> {
        let mut handles = vec![];
        let threads = 4;
        let row_segment = size / threads;

        for _ in 0..threads {
            let mut image_part = ndarray::Array3::<u8>::zeros((row_segment, size, 3));
            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for i in 0..row_segment {
                    for j in 0..size {
                        for k in 0..3 { 
                            image_part[[i, j, k]] = rng.gen_range(0..255);
                        }
                    }
                }
                image_part
            });
            handles.push(handle);
        }

        let mut image = ndarray::Array3::<u8>::zeros((0, size, 3));
        for handle in handles {
            let image_part = handle.join().unwrap();
            image = ndarray::concatenate(ndarray::Axis(0), &[image.view(), image_part.view()]).unwrap();
        }

        image.into_pyarray(py)
    }

    #[pyfn(m)]
    fn generate_noise_threaded_with_locks<'py>(py: Python<'py>, size: usize) -> &'py numpy::PyArray3<u8> {
        let image = Arc::new(Mutex::new(ndarray::Array3::<u8>::zeros((size, size, 3))));
        let mut handles = vec![];
        let threads = 4;

        for t in 0..threads {
            let image = Arc::clone(&image);
            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for i in (t..size).step_by(threads) {
                    for j in 0..size {
                        for k in 0..3 {
                            let mut pixels = image.lock().unwrap(); 
                            pixels[[i, j, k]] = rng.gen_range(0..255);
                        }
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let result = image.lock().unwrap().to_owned();
        result.into_pyarray(py)
    }

    #[pyfn(m)]
    fn generate_noise_parallel<'py>(py: Python<'py>, size: usize) -> &'py numpy::PyArray3<u8> {      
        let mut image = ndarray::Array3::<u8>::zeros((size, size, 3));
        
        image.iter_mut().par_bridge().for_each(|x| {
            let mut rng = SmallRng::from_entropy();
            *x = rng.gen_range(0..255);
        });
    
        image.into_pyarray(py)
    }

    #[pyfn(m)]
    fn generate_mandelbrot<'py>(py: Python<'py>, size: usize) -> &'py numpy::PyArray3<u8> {
        let mut image = ndarray::Array3::<u8>::zeros((size, size, 3));
        let xmin = -2.0;
        let xmax = 1.0;
        let ymin = -1.5;
        let ymax = 1.5;

        let brightness = 2.0;
        let start_color = [64, 0, 64];
        let end_color = [255, 0, 255];
        let gradient = [end_color[0] - start_color[0], end_color[1] - start_color[1], end_color[2] - start_color[2]];

        for i in 0..size {
            for j in 0..size {
                let x = xmin + (xmax - xmin) * (i as f64) / (size as f64);
                let y = ymin + (ymax - ymin) * (j as f64) / (size as f64);
                let c = Complex::new(x, y);
                let mut z = Complex::new(0.0, 0.0);
                let mut count = 0;
                while count < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    count += 1;
                }
                match count {
                    255 => {
                        for k in 0..3 {
                            image[[i, j, k]] = 0;
                        }
                    },
                    _ => {
                        for k in 0..3 {
                            image[[i, j, k]] = (
                                ((count as f64 / 255.0) * gradient[k] as f64 + start_color[k] as f64) * brightness
                            ) as u8;
                        }
                    }
                };
                
            }
        }
        image.into_pyarray(py)
    }

    Ok(())
}
