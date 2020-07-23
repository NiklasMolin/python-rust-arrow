PYTHON="python3"
PIP="pip3"
build_c:
	# we disable the return-stack address since it the whole point
	gcc -c c-code/src/c_test.c -o c-code/target/c_test.o -arch x86_64 -Wno-return-stack-address
	libtool -static -o c-code/target/libc_test.a c-code/target/c_test.o -arch_only x86_64

build_cython:
	$(PIP) install Cython
	cd cython_code && $(PYTHON) setup.py build_ext --inplace

build_rust:
	cargo +nightly build --release
	cp target/release/libarrowlab.dylib arrowlab.so
	
install_c_libs:
		# Installed libs req
		brew install glib
		brew install apache-arrow-glib
		brew install apache-arrow
	
install_rust_binary:
	cargo +nightly install --path .
	cp target/release/libarrowlab.dylib arrowlab.so

install_cli:
	pipenv install --python $(PYTHON) -e .

activate_cli:
	pipenv shell