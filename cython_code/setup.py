# This example is taken from 
# https://arrow.apache.org/docs/python/extending.html#id1

#pip3 install Cython
#python3 setup.py build_ext --inplace
# cython: languale_level=3, Unused=True 
from distutils.core import setup
from distutils.extension import Extension
from Cython.Build import cythonize
import os
import numpy as np
import pyarrow as pa

ext_modules = cythonize([Extension(name="unwrap",sources=["unwrap.pyx", "pcffi.py"])])



for ext in ext_modules:
    # The Numpy C headers are currently required
    ext.include_dirs.append(np.get_include())
    ext.include_dirs.append(pa.get_include())
    ext.libraries.extend(pa.get_libraries())
    ext.library_dirs.extend(pa.get_library_dirs())

    if os.name == 'posix':
        ext.extra_compile_args.append('-std=c++11')

setup(ext_modules=ext_modules)