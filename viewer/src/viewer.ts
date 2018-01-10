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
  const amp = Math.min(opts.width * 0.32, 220);
  const turns = 3.2;

  const strandA = el("path", { fill: "none", stroke: "#2b6f6a", "stroke-width": 2, opacity: 0.5 });
  const strandB = el("path", { fill: "none", stroke: "#7a5a2b", "stroke-width": 2, opacity: 0.5 });
  let dA = "";
  let dB = "";
  const steps = 240;
  for (let s = 0; s <= steps; s++) {
    const t = s / steps;
    const y = topPad + t * usableH;
    const angle = t * turns * Math.PI * 2;
    const xA = cx + Math.sin(angle) * amp;
    const xB = cx + Math.sin(angle + Math.PI) * amp;
    dA += `${s === 0 ? "M" : "L"}${xA.toFixed(2)},${y.toFixed(2)} `;
    dB += `${s === 0 ? "M" : "L"}${xB.toFixed(2)},${y.toFixed(2)} `;
  }
  strandA.setAttribute("d", dA);
  strandB.setAttribute("d", dB);
  svg.appendChild(strandA);
  svg.appendChild(strandB);

  order.forEach((nodeIndex, rank) => {
    const node = nodes[nodeIndex];
    const t = order.length > 1 ? rank / (order.length - 1) : 0;
    const y = topPad + t * usableH;
    const angle = t * turns * Math.PI * 2 + (rank % 2 === 0 ? 0 : Math.PI);
    const x = cx + Math.sin(angle) * amp;

    const r = churnRadius(node.churn, maxC);
    const g = el("g", { class: "helix-node", "data-index": nodeIndex });

    const dot = el("circle", {
      cx: x,
      cy: y,
      r,
      fill: concentrationColour(node.concentration),
      stroke: "#0d1b1a",
      "stroke-width": 1,
    });
    const dur = (2.4 - 1.6 * (node.churn / (maxC || 1))).toFixed(2);
    dot.appendChild(
      el("animate", {
        attributeName: "r",
        values: `${r};${(r * 1.18).toFixed(2)};${r}`,
        dur: `${dur}s`,
        repeatCount: "indefinite",
      }),
    );
    g.appendChild(dot);

    const label = el("text", {
      x: x + r + 4,
      y: y + 3,
      "font-size": 9,
      fill: "#cfe8e6",
      "font-family": "monospace",
    });
    label.textContent = basename(node.path);
    g.appendChild(label);

    const title = el("title");
    title.textContent =
