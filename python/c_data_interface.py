import pandas as pd
import time
import pyarrow as pa
import pyarrow.parquet as pq
from arrowlab import arrowlab_from_address
from memory_profiler import profile
from cython_code.unwrap import get_array

def print_sleep(sleep, message):
    if sleep:
        print(message)
        time.sleep(sleep)

def read_and_get_batches(file_name, sleep):
    """ This function will read the parquet files, turn it into RecordBatches and 
        then write tohose to a BufferStream that is returned 
    """
    a = pd.read_parquet('test_data.parquet')
    print("read data, we have now instatiated the data once")
    print_sleep(sleep,f"waiting {sleep} seconds after reading file")
    p = batches = pa.RecordBatch.from_pandas(a)
    print(f"created {len(batches)} batches, second time we instatiate the data")
    print("Each batch corresponds to a column, we return the third column") 
    return p, a

def read_primitivearray_from_buffer_pointer(filename, sleep = 0):
    p, _ = read_and_get_batches(filename, sleep)
    print_sleep(sleep, f"sleeping for {sleep} seconds")

    print("""We now that this is an doublearray so it has two buffers, 
    the first is the schema (kind of known, it's s double) we will only look at
    the data buffer""")
    print(f"buffer size {p[2].buffers()[1].size}")
    print("going to rust")
    e = arrowlab_from_address.read_primitivearray_from_buffer_pointer(0,0,0,p[2].buffers()[1].address,sleep)
    print("done with rust")
    print_sleep(sleep,f"sleeping for {sleep} seconds")
    print(f"the result from the rust call: {e}")
    print_sleep(sleep,f"waiting {sleep} seconds before finishing")

def read_float64array_from_pointer(filename, sleep = 0):
    p, _ = read_and_get_batches(filename, sleep)
    print_sleep(sleep, f"sleeping for {sleep} seconds")
    print("going to rust")
    e = arrowlab_from_address.read_float64array_from_pointer(get_array(p[2]))
    print("done with rust")
    print_sleep(sleep, f"sleeping for {sleep} seconds")
    print(f"the result from the rust call: {e}")
    print_sleep(sleep,f"waiting {sleep} seconds before finishing")
 



