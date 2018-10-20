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
