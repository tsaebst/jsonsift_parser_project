.PHONY: help all decode convert inspect test fmt clippy proj_info clean

FILE ?= test.json
OUT  ?= result.csv

all: test

help:
	@echo "Makefile commands:"
	@echo "  make decode FILE=<input.json> OUT=<output.csv>  - Decode & save CSV to file"
	@echo "  make proj_info                                  - Show project general info"
	@echo "  make test                                       - Run tests"
	@echo "  make fmt                                        - Format the code via rustfmt"
	@echo "  make clippy                                     - Run clippy linter on the code"
	@echo "  make clean                                      - Clean build artifacts"

proj_info:
	@echo "Project general info:"
	@cargo read-manifest | jq -r '"Name: \(.name)\nVersion: \(.version)\nEdition: \(.edition)\nDescription: \(.description)\nLicense: \(.license)\nAuthor(s): \(.authors | join(", "))"'

# save+decode
decode:
	@echo "→ Decoding & saving: $(FILE) -> $(OUT)"
	@[ -n "$(dir $(OUT))" ] && mkdir -p "$(dir $(OUT))" || true
	@rm -f "$(OUT)"
	cargo run --bin json_sift_parser -- decode "$(FILE)" --output "$(OUT)"
	@echo "→ Written:" && ls -l "$(OUT)" || true


test:
	cargo test

fmt:
	@echo "→ Formatting code..."
	cargo fmt --all

clippy:
	@echo "→ Running clippy..."
	cargo clippy -- -D warnings

clean:
	cargo clean
	@echo "→ Cleaned target/"
