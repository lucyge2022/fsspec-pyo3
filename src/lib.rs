use pyo3::prelude::*;
use std::{thread,time};
use pyo3::types::PyList;
use std::error::Error;
use std::io::Read;
use bytes::Bytes;
use tokio::runtime::{Builder, Runtime};
use reqwest::blocking;
use std::sync::Arc;

use pyo3::{
    exceptions::PyIOError,
    exceptions::PyIndexError,
    exceptions::PyValueError,
    prelude::{pymodule, PyModule, PyResult, Python},
    types::PyBytes,
    PyErr,
};

// lazy_static::lazy_static! {
//     static ref ALLUXIO_RUNTIME: Runtime = Builder::new_multi_thread()
//         .worker_threads(num_cpus::get())
//         // .thread_stack_size(3 * 1024 * 1024)
//         .thread_name("alluxiofsspec")
//         .build()
//         .unwrap();
// }

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
// fn multi_http_requests(urls: Vec<String>) -> PyResult<PyList> {
fn multi_http_requests(py: Python, urls: Vec<String>, ranges: Vec<(isize,isize)>) -> PyObject {
                                                                      //Vec<Result<String, reqwest::Error>> {
    let ALLUXIO_RUNTIME: Runtime = Builder::new_multi_thread()
        .worker_threads(4)
        // .thread_stack_size(3 * 1024 * 1024)
        .thread_name("alluxiofsspec")
        .build()
        .unwrap();

    let num_reqs = urls.len();
    let mut content_results = Vec::with_capacity(num_reqs);
    let mut senders = Vec::with_capacity(num_reqs);
    for _ in 0..num_reqs {
        senders.push(None);
    }

    let mut threads = Vec::new();
    for i in 0..num_reqs {
        let url_owned = urls[i].to_owned();
        let range_owned = ranges[i].to_owned();
        let (send, recv) = tokio::sync::oneshot::channel();
        // handles.push(ALLUXIO_RUNTIME.spawn_blocking(move || -> Result<(), reqwest::Error> {
        threads.push(thread::spawn(move || -> Result<(), reqwest::Error> {
            println!("request idx:{}", i);
            let body = perform_http_get(url_owned.as_str(), range_owned);
            send.send(body).unwrap();
            Ok(())
        }));
        // Store the sender in the vector
        senders[i] = Some(recv);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    for sender in senders {
        let result = sender.unwrap().blocking_recv().unwrap();
        content_results.push(result);
    }
    println!("content_results:{:?}", content_results);
    let mut concatenated_data: Vec<u8> = Vec::new();
    for content in &content_results {
        concatenated_data.extend(content.as_ref().unwrap());
    }
    // ?? what return type to python?
    // return content_results;
    PyBytes::new(py, &concatenated_data).into()
}

fn type_name_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

// #[tokio::main]
fn perform_http_get(url: &str, range: (isize, isize)) -> Result<Vec<u8>, reqwest::Error> {
    // let body = reqwest::blocking::get(url)?
    //     .text()?;
    // Ok(body)
    let bytes: Vec<u8> = Vec::new();
    Ok(bytes)
    // let bytes: Vec<u8> = reqwest::blocking::get(url)?
    //     .bytes()?
    //     .iter()
    //     .map(|b| b.to_be()).collect();
    // Ok(bytes);

    // let client = reqwest::Client::new();
    // let resp = client.get(url)
    //     .header("Range",format!("bytes={}-{}", range.0, range.1))
    //     .send()
    //     .await?;
    // let bytes = resp.bytes().await?;
    // // println!("bytes: {:?}", bytes);
    // Ok(bytes)
}

#[cfg(test)] // Indicates that the following functions are only compiled when running tests
mod tests {
    // Import the `add` function from the parent module
    use super::*;

    #[test]
    fn test_lucy() {
        println!("LUCY!");
    }

    #[test]
    fn test_add() {
        let url1 = "https://lucybucket2022.s3.us-east-1.amazonaws.com/testfolder/file1?response-content-disposition=inline&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEGYaCXVzLXdlc3QtMSJHMEUCIQCujkAYqh%2FTKQMcSK6jf2VFuJCbYUWHS39KsiYC7MPMtAIgCgaeaPCTt1bpTDzFMJNEJn6CVEXmWIH1kO91yFwcAPoq5AIIfxAAGgwzODQ1Mjg2MzQ5MDEiDN51L%2Brx5BkmBvOjCSrBAkeEQFtZEi6XvbV1%2Fq2uGthRyl75wiVQ2GieArPMw9nTFYVCGsMPbRPcZDLddUAZ%2FgR%2Ffs4gT6iPpLP2Yz0wZ6hgOU9IFZP6MD4476dnegVHK%2BafAO6RDWTYvR4AnJevigvQ1GECrCMXecFlpfEWOn1L03BObHTQ2YHSJCP8lYliG2s4kCBe0B0EIL6wT76Fgx4a1gRzBMdQzAebRTqCs5FofpShwzSmoYCETSOuwIwrqFql30HZMV08FMqzGkECblq%2BTpRX%2FpkrV2aK%2FcnsNuaMgvdej2iTEXjjwn6vM8yu3iODMRYMYzQjKf7wAArr3AYxIlFT80hf%2FfztACBkY%2FbNxgm0QQJ1c%2BpecSTmWIpdPLTLCCMhDoThdx%2BIijCNTGEtzFUfgGlUQxhn0qtsubQj1Vx0Ao0sqIXZKQdIPGO9dzD8woKwBjqzAtVINKuFZ%2BqMCF9Dt3CAqFy%2B9ivcO7jBemeBRevX7WFFUeGdaD13vK%2BZ5OsuPWOSLL%2FLDNGyLkdCZ1sYyNQ6ubr%2F64yYZG3iT%2BX2N70YUUrrlODUF8D%2BCEeg%2F0FkMt4OJTG5ufIKyooBuRMFHX9bFQ7q%2B4UJL98mIeGCRoXUTNho0fwHoLqDVO64BVyYVL4HdNK5BqS4Ti%2BCNMOA6EnEAQUYEH1ac0HVZ3gCe3hyyFIrGFzuy6NQOqsAQNsRKXfv4ufJfqKXP%2F0W6CU8R08h2G%2BT5NjgYxx0w%2BsG%2FctPmvkGUcHKT4XE800dRAs9E2AyTJwIjiqGk7au4h6myHeduLQMy1lzXSuyYpYx8dJ5QqmcFyKFWw8v7BvJkhfRXJherlWyFNSJZSDDGOLbr%2BmMP3pOgwY%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20240324T215817Z&X-Amz-SignedHeaders=host&X-Amz-Expires=43200&X-Amz-Credential=ASIAVTB5RKQKUWBCIRFO%2F20240324%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Signature=76fba8874a37da4e53e452499dd7369b280f6589aec9b31e384c0b29969ab72d";
        let urls = vec![
            String::from(url1),
            String::from(url1)
        ];
        let ranges = vec![
            (0,0),
            (0,0)
        ];
        // let gil = Python::acquire_gil();
        // let py = gil.python();
        // let results = multi_http_requests(py, urls, ranges);
        // println!("results:{:?}", results);
    }
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
fn alluxiocommon(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(perform_computation, m)?)?;
    m.add_function(wrap_pyfunction!(multi_http_requests, m)?)?;
    Ok(())
}


/* what does this do ?? [LUCY]
#[no_mangle]
pub extern "C" fn call_perform_computation() {
    perform_computation();
}
*/
