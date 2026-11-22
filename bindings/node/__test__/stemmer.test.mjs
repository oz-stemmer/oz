// Smoke tests for the @oz/stemmer Node.js bindings.
// Uses Node 18+ built-in test runner so no dev dependency is required at test time.

import { test } from "node:test";
import assert from "node:assert/strict";

import { Stemmer } from "../index.js";

test("stems simple plural (no mutation trigger)", () => {
  const s = new Stemmer();
  assert.equal(s.stem("evler"), "ev");
  assert.equal(s.stem("arabalar"), "araba");
});

test("stemBatch matches single stem", () => {
  const s = new Stemmer();
  const words = ["evler", "arabalar", "masada"];
  const batch = s.stemBatch(words);
  assert.deepEqual(batch, words.map((w) => s.stem(w)));
});

test("stemWithConfidence returns object with valid score", () => {
  const s = new Stemmer();
  const r = s.stemWithConfidence("evler");
  assert.equal(r.stem, "ev");
  assert.ok(r.confidence >= 0 && r.confidence <= 1);
});

test("analyze returns full decomposition", () => {
  const s = new Stemmer();
  const a = s.analyze("evler");
  assert.equal(a.input, "evler");
  assert.equal(a.stem, "ev");
  assert.ok(Array.isArray(a.suffixes));
  assert.ok(a.confidence >= 0 && a.confidence <= 1);
});

test("aggressive config flag is accepted", () => {
  const s = new Stemmer({ aggressive: true });
  assert.equal(s.stem("ev"), "ev");
});

test("non-Turkish token passes through unchanged", () => {
  const s = new Stemmer();
  assert.equal(s.stem("stress"), "stress");
});
