# makefile
# :PROPERTIES:
# :header-args: :tangle makefile
# :END:

# [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*makefile][makefile:1]]
make:
	rustc test.rs -o test -L. -lsample
clean:
	rm *.o -f
cc:
	clang -shared sample.c liblbfgs.so -o libsample.so

run:
	LD_LIBRARY_PATH=. ./test
# makefile:1 ends here
