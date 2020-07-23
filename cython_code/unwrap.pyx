# distutils: language=c++

from pyarrow.lib cimport *
from cython_code.pcffi import ffi

from cython.operator import address

def get_array_pointer(obj):
    # Just an example function accessing both the pyarrow Cython API
    # and the Arrow C++ API

    cdef shared_ptr[CArray] arr = pyarrow_unwrap_array(obj)
    if arr.get() == NULL:
        raise TypeError("not an array")
    addresss = <unsigned long>arr.get()
    print(addresss)
    cdef CArray * newarr = <CArray *> addresss
    print(newarr.length())
    print(<unsigned long> newarr)
    return pyarrow_wrap_array(arr), <unsigned long> addresss

def get_array(obj):
    c_schema = ffi.new("struct ArrowSchema*")
    ptr_schema = int(ffi.cast("uintptr_t", c_schema))
    c_array = ffi.new("struct ArrowArray*")
    ptr_array = int(ffi.cast("uintptr_t", c_array))
    obj._export_to_c(ptr_array, ptr_schema)
    return ptr_array

def get_array_from_address(address_obj):
    cdef long a = <unsigned long> address_obj 
    print(a)
    cdef CArray * newarr = <CArray *> a
    print(*newarr.length())

def get_array_length(obj):
    # An example function accessing both the pyarrow Cython API
    # and the Arrow C++ API
    cdef shared_ptr[CArray] arr = pyarrow_unwrap_array(obj)
    if arr.get() == NULL:
        raise TypeError("not an array")
    return arr.get().length()