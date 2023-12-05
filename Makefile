# dev tooling and versions
RUN_THAT_APP_VERSION = 0.2.0


cuke: target/debug/has  # runs the feature tests
	cargo test --test cucumber

cukethis: target/debug/has  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix: tools/run-that-app@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	tools/rta dprint fmt
	cargo fmt
	cargo fix

help:  # prints all make targets
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT' | grep -v help | grep -v '^tools\/' | grep -v '^target/debug' | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint: tools/run-that-app@${RUN_THAT_APP_VERSION}  # checks formatting
	tools/rta dprint check
	tools/rta actionlint
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo fmt -- --check
	git diff --check

ps: fix test  # pitstop

test: lint unit cuke  # runs all tests

.PHONY: target/debug/has
target/debug/has:
	cargo build

unit:  # runs the unit tests
	cargo test

update: tools/run-that-app@${RUN_THAT_APP_VERSION}  # updates dependencies
	cargo upgrade
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/run-that-app@${RUN_THAT_APP_VERSION}:
	@rm -f tools/run-that-app* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/run-that-app tools/run-that-app@${RUN_THAT_APP_VERSION}
	@ln -s run-that-app@${RUN_THAT_APP_VERSION} tools/rta


.SILENT:
.DEFAULT_GOAL := help
