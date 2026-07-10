/**
 * Minimal ambient declarations for the tiny slice of Node.js the tests use.
 *
 * repohelix's viewer intentionally ships with zero runtime and (near) zero dev
 * dependencies. Rather than pulling the full `@types/node` package just to run
 * a handful of assertions, we declare exactly what the test file touches.
 */

declare module "node:assert/strict" {
  interface StrictAssert {
    (value: unknown, message?: string): void;
    equal(actual: unknown, expected: unknown, message?: string): void;
    notEqual(actual: unknown, expected: unknown, message?: string): void;
    deepEqual(actual: unknown, expected: unknown, message?: string): void;
    match(value: string, regexp: RegExp, message?: string): void;
    ok(value: unknown, message?: string): void;
  }
  const assert: StrictAssert;
  export default assert;
}

declare const console: {
  log(...args: unknown[]): void;
  error(...args: unknown[]): void;
};
