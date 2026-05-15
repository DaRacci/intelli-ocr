# AT-SPI Fixture Dumper Plan

This document defines a future helper for exporting normalized AT-SPI snapshots.

## Goal

Capture enough accessibility tree data from a live application to test parsing and merge logic offline.

## Intended Inputs

- application name filter
- window title filter
- output fixture path

## Intended Output

JSON file matching `AtSpiFixture` schema under `data/datasets/atspi_eval/fixtures/`.

## Normalization Rules

- keep only stable fields needed by parser
- remove transient ids and process-specific values
- preserve tree structure
- preserve text and bounds where available
- prefer screen coordinates for bounds

## Suggested Workflow

1. Connect to AT-SPI bus.
2. Enumerate desktop applications.
3. Filter target application/window.
4. Walk subtree.
5. Normalize nodes.
6. Write fixture JSON.

## Future Enhancements

- dump multiple windows in one run
- include optional raw debug payload
- include role filtering
- include text interface metadata
