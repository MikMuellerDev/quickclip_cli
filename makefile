appname := quickclip-0.2.0
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
	@$(CARGO) build --release
	rm -rf ./bin
	mkdir -p ./bin
	mv ./target/release/quickclip_cli ./bin
	# mv ./quickclip_cli ./bin

bench:
	@$(CARGO) bench

check:
	@$(CARGO) check

install: build
	chmod +x ./bin/quickclip_cli
	sudo mv ./bin/quickclip_cli /usr/bin/quick-clip

uninstall:
	sudo rm /usr/bin/quickclip

run:
	@$(CARGO) run