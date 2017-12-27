/**
 * repohelix viewer — dependency-free, interactive helix / co-change atlas.
 *
 * Renders the compact JSON produced by `repohelix viewer-data`. Uses nothing
 * but the DOM and SVG: no framework, no bundler, no network access. Pure
 * computation lives in `core.ts`; this file adds SVG rendering on top.
 *
 * Two coordinated views:
 *   1. HELIX — files along a double-helix strand, ordered by first appearance;
 *      radius encodes churn, colour encodes ownership concentration.
 *   2. ATLAS — a radial co-change atlas: files on a ring, chords weighted by
 *      temporal-coupling strength.
 *
 * Reminder shown in the UI: these metrics describe files and change activity,
 * never people.
 */

import {
  basename,
  churnRadius,
  concentrationColour,
  helixOrder,
  lerp,
  maxChurn,
  ringPoint,
  summaryText,
  type ViewerData,
} from "./core.js";

const SVG_NS = "http://www.w3.org/2000/svg";

/** Create an SVG element with attributes in one call. */
function el(name: string, attrs: Record<string, string | number> = {}): SVGElement {
  const node = document.createElementNS(SVG_NS, name);
  for (const [k, v] of Object.entries(attrs)) {
    node.setAttribute(k, String(v));
  }
  return node;
}

/** Render the double-helix strand view into the given SVG element. */
export function renderHelix(
  svg: SVGElement,
  data: ViewerData,
  opts: { width: number; height: number },
): void {
  while (svg.firstChild) svg.removeChild(svg.firstChild);
  svg.setAttribute("viewBox", `0 0 ${opts.width} ${opts.height}`);

  const nodes = data.nodes;
  const order = helixOrder(nodes);
  const maxC = maxChurn(nodes);

  const cx = opts.width / 2;
  const topPad = 40;
  const usableH = opts.height - topPad - 40;
