prog :=rfetch

debug ?=

ifdef debug
  release :=
  target :=debug
else
  release :=--release
  target :=release
endif

build:
	cargo build $(release)

install:
ifdef debug
	cp target/$(target)/$(prog) /usr/bin/$(prog)-debug
	chmod 755 /usr/bin/$(prog)-debug
else
	cp target/$(target)/$(prog) /usr/bin/$(prog)
	chmod 755 /usr/bin/$(prog)
endif

all: build install

uninstall:
ifdef debug
	rm /usr/bin/$(prog)-debug
else
	rm /usr/bin/$(prog)
endif
 
help:
	@echo "usage: make $(prog) [debug=1]"
