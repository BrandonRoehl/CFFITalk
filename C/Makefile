a.out: main.c ../link/* ../link.h
	$(CC) main.c -L ../link -L "$(shell xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx" \
		-l c_get_set -l c_next -l c_step -o $@

