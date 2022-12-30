use pyo3::prelude::*;
use ndarray;
use numpy::{self, IntoPyArray};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;


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
    fn generate_noise_threaded<'py>(py: Python<'py>, size: usize, threads: usize) -> &'py numpy::PyArray3<u8> {
        let image = Arc::new(Mutex::new(ndarray::Array3::<u8>::zeros((size, size, 3))));
        let mut handles = vec![];

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

    Ok(())
}
