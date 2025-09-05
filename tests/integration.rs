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

#[test]
fn handlers_router_are_most_coupled() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    let top = &analysis.co_changes[0];
    let pair = {
        let mut v = [top.a.as_str(), top.b.as_str()];
        v.sort_unstable();
        v
    };
    assert_eq!(pair, ["src/handlers.rs", "src/router.rs"]);
    assert!(top.strength > 0.5);
}

#[test]
fn four_temporal_clusters() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    // The fixture is built as four sessions separated by multi-day gaps.
    assert_eq!(analysis.clusters.len(), 4);
    let total: u64 = analysis.clusters.iter().map(|c| c.commits).sum();
    assert_eq!(total, 24);
}

#[test]
fn single_author_files_have_max_concentration() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    let migrate = analysis
        .ownership
        .iter()
        .find(|o| o.path == "src/migrate.rs")
        .unwrap();
    assert_eq!(migrate.authors, 1);
    assert!((migrate.concentration - 1.0).abs() < 1e-9);
}

#[test]
fn output_is_deterministic() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let a1 = analyze(&history, &Params::default());
    let a2 = analyze(&history, &Params::default());
    assert_eq!(report::to_json(&a1, true), report::to_json(&a2, true));
    assert_eq!(report::to_text(&a1), report::to_text(&a2));
    assert_eq!(
        viewer::build(&a1, 64).to_compact(),
        viewer::build(&a2, 64).to_compact()
    );
}

#[test]
fn json_report_declares_it_is_not_a_personnel_judgment() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    let json = report::to_json(&analysis, false);
    assert!(json.contains("not"));
    assert!(json.contains("personnel judgments") || json.contains("not measures"));
}

#[test]
fn max_commits_limits_history() {
    let history = load_from_log(fixture_path(), Some(4)).unwrap();
    assert_eq!(history.commit_count(), 4);
}

#[test]
fn viewer_links_have_valid_node_indices() {
    let history = load_from_log(fixture_path(), None).unwrap();
    let analysis = analyze(&history, &Params::default());
    let node_count = analysis.file_stats.len().min(64);
    let payload = viewer::build(&analysis, 64).to_compact();
    // Every "source"/"target" index must be < node_count. We spot-check by
    // parsing the compact JSON for the numbers after those keys.
    for key in ["\"source\":", "\"target\":"] {
        let mut idx = 0;
        while let Some(pos) = payload[idx..].find(key) {
            let start = idx + pos + key.len();
            let rest = &payload[start..];
            let end = rest.find([',', '}']).unwrap();
            let num: usize = rest[..end].parse().unwrap();
            assert!(num < node_count, "index {num} out of range");
            idx = start + end;
        }
    }
}
