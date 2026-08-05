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

test("lerp interpolates", () => {
  assert.equal(lerp(0, 10, 0.5), 5);
  assert.equal(lerp(2, 4, 0), 2);
});

test("clamp bounds values", () => {
  assert.equal(clamp(-1, 0, 1), 0);
  assert.equal(clamp(2, 0, 1), 1);
  assert.equal(clamp(0.4, 0, 1), 0.4);
});

test("concentrationColour endpoints differ", () => {
  const low = concentrationColour(0);
  const high = concentrationColour(1);
  assert.notEqual(low, high);
  assert.match(low, /^rgb\(\d+,\d+,\d+\)$/);
});

test("churnRadius grows with churn and handles zero max", () => {
  assert.equal(churnRadius(10, 0), 3);
  const small = churnRadius(10, 100);
  const big = churnRadius(100, 100);
  assert.ok(big > small);
  assert.ok(big <= 18.0001);
});

test("basename strips directories", () => {
  assert.equal(basename("src/models/user.rs"), "user.rs");
  assert.equal(basename("README.md"), "README.md");
});

test("maxChurn finds the peak", () => {
  assert.equal(maxChurn(sample().nodes), 100);
  assert.equal(maxChurn([]), 0);
});

test("helixOrder sorts by first_seen then path", () => {
  const order = helixOrder(sample().nodes);
  // c.rs first_seen 50, a.rs 100, b.rs 200 -> indices [2, 0, 1]
  assert.deepEqual(order, [2, 0, 1]);
});

test("ringPoint places first slot at top", () => {
  const p = ringPoint(0, 4, 100, 100, 50);
  // angle -PI/2 -> straight up: x=100, y=50.
  assert.ok(Math.abs(p.x - 100) < 1e-9);
  assert.ok(Math.abs(p.y - 50) < 1e-9);
});

test("linksAreValid accepts good data and rejects bad", () => {
  assert.equal(linksAreValid(sample()), true);
  const bad = sample();
  bad.links = [{ source: 0, target: 99, strength: 1, together: 1 }];
  assert.equal(linksAreValid(bad), false);
  const selfLink = sample();
  selfLink.links = [{ source: 1, target: 1, strength: 1, together: 1 }];
  assert.equal(linksAreValid(selfLink), false);
});

test("summaryText includes headline metrics", () => {
  const s = summaryText(sample());
  assert.match(s, /commits: 5/);
  assert.match(s, /files: 3/);
  assert.match(s, /hotspot: a\.rs/);
  assert.match(s, /top coupling: a\.rs↔b\.rs/);
});

console.log(`\n${passed} tests passed`);
