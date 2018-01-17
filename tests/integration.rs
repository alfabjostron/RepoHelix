//! End-to-end integration tests exercising the full pipeline against the
//! committed fixture log. These verify that parsing, analysis, and rendering
//! agree with the hand-computed expectations for the synthetic `nebula`
//! history, and that output is deterministic.

use std::path::Path;

use repohelix::metrics::{analyze, Params};
use repohelix::{load_from_log, report, viewer};

fn fixture_path() -> &'static Path {
    Path::new("fixtures/nebula.gitlog")
}

#[test]
fn fixture_loads_expected_shape() {
    let history = load_from_log(fixture_path(), None).expect("fixture should load");
    assert_eq!(history.commit_count(), 24, "fixture has 24 commits");
    // Four distinct author identities.
    let analysis = analyze(&history, &Params::default());
    assert_eq!(analysis.summary.authors, 4);
    assert_eq!(analysis.summary.files, 18);
}

#[test]
fn handlers_is_the_top_hotspot() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    assert_eq!(analysis.hotspots[0].path, "src/handlers.rs");
    // Its normalized score is the maximum, so exactly 1.0.
    assert!((analysis.hotspots[0].score - 1.0).abs() < 1e-9);
}

