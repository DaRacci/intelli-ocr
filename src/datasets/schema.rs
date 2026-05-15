use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetScreenSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrDatasetManifest {
    pub version: u32,
    pub description: String,
    pub samples: Vec<OcrDatasetSample>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrDatasetSample {
    pub id: String,
    pub image: String,
    pub annotation: String,
    pub tags: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrAnnotation {
    pub image_file: String,
    pub screen_size: DatasetScreenSize,
    pub language: String,
    pub notes: String,
    pub fragments: Vec<OcrFragmentAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrFragmentAnnotation {
    pub id: u64,
    pub text: String,
    pub bounds: DatasetBounds,
    pub block_index: u32,
    pub line_index: u32,
    pub fragment_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtSpiFixtureManifest {
    pub version: u32,
    pub description: String,
    pub fixtures: Vec<AtSpiFixtureEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtSpiFixtureEntry {
    pub id: String,
    pub fixture: String,
    pub tags: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtSpiFixture {
    pub fixture_name: String,
    pub application: String,
    pub window_title: String,
    pub captured_at: String,
    pub nodes: Vec<AtSpiNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtSpiNode {
    pub role: String,
    pub name: String,
    pub text: Option<String>,
    pub bounds: DatasetBounds,
    pub children: Vec<AtSpiNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCaseManifest {
    pub version: u32,
    pub description: String,
    pub cases: Vec<SelectionCaseEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCaseEntry {
    pub id: String,
    pub case: String,
    pub tags: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCase {
    pub name: String,
    pub fragments: Vec<SelectionCaseFragment>,
    pub drag: SelectionCasePointerDrag,
    pub expected: SelectionCaseExpected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCaseFragment {
    pub id: u64,
    pub text: String,
    pub bounds: DatasetBounds,
    pub source: String,
    pub confidence: Option<f32>,
    pub block_index: Option<u32>,
    pub line_index: Option<u32>,
    pub fragment_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCasePointerDrag {
    pub start: DatasetPoint,
    pub end: DatasetPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionCaseExpected {
    pub fragment_ids: Vec<u64>,
    pub text: String,
}
