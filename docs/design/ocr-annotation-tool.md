# OCR Annotation Tool Plan

This document defines a future annotation workflow for building OCR benchmark samples.

## Goal

Create a small local tool that helps annotate screenshots with ordered text fragments and bounding boxes.

## Scope

The tool should:

- load a screenshot from `data/datasets/ocr_eval/images/`
- let user draw fragment bounding boxes
- let user enter fragment text
- let user assign reading order metadata
- save annotation JSON into `data/datasets/ocr_eval/annotations/`

The tool does not need to:

- run OCR automatically in first version
- support collaborative review
- support polygon annotations

## Proposed Data Model

Each annotation session should track:

- image path
- screen size
- language
- notes
- fragment list

Each fragment should track:

- id
- text
- bounds
- block index
- line index
- fragment index

## Suggested UX

1. Open image.
2. Click and drag to create box.
3. Enter text for box.
4. Repeat for all visible text fragments.
5. Reorder fragments if needed.
6. Save annotation file.

## Future Enhancements

- import OCR suggestions and edit them
- zoom and pan
- keyboard-only fragment navigation
- line grouping helpers
- validation for overlapping boxes
- side-by-side diff against OCR output

## Validation Rules

- fragment ids unique within file
- image path must exist
- bounds must stay within screen size
- fragment order fields should be contiguous where possible
- empty text fragments should be rejected unless explicitly marked ignored
