# makefile
# :PROPERTIES:
# :header-args: :tangle src/makefile
# :END:

# [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*makefile][makefile:1]]
make:
	rustc test.rs -o test -L. -lsample
clean:
	rm *.o -f
cc:
	clang -shared sample.c liblbfgs.so -o libsample.so

run: clean make
	LD_LIBRARY_PATH=. ./test
# makefile:1 ends here
