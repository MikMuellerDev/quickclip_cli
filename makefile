appname := quickclip-0.1.1
appdir := quickclip_cli

COLOR ?= auto
CARGO = cargo --color $(COLOR)

.PHONY: all

all: check bench build tar


tar:
	mkdir -p build
	cd ../ && tar -cvzf ./$(appname).tar.gz $(appdir)/bin $(appdir)/config && mv $(appname).tar.gz $(appdir)/build

clean:
	rm -rf bin build target

build:
	@$(CARGO) build --target-dir=bin --release
	mv ./bin/release/quickclip_cli ./
	rm -rf ./bin
	mkdir -p ./bin
	mv ./quickclip_cli ./bin

bench:
	@$(CARGO) bench

check:
	@$(CARGO) check

install:
	build
	sudo mv ./bin/release/quickclip_cli /usr/bin/quickclip

uninstall:
	sudo rm /usr/bin/quickclip

run:
	@$(CARGO) run