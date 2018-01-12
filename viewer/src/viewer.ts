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
      `${node.path}\nchurn ${node.churn} · commits ${node.commits} · ` +
      `authors ${node.authors} · concentration ${node.concentration.toFixed(2)}`;
    g.appendChild(title);

    svg.appendChild(g);
  });
}

/** Render the radial co-change atlas. */
export function renderAtlas(
  svg: SVGElement,
  data: ViewerData,
  opts: { width: number; height: number },
): void {
  while (svg.firstChild) svg.removeChild(svg.firstChild);
  svg.setAttribute("viewBox", `0 0 ${opts.width} ${opts.height}`);

  const nodes = data.nodes;
  const n = nodes.length;
  if (n === 0) return;

  const cx = opts.width / 2;
  const cy = opts.height / 2;
  const radius = Math.min(cx, cy) - 60;
  const maxC = maxChurn(nodes);

  const ring = nodes.map((_, i) => ringPoint(i, n, cx, cy, radius));

  const maxStrength = data.links.reduce((m, l) => Math.max(m, l.strength), 0) || 1;
  data.links.forEach((link, li) => {
    const a = ring[link.source];
    const b = ring[link.target];
    if (!a || !b) return;
    const rel = link.strength / maxStrength;
    const bow = lerp(0.85, 0.2, rel);
    const mx = lerp((a.x + b.x) / 2, cx, bow);
    const my = lerp((a.y + b.y) / 2, cy, bow);
    const path = el("path", {
      d: `M${a.x.toFixed(2)},${a.y.toFixed(2)} Q${mx.toFixed(2)},${my.toFixed(2)} ${b.x.toFixed(2)},${b.y.toFixed(2)}`,
      fill: "none",
      stroke: concentrationColour(0.3 + 0.5 * rel),
      "stroke-width": (0.6 + 4 * rel).toFixed(2),
      opacity: 0,
    });
    path.appendChild(
      el("animate", {
        attributeName: "opacity",
        values: `0;${(0.25 + 0.6 * rel).toFixed(2)}`,
        dur: "0.8s",
        begin: `${(li * 0.06).toFixed(2)}s`,
        fill: "freeze",
      }),
    );
    const title = el("title");
    title.textContent =
      `${nodes[link.source].path} ↔ ${nodes[link.target].path}\n` +
      `strength ${link.strength.toFixed(2)} · together ${link.together}`;
    path.appendChild(title);
    svg.appendChild(path);
  });

  ring.forEach((p, i) => {
    const node = nodes[i];
    const r = churnRadius(node.churn, maxC) * 0.8 + 2;
    const dot = el("circle", {
      cx: p.x,
      cy: p.y,
      r,
      fill: concentrationColour(node.concentration),
      stroke: "#0d1b1a",
      "stroke-width": 1,
    });
    const title = el("title");
    title.textContent = `${node.path}\nchurn ${node.churn} · commits ${node.commits}`;
    dot.appendChild(title);
    svg.appendChild(dot);

    if (node.churn >= maxC * 0.15) {
      const outward = 14;
      const lx = cx + Math.cos(p.angle) * (radius + outward);
      const ly = cy + Math.sin(p.angle) * (radius + outward);
      const anchor =
        Math.cos(p.angle) < -0.1 ? "end" : Math.cos(p.angle) > 0.1 ? "start" : "middle";
      const label = el("text", {
        x: lx,
        y: ly,
        "font-size": 9,
        fill: "#cfe8e6",
        "font-family": "monospace",
        "text-anchor": anchor,
      });
      label.textContent = basename(node.path);
      svg.appendChild(label);
    }
  });
