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
