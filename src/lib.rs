#![feature(let_chains)]
use cty::*;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::wrap_pymodule;
use std::slice;
use std::{thread, time};

use arrow::array::Float64Array;
use arrow::array::PrimitiveArray;
use arrow::buffer::*;
use arrow::error::Result;
//IPC
use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use arrow::record_batch::RecordBatchReader;

fn sleep_int(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}
fn sleep_30_sec() {
    sleep_int(30);
}
//DISCLAIMER, couldn't get it to work to have the pyfunction
//definitions in other modules, the pyo3 macros complained
//so just showed them in here

/**************************************************
 * Basic pyo3 examples
 **************************************************/

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// Joins a string vector
#[pyfunction]
pub fn concat_string(s: Vec<String>) -> PyResult<String> {
    Ok(s.join(","))
}

/**************************************************
 * IPC
 **************************************************/
fn read_recordbatch_from_streamreader(
    sr: Result<StreamReader<&[u8]>>,
    sleep: u64,
) -> Vec<RecordBatch> {
    let mut batches: Vec<RecordBatch> = vec![];
    match sr {
        Ok(mut e) => {
            let mut res: String = "".to_owned();

            while (!e.is_finished()) {
                match e.next_batch() {
                    Ok(v) => match v {
                        Some(x) => {
                            let mut r: RecordBatch = x.into();
                            batches.push(r);
                        }
                        None => (),
                    },
                    Err(f) => panic!("Error reading record batch"),
                };
            }
            if sleep > 0 {
                println!("going to sleep for {} seconds in read_recordbatch", sleep);
                crate::sleep_int(sleep);
            }
            batches
        }
        Err(f) => panic!("Failed to get Streamreader"),
    }
}

/// A function that reads recordbatches from a memory addresss pointing to an ipc stream.
/// The we just read the batches schema into a variable that gets printed.
#[pyfunction]
fn read_recordbatch(buf_address: i64, size: usize, sleep: u64) -> PyResult<String> {
    let buf_ref = unsafe { slice::from_raw_parts(buf_address as *const u8, size) };
    let sr = StreamReader::try_new(buf_ref);
    let mut batches: Vec<RecordBatch> = read_recordbatch_from_streamreader(sr, sleep);
    Ok(batches[0].schema().to_json().to_string())
}
/// A function that reads recordbatches from a memory addresss pointing to an ipc stream.
/// The we just read the batches schema into a variable that gets printed.
#[pyfunction]
fn read_recordbatch_from_byte_stream(buf_ref: &[u8], sleep: u64) -> PyResult<String> {
    let sr = StreamReader::try_new(buf_ref);
    let mut batches: Vec<RecordBatch> = read_recordbatch_from_streamreader(sr, sleep);
    Ok(batches[0].schema().to_json().to_string())
}

/**************************************************
 * Sharing a buffer pointer
 **************************************************/
///Function that gets an adress to a DoubleArray buffer and reconstructs it
///It will faile with memory not aligned errors every seconds time
#[pyfunction]
fn read_primitivearray_from_buffer_pointer(
    index: usize,
    length: usize,
    capacity: usize,
    address: i64,
    sleep: u64,
) -> PyResult<String> {
    let mut int_arr: Float64Array;
    let buffer_ptr = address as *const u8;
    unsafe {
        /*
            Creating the buffer will only wrap the pointer in a BufferData struct
            that gets wrapped in a ARC https://doc.rust-lang.org/std/sync/struct.Arc.html
            so should be no copying involved
        */
        let b: Buffer = Buffer::from_unowned(buffer_ptr, length, capacity);
        int_arr = Float64Array::new(b.len(), b, 0, 0);
    }
    println!(
        "the Float64Array has been created in rust, going to sleep for {} sec",
        sleep
    );
    sleep_int(sleep);
    Ok(int_arr.value(index).to_string())
}

/**************************************************
 * C data interface
 **************************************************/
#[derive(Debug)]
struct BufferDefinition {
    address: i64,
    offset: usize,
}

#[repr(C)] // to get the same type of memory alignment and offset
#[derive(Debug)]
pub struct ArrowArray {
    pub length: i64,
    pub null_count: cty::int64_t,
    pub offset: cty::int64_t,
    pub n_buffers: cty::int64_t,
    pub n_children: cty::int64_t,
    pub buffers: cty::int64_t,
    pub children: cty::int64_t,
    pub dictionary: cty::int64_t,
}

