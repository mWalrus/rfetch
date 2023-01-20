prog :=rfetch

debug ?=

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
ifdef debug
	cp target/$(target)/$(prog) /usr/bin/$(prog)-$(extension)
	chmod 755 /usr/bin/$(prog)-($extension)

else
	cp target/$(target)/$(prog) /usr/bin/$(prog)
	chmod 755 /usr/bin/$(prog)
endif

all: build install

uninstall:
ifdef debug
	rm /usr/bin/$(prog)-$(extension)
else
	rm /usr/bin/$(prog)
endif
 
help:
	@echo "usage: make $(prog) [debug=1]"
