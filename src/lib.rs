use pyo3::prelude::*;
use std::{thread,time};

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
pub fn perform_computation() {
    let handles: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| {
            let sleep_time = time::Duration::from_millis(5000);
            println!("start sleep for:{} sec", sleep_time.as_secs());
            thread::sleep(sleep_time);
            println!("DONE!")
            // Perform some intensive computation
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(perform_computation, m)?)?;
    Ok(())
}


/* what does this do ?? [LUCY]
#[no_mangle]
pub extern "C" fn call_perform_computation() {
    perform_computation();
}
*/