/// This just to fool the alignment check in rust arrow, the check that the buffer is 128 aligned in Buffers.rs (l 181)
/// for x_86_64, from what I get they've tried to optimze for different arch when allocating, don't know why the have
/// the check as a mandatory thing when bulding from an existing ptr.
fn derive_bufferdefinition(address: i64) -> BufferDefinition {
    if (address % 128 != 0) {
        return BufferDefinition {
            address: address - 64,
            offset: 8,
        };
    }
    BufferDefinition {
        address: address,
        offset: 0,
    }
}
unsafe fn derive_bufferdefinitions(address: i64, n_buffers: usize) -> Vec<BufferDefinition> {
    let mut buffer_array: Vec<i64> =
        slice::from_raw_parts(address as *const i64, n_buffers as usize).to_vec();
    buffer_array
        .into_iter()
        .map(|x| derive_bufferdefinition(x))
        .collect()
}
#[pyfunction]
pub fn read_float64array_from_pointer(address: u64) -> PyResult<String> {
    //let g: crate::arrow_c_bind::gint64 ;
    let arrow_array_ptr = address as *const ArrowArray;

    let mut res = "".to_string();
    unsafe {
        let arrow_array = arrow_array_ptr.read();
        println!("The ArrowArray instantiated: {:#?}", arrow_array);
        //The primitive array should have 2 buffers, the null bitmap and the value buffer
        if arrow_array.n_buffers != 2 {
            panic!(
                "Wrong number of buffers for a primitive array: {}, it should be 2!",
                arrow_array.n_buffers
            );
        }
        //This a hack to handle the memory alignement check in the rust code, line 181 in Buffers.rs
        let buffer_defs =
            derive_bufferdefinitions(arrow_array.buffers, arrow_array.n_buffers as usize);

        //The first buffer should be the null bitmap and should be 0 here since it's an array without nulls
        if buffer_defs[0].address != 0 {
            panic!(
                "The null bitmap buffer doesn't have a zero address: {}",
                buffer_defs[0].address
            );
        }
        println!("The value buffer def: {:#?}", buffer_defs[1]);

        //Haven't checked the code but doesn't seem mandatory to set any values for size and capacity
        //And probably should be since we supply more or less the same info to the array object
        let buffer =
            Buffer::from_unowned(buffer_defs[1].address as *const u8, 0 as usize, 0 as usize);

        let primitiv_int32_array = Float64Array::new(
            arrow_array.length as usize,
            buffer,
            arrow_array.null_count as usize,
            (arrow_array.offset as usize) + buffer_defs[1].offset,
        );
        let mut i = 0;
        println!("printing 10 values");
        while i < 10 {
            println!("This should be a value: {}", primitiv_int32_array.value(i));
            i += 1;
        }
    }
    Ok(res)
}

/// A Python module with arrow examples.
#[pymodule]
pub fn arrowlab_from_address(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_primitivearray_from_buffer_pointer))?;
    m.add_wrapped(wrap_pyfunction!(read_float64array_from_pointer))?;
    m.add_wrapped(wrap_pyfunction!(read_recordbatch))?;
    m.add_wrapped(wrap_pyfunction!(read_recordbatch_from_byte_stream))?;
    Ok(())
}

/// A Python module with basic pyo3 examples.
#[pymodule]
fn arrowlab_basic(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(concat_string))?;
    Ok(())
}

#[pymodule]
fn arrowlab(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pymodule!(arrowlab_from_address))?;
    module.add_wrapped(wrap_pymodule!(arrowlab_basic))?;
    Ok(())
}

//use arrow::ipc::StreamReader;
//StreamReader::try_new(r )
/*
from string_sum import read_arrow
import pyarrow as pa
data = [pa.array([1, 2, 3, 4]),pa.array(['foo', 'bar', 'baz', None]),
       pa.array([True, None, False, True])]
batch = pa.record_batch(data, names=['f0', 'f1', 'f2'])
sink = pa.BufferOutputStream()
writer = pa.ipc.new_stream(sink, batch.schema)
#for i in range(5):
writer.write_batch(batch)

writer.close()

buf = sink.getvalue()

buf.size
read_arrow(buf.to_pybytes())
from string_sum import read_recordbatch
read_recordbatch(buf.to_pybytes())
read_recordbatch(buf.address, buf.size)
import pyarrow as pa
p = pa.array([1.1, 2.2, 3.3, 4.4])
from string_sum import read_primitivebuffer, read_primitivebuffer_from_pointer
#This call with the to_pybytes will cause a memory copy
read_primitivebuffer(0,p.buffers()[1].to_pybytes())
read_primitivebuffer_from_pointer(0,0,p.buffers()[1].address,5)
from string_sum import read_carray_from_pointer
*/
