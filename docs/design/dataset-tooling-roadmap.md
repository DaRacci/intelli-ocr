# Dataset Tooling Roadmap

This roadmap covers tooling needed to turn dataset skeletons into useful regression assets.

## Stage 1

- JSON schema types in Rust
- fixture loading tests
- manifest templates
- collection checklist

## Stage 2

- AT-SPI fixture dumper CLI
- OCR annotation helper
- selection case validator

## Stage 3

- OCR benchmark runner
- AT-SPI parser regression tests
- merge strategy regression tests
- selection strategy regression tests

## Stage 4

- preprocessing comparison runner
- multilingual benchmark sets
- difficult sample tagging and reporting

## Success Criteria

- new sample can be added with predictable file layout
- sample can be parsed by tests
- regressions can be caught without live compositor state
