import click
from python.generate_parquet_data import generate_file
from python.ipc import ipc_transfer_recordbatch, ipc_transfer_recordbatch_as_byte_stream
from python.c_data_interface import read_float64array_from_pointer,read_primitivearray_from_buffer_pointer
from arrowlab import arrowlab_basic

@click.group()
def main():
    pass

@main.command()
@click.option('-filename',type=str, default='test_data.parquet', help="generate a small sample parquet file")
def generate_data(filename):
    generate_file(filename)

@main.group()
def basic():
    pass

@basic.command(help="add two numbers and return the string")
@click.option('-a',type=int,  help="first number")
@click.option('-b',type=int, help="second number")
def sum_as_string(a, b):
    print(arrowlab_basic.sum_as_string(a,b))

@basic.command(help="concats all string arg by ,")
@click.option('-s',type=str,  help="strings to concat", multiple=True)
def concat_string(s):
    print(arrowlab_basic.concat_string(s))

@main.group()
def ipc():
    pass

@ipc.command(help="transfer an recordbatch to rust by providing a byte array pointer")
@click.option('-filename',type=str, default='test_data.parquet', help="The file to read and send to rust")
@click.option('-sleep',type=int, default=0, help="The file to read and send to rust")
def transfer_recordbatch(filename, sleep):
    ipc_transfer_recordbatch(filename, sleep)

@ipc.command(help="transfer an recordbatch to rust by providing a buffer output stream pointer")
@click.option('--filename',type=str, default='test_data.parquet', help="The file to read and send to rust")
@click.option('--sleep',type=int, default=0, help="The file to read and send to rust")
def transfer_recordbatch_as_byte_stream(filename, sleep):
    ipc_transfer_recordbatch_as_byte_stream(filename, sleep)

@main.group()
def cdata():
    pass

@cdata.command(help="Create a float64 by providing pointer to a full array spec to rust")
@click.option('--filename',type=str, default='test_data.parquet', help="The file to read and send to rust")
@click.option('--sleep',type=int, default=0, help="The file to read and send to rust")
def arrayfrompointer(filename, sleep):
    read_float64array_from_pointer(filename, sleep)

@cdata.command(help="Create a primitive array (float64) from a buffer pointer")
@click.option('--filename',type=str, default='test_data.parquet', help="The file to read and send to rust")
@click.option('--sleep',type=int, default=0, help="The file to read and send to rust")
def arrayfrombufferpointer(filename, sleep):
    read_primitivearray_from_buffer_pointer(filename=filename, sleep=sleep)