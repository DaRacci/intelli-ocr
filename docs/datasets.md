# Dataset and Test Data Plan

This document defines dataset layout for OCR evaluation, AT-SPI fixture capture, and selection behavior tests.

## Goals

- Keep all evaluation data local to repository structure.
- Separate raw captures from normalized annotations.
- Make OCR, AT-SPI, and selection logic testable without live compositor state.
- Allow later expansion into benchmark scripts and regression tests.

## Directory Layout

- `data/datasets/ocr_eval/images/`
  - Raw screenshots used for OCR evaluation.
- `data/datasets/ocr_eval/annotations/`
  - Ground truth text and bounding boxes for each OCR image.
- `data/datasets/atspi_eval/fixtures/`
  - Serialized AT-SPI snapshots captured from real applications.
- `data/datasets/selection_eval/cases/`
  - Synthetic fragment lists and expected selection outputs.

## OCR Evaluation Dataset

Each OCR sample should have:

1. Image file in `images/`
2. Annotation file in `annotations/`
3. Matching stem name

Example:

- `images/terminal_dark_001.png`
- `annotations/terminal_dark_001.json`

Annotation schema fields:

- `image_file`: relative image path
- `screen_size`: width and height
- `language`: expected language code
- `fragments`: ordered list of expected text fragments
- `notes`: optional capture notes

Each fragment should include:

- `id`
- `text`
- `bounds`
- `block_index`
- `line_index`
- `fragment_index`

## AT-SPI Fixture Dataset

Each fixture should represent one application state snapshot.

Recommended capture targets:

- GTK app with selectable text
- Qt app with selectable text
- Electron app with selectable text
- Terminal emulator
- Browser page with mixed headings and paragraphs

Fixture schema fields:

- `fixture_name`
- `application`
- `window_title`
- `captured_at`
- `nodes`

Each node should include:

- `role`
- `name`
- `text`
- `bounds`
- `children`

## Selection Evaluation Dataset

Each case should define:

- input fragments
- drag start point
- drag end point
- expected selected fragment ids
- expected reconstructed text

This dataset should be synthetic and hand-authored first.

## Collection Workflow

### OCR samples

1. Capture screenshot from real desktop state.
2. Save original image without preprocessing.
3. Manually transcribe visible target text.
4. Record approximate or exact bounding boxes.
5. Review annotation for reading order correctness.

### AT-SPI fixtures

1. Open target application state.
2. Inspect accessibility tree.
3. Export normalized snapshot into fixture JSON.
4. Remove unstable fields.
5. Keep only fields needed by parser and merger.

### Selection cases

1. Build synthetic fragment list.
2. Define drag rectangle.
3. Define expected selected ids.
4. Define expected copied text.

## Versioning Rules

- Keep raw images immutable after commit.
- If annotation changes, update only annotation file.
- Add notes when sample is intentionally difficult.
- Prefer many small focused samples over few giant samples.

## Initial Coverage Targets

### OCR

- Light theme paragraph text
- Dark theme paragraph text
- Terminal text
- Mixed font sizes
- Buttons and labels near text
- Dense UI with sidebars
- Japanese sample later

### AT-SPI

- GTK text view
- Qt text view
- Electron content area
- Browser article page

### Selection

- Single fragment drag
- Multi-fragment same line drag
- Multi-line drag
- Partial overlap drag
- Empty selection drag

## Tooling References

- OCR annotation workflow: `docs/design/ocr-annotation-tool.md`
- OCR annotation schema notes: `docs/design/ocr-annotation-schema.md`
- AT-SPI fixture dumper notes: `docs/design/atspi-fixture-dumper.md`
- Dataset tooling roadmap: `docs/design/dataset-tooling-roadmap.md`

## Future Extensions

- Add preprocessing comparison dataset.
- Add rotated text samples.
- Add multilingual OCR dataset.
- Add false-positive rejection dataset.
- Add privacy-sensitive exclusion fixtures.

