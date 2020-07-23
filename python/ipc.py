import pandas as pd
import time
import pyarrow as pa
import pyarrow.parquet as pq
from arrowlab import arrowlab_from_address
from memory_profiler import profile

def print_sleep(sleep, message):
    if sleep:
        print(message)
        time.sleep(sleep)

def read_and_get_buffer(file_name, sleep):
    """ This function will read the parquet files, turn it into RecordBatches and 
        then write tohose to a BufferStream that is returned 
    """
    a = pq.read_table(file_name)
    print("read data, we have now instatiated the data once")
    batches = a.to_batches(max_chunksize=9999999999) 

    print(f"created {len(batches)} batches, second time we instatiate the data")
    sink = pa.BufferOutputStream()
    writer = pa.ipc.new_stream(sink, batches[0].schema)
    print_sleep(sleep,f"waiting {sleep} seconds before batches")
    for batch in batches:
        writer.write_batch(batch)
    print("wrote batches, third instanciation")
    writer.close()
    print_sleep(sleep,f"waiting {sleep} seconds after batches")
    return sink.getvalue() ,a ,batches

def ipc_transfer_recordbatch(file_name, sleep = 0):
    buf, _ , _1 = read_and_get_buffer(file_name, sleep)
    print_sleep(sleep,f"waiting {sleep} seconds before calling the rust function")
    print('calling rust')
    res = arrowlab_from_address.read_recordbatch(buf.address, buf.size,sleep)
    print(f"the result from the rust call: {res}")
    print_sleep(sleep,f"waiting {sleep} seconds before finishing")

def ipc_transfer_recordbatch_as_byte_stream(file_name, sleep = 0):
    buf, _ , _1 = read_and_get_buffer(file_name, sleep)
    print_sleep(sleep, f"wating {sleep} sec before creating converting buf to python bytes object")    
    py_bytes = buf.to_pybytes()
    print("converted batches to a python bytes object, fourth instanciation")
    print_sleep(sleep, f"waiting {sleep} seconds before calling the rust function")
    print('calling rust')
    res = arrowlab_from_address.read_recordbatch_from_byte_stream(py_bytes ,sleep)
    print(f"the result from the rust call: {res}")
    print_sleep(sleep, f"waiting {sleep} seconds before finishing")


