# OCR Annotation Schema Refinement

This document records schema decisions for OCR benchmark annotations.

## Current Shape

Each annotation stores:

- image file path
- screen size
- language
- notes
- ordered fragments

Each fragment stores:

- id
- text
- bounds
- block index
- line index
- fragment index

## Why Fragment-Level Annotations

Fragment-level annotations match planned OCR output normalization and selection behavior.

Benefits:

- easier comparison against OCR word boxes
- easier merge with AT-SPI fragments
- easier selection regression tests

## Possible Future Fields

- `ignored`: mark decorative or intentionally skipped text
- `confidence_hint`: optional human quality note
- `style`: font size or weight hints
- `rotation_degrees`: for rotated text samples
- `language_override`: per-fragment language

## Compatibility Rule

New fields should be additive and optional where possible.
