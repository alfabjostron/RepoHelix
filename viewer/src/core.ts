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
