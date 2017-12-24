/**
 * repohelix viewer — pure, DOM-free core.
 *
 * All the math and data-shaping logic lives here so it can be unit-tested under
 * plain Node without a browser or DOM shim. `viewer.ts` imports these helpers
 * and adds the SVG rendering on top.
 */

/** A file node as emitted by `repohelix viewer-data`. */
export interface ViewerNode {
  path: string;
  churn: number;
  commits: number;
  authors: number;
  concentration: number;
  first_seen: number;
  last_seen: number;
}

/** A co-change link referencing node indices. */
export interface ViewerLink {
  source: number;
  target: number;
  strength: number;
  together: number;
}

/** A temporal cluster (development session). */
export interface ViewerCluster {
  index: number;
  start: number;
  end: number;
  commits: number;
  churn: number;
}

/** The full payload. */
export interface ViewerData {
  schema: string;
  commits: number;
  span_days: number;
  nodes: ViewerNode[];
  links: ViewerLink[];
  clusters: ViewerCluster[];
}

/** Linear interpolation. */
export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

/** Clamp a value into [lo, hi]. */
export function clamp(x: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, x));
}

/**
 * Map a concentration value (0..1) to an rgb() colour. Low concentration
 * (shared stewardship) reads as calm teal; high concentration (single-owner /
 * bus-factor risk) reads as warm amber. This is a risk gradient, never a value
 * judgment about any contributor.
 */
export function concentrationColour(c: number): string {
  const t = clamp(c, 0, 1);
  const r = Math.round(lerp(40, 240, t));
  const g = Math.round(lerp(200, 170, t));
  const b = Math.round(lerp(180, 60, t));
  return `rgb(${r},${g},${b})`;
}

/** Scale churn to a node radius with a gentle square-root curve. */
export function churnRadius(churn: number, maxChurn: number): number {
  if (maxChurn <= 0) return 3;
  const t = Math.sqrt(churn / maxChurn);
  return lerp(3, 18, t);
}

/** Basename of a path. */
export function basename(path: string): string {
  const i = path.lastIndexOf("/");
  return i >= 0 ? path.slice(i + 1) : path;
}

/** Maximum churn across nodes (0 if empty). */
export function maxChurn(nodes: ViewerNode[]): number {
  return nodes.reduce((m, n) => Math.max(m, n.churn), 0);
