cuke:  # runs the feature tests
	cargo test --test cucumber

help:  # prints all make targets
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: tools/actionlint  # checks formatting
	dprint check
	cargo clippy --all-targets --all-features -- -W clippy::pedantic
	cargo fmt -- --check
	git diff --check
	tools/actionlint

test: cuke  # runs all tests

tools/actionlint:
	curl -s https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash
	mkdir -p tools
	mv actionlint tools


.SILENT:
.DEFAULT_GOAL := help
