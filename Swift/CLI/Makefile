a.out: .build/release/cli
	ln -sf $(PWD)/$< $@

.build/release/cli: Sources/* ../../link/* ../../link.h
	rm -r .build
	swift build -c release -Xlinker -L../../link/

