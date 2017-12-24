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
