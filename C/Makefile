a.out: main.c ../link/* ../link.h
	$(CC) main.c -L ../link -L "$(shell xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx" \
		-lc_get_set -lc_next -lc_step -o $@

