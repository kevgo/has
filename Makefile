cuke: target/debug/has  # runs the feature tests
	cargo test --test cucumber

cukethis: target/debug/has  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix:  # auto-corrects issues
	dprint fmt
	cargo fmt
	cargo fix

help:  # prints all make targets
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint: tools/actionlint  # checks formatting
	dprint check
	cargo clippy --all-targets --all-features -- -W clippy::pedantic
	cargo fmt -- --check
	git diff --check
	tools/actionlint

ps: fix test

test: lint cuke  # runs all tests

tools/actionlint:
	curl -s https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash
	mkdir -p tools
	mv actionlint tools

target/debug/has:
	cargo build

update:  # updates dependencies
	cargo upgrade


.SILENT:
.DEFAULT_GOAL := help
