use std::fs;
use std::path::Path;

use intelli_ocr::datasets::{AtSpiFixture, AtSpiFixtureManifest, OcrAnnotation, OcrDatasetManifest, SelectionCase, SelectionCaseManifest};

fn read_json(path: &Path) -> String {
    fs::read_to_string(path).expect("fixture file should be readable")
}

#[test]
fn loads_ocr_manifest_template() {
    let path = Path::new("data/datasets/ocr_eval/manifest.json");
    let json = read_json(path);
    let manifest: OcrDatasetManifest = serde_json::from_str(&json).expect("ocr manifest should parse");

    assert_eq!(manifest.version, 1);
    assert!(!manifest.samples.is_empty());
}

#[test]
fn loads_ocr_annotation_template() {
    let path = Path::new("data/datasets/ocr_eval/annotations/example.annotation.json");
    let json = read_json(path);
    let annotation: OcrAnnotation = serde_json::from_str(&json).expect("ocr annotation should parse");

    assert_eq!(annotation.language, "eng");
    assert!(!annotation.fragments.is_empty());
}

#[test]
fn loads_atspi_manifest_template() {
    let path = Path::new("data/datasets/atspi_eval/manifest.json");
    let json = read_json(path);
    let manifest: AtSpiFixtureManifest = serde_json::from_str(&json).expect("atspi manifest should parse");

    assert_eq!(manifest.version, 1);
    assert!(!manifest.fixtures.is_empty());
}

#[test]
fn loads_atspi_fixture_template() {
    let path = Path::new("data/datasets/atspi_eval/fixtures/example.fixture.json");
    let json = read_json(path);
    let fixture: AtSpiFixture = serde_json::from_str(&json).expect("atspi fixture should parse");

    assert_eq!(fixture.fixture_name, "example_fixture");
    assert!(!fixture.nodes.is_empty());
}

#[test]
fn loads_selection_manifest_template() {
    let path = Path::new("data/datasets/selection_eval/manifest.json");
    let json = read_json(path);
    let manifest: SelectionCaseManifest = serde_json::from_str(&json).expect("selection manifest should parse");

    assert_eq!(manifest.version, 1);
    assert!(!manifest.cases.is_empty());
}

#[test]
fn loads_selection_case_template() {
    let path = Path::new("data/datasets/selection_eval/cases/example.case.json");
    let json = read_json(path);
    let case: SelectionCase = serde_json::from_str(&json).expect("selection case should parse");

    assert_eq!(case.expected.text, "Hello world");
    assert_eq!(case.expected.fragment_ids, vec![1, 2]);
}
