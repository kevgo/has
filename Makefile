cuke:  # runs the feature tests
	cargo test --test cucumber

help:  # prints all make targets
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t


.SILENT:
.DEFAULT_GOAL := help
