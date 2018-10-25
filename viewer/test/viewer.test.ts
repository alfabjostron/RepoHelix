/**
 * Unit tests for the DOM-free viewer core. Run with plain Node — no test
 * framework, no browser. Uses the built-in `assert` module only.
 */

import assert from "node:assert/strict";
import {
  basename,
  churnRadius,
  clamp,
  concentrationColour,
  helixOrder,
  lerp,
  linksAreValid,
  maxChurn,
  ringPoint,
  summaryText,
  type ViewerData,
} from "../src/core.js";

let passed = 0;
function test(name: string, fn: () => void): void {
  fn();
  passed += 1;
  console.log(`ok - ${name}`);
}

function sample(): ViewerData {
  return {
    schema: "repohelix/viewer/v1",
    commits: 5,
    span_days: 12,
    nodes: [
      { path: "src/a.rs", churn: 100, commits: 5, authors: 3, concentration: 0.1, first_seen: 100, last_seen: 500 },
      { path: "src/b.rs", churn: 40, commits: 2, authors: 1, concentration: 1.0, first_seen: 200, last_seen: 300 },
      { path: "src/c.rs", churn: 0, commits: 1, authors: 1, concentration: 1.0, first_seen: 50, last_seen: 50 },
    ],
    links: [{ source: 0, target: 1, strength: 0.5, together: 3 }],
    clusters: [{ index: 0, start: 50, end: 500, commits: 5, churn: 140 }],
  };
}
