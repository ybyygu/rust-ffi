# makefile
# :PROPERTIES:
# :header-args: :tangle src/makefile
# :END:

# [[file:~/Workspace/Programming/rust-libs/rust-ffi/rust-ffi.note::*makefile][makefile:1]]
rust: cc
	# rustc test.rs -o test -L. -lsample
	rustc test.rs -o test -llbfgs
clean:
	rm *.o -f
cc:
	clang -shared sample.c liblbfgs.so -o libsample.so

run: clean rust
	LD_LIBRARY_PATH=. ./test
# makefile:1 ends here
