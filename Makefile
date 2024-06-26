# dev tooling and versions
RUN_THAT_APP_VERSION = 0.5.0


cuke: target/debug/has  # runs the feature tests
	cargo test --test cucumber

cukethis: target/debug/has  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix: tools/rta@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	tools/rta dprint fmt
	cargo +nightly fmt
	cargo +nightly fix --allow-dirty

help:  # prints all make targets
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT' | grep -v help | grep -v '^tools\/' | grep -v '^target/debug' | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint: tools/rta@${RUN_THAT_APP_VERSION}  # checks formatting
	tools/rta dprint check
	tools/rta actionlint
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo +nightly fmt -- --check
	git diff --check

ps: fix test  # pitstop

setup:  # install development dependencies on this computer
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly

test: lint unit cuke  # runs all tests

.PHONY: target/debug/has
target/debug/has:
	cargo build

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates dependencies
	cargo install cargo-edit cargo-machete
	cargo machete
	cargo upgrade
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta


.SILENT:
.DEFAULT_GOAL := help
