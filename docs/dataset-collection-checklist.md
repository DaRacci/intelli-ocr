# Dataset Collection Checklist

Use this checklist when adding real samples.

## OCR Sample Checklist

- [ ] Screenshot captured at native resolution
- [ ] File saved under `data/datasets/ocr_eval/images/`
- [ ] Matching annotation file created
- [ ] Visible target text manually transcribed
- [ ] Fragment order reviewed
- [ ] Bounding boxes reviewed
- [ ] Tags added to manifest
- [ ] Difficulties noted in annotation notes

## AT-SPI Fixture Checklist

- [ ] Application type recorded
- [ ] Window title recorded
- [ ] Snapshot normalized into fixture JSON
- [ ] Unstable fields removed
- [ ] Bounds verified in screen coordinates
- [ ] Nested children preserved where needed
- [ ] Fixture added to manifest

## Selection Case Checklist

- [ ] Fragment list minimal and focused
- [ ] Drag rectangle defined
- [ ] Expected fragment ids defined
- [ ] Expected copied text defined
- [ ] Case added to manifest

## Review Checklist

- [ ] File names use stable sample ids
- [ ] No private or sensitive content included
- [ ] No generated files mixed with hand-authored annotations
- [ ] Notes explain why sample exists
