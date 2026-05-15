use std::fs;
use std::path::Path;

use intelli_ocr::datasets::SelectionCase;

fn load_case(path: &Path) -> SelectionCase {
    let json = fs::read_to_string(path).expect("selection case file should be readable");
    serde_json::from_str(&json).expect("selection case should parse")
}

#[test]
fn example_selection_case_has_non_empty_fragments() {
    let case = load_case(Path::new(
        "data/datasets/selection_eval/cases/example.case.json",
    ));
    assert!(!case.fragments.is_empty());
}

#[test]
fn example_selection_case_drag_rectangle_is_well_formed() {
    let case = load_case(Path::new(
        "data/datasets/selection_eval/cases/example.case.json",
    ));

    assert!(case.drag.start.x <= case.drag.end.x);
    assert!(case.drag.start.y <= case.drag.end.y);
}

#[test]
fn example_selection_case_expected_ids_exist_in_fragments() {
    let case = load_case(Path::new(
        "data/datasets/selection_eval/cases/example.case.json",
    ));

    for expected_id in &case.expected.fragment_ids {
        assert!(
            case.fragments
                .iter()
                .any(|fragment| &fragment.id == expected_id)
        );
    }
}
