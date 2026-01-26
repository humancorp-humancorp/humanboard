# Production Readiness Audit - Humanboard

Based on matklad's Rust best practices. Generated from comprehensive codebase analysis.

---

## CRITICAL (Must Fix Before Production)

### C1. ~~Invalid Rust Edition~~ - VERIFIED OK
Edition 2024 is valid (stabilized in Rust 1.85, project uses 1.89).

### C2. Disable Incremental Compilation in CI
**File:** `.cargo/config.toml`, `.github/workflows/release.yml`
**Issue:** `CARGO_INCREMENTAL = "1"` set globally, including CI
**Fix:** Add `CARGO_INCREMENTAL: "0"` override in CI workflow
**Impact:** Cleaner CI builds, better cache behavior

### C3. Consolidate Test Binaries (3 → 1)
**Files:** `tests/board_tests.rs`, `tests/integration.rs`, `tests/unit.rs`
**Issue:** Each root test file compiles as separate binary (3x linking overhead)
**Fix:** Consolidate into `tests/it/main.rs` with module structure
**Impact:** 66% reduction in test link time, ~24MB smaller test artifacts

---

## HIGH PRIORITY (Week 1-2)

### H1. Remove/Verify Unused Dependencies
**File:** `Cargo.toml`
**Candidates:**
- `reqwest` - only placeholder comment found in code
- `log` - appears unused (tracing handles logging)
- Review: `lofty` (1 file), `rstar` (1 usage), `pulldown-cmark` (3 uses)
**Impact:** Faster builds, smaller binary

### H2. Audit Polars Necessity
**File:** `Cargo.toml`, `src/data/`
**Issue:** Polars adds ~200+ transitive deps, heavy proc macros
**Options:**
- If only CSV/JSON parsing needed: replace with `csv` + `serde_json`
- If lazy eval needed: keep but document why
**Impact:** Potentially 30-60 seconds off clean build

### H3. Eliminate thread::sleep() in Tests
**Files:** `tests/unit/settings_watcher_tests.rs`, `tests/unit/perf_tests.rs`, `tests/unit/background_tests.rs`
**Issue:** 38 I/O instances, ~1.8 seconds of sleep overhead
**Fix:** Use mock clocks, synchronous channels
**Impact:** 50% faster test suite

### H4. Add Doctest Configuration
**File:** `Cargo.toml`
**Issue:** No `doctest = false` despite all examples being `/// ```ignore`
**Fix:** Add `[lib] doctest = false`
**Impact:** Minor CI speedup

### H5. Consolidate Duplicate Dependency Versions
**File:** `Cargo.toml`
**Duplicates found:**
- `syn` 1.0 + 2.0 (both compiled)
- `strum_macros` 0.26 + 0.27
- `thiserror-impl` 1.0 + 2.0
- `async-channel` 1.9 + 2.5
**Fix:** Add version overrides in `[patch]` or update deps
**Impact:** Faster compilation

---

## MEDIUM PRIORITY (Week 3-4)

### M1. Extract Test Helpers/Builders
**Files:** `tests/board_tests.rs`, all test files
**Issue:** Repetitive item setup, manual state construction (~200 lines boilerplate)
**Fix:** Create `TestBoardBuilder`, `add_text_item()` helpers
**Impact:** 15% less test code, easier maintenance

### M2. Add Snapshot Testing
**File:** `Cargo.toml`, test files
**Issue:** No `insta`/`expect-test` for complex output verification
**Fix:** Add `insta` crate, convert serialization tests
**Impact:** Self-documenting tests, easier updates

### M3. Move Serialize/Deserialize to Leaf Modules
**File:** `src/types.rs`
**Issue:** 11 serde derives in core types (could be in persistence layer)
**Options:**
- Add doc comments explaining why each type is serializable
- Create newtype wrappers in `board.rs` for persistence
**Impact:** Cleaner architecture, optional

### M4. Review Feature Flags
**File:** `Cargo.toml`
**Candidates to disable:**
- `gpui-component`: is `tree-sitter-languages` needed?
- `polars`: is `streaming` needed? is `dtype-struct` needed?
**Impact:** Faster builds if features unused

### M5. Add #[must_use] Annotations
**Files:** Various public APIs
**Issue:** Missing `#[must_use]` on `Result` types and builders
**Impact:** Better compile-time warnings

---

## LOW PRIORITY (Polish)

### L1. Fill Test Coverage Gaps
**Areas:**
- `csv_parser` (only 2 minimal tests)
- Drag/drop integration tests
- Focus management scenarios
- Command registry (only 3 tests)

### L2. Table-Driven Tests
**Files:** `tests/unit/types_tests.rs`, `tests/unit/validation_tests.rs`
**Issue:** 8+ similar tests with manual assertions
**Fix:** Convert to parametrized/table-driven tests

### L3. Remove Trivial Tests
**Candidates:**
- `test_focus_context_priority()` - compiler-enforced
- `test_key_context_strings()` - trivial enum orderings

### L4. Target Directory Cleanup Strategy
**Issue:** 8GB target directory (6.3GB debug, 1.7GB release)
**Fix:** Add periodic `cargo clean` or cache cleanup in CI

### L5. Consider sccache
**File:** `.cargo/config.toml` (already commented out)
**Fix:** Uncomment and install for distributed caching

---

## METRICS SUMMARY

| Category | Current | Target |
|----------|---------|--------|
| Test binaries | 3 | 1 |
| Direct deps | 30 | 25-27 |
| Transitive deps | ~990 | ~700 (if polars removed) |
| Proc macro instances | 112 | ~80 |
| Test time (sleep) | 1.8s | <0.1s |
| Clean build | ~2-3 min | ~1-1.5 min |
| Target size | 8GB | 5GB |

---

## PASSED AUDITS (No Action Needed)

- **Profile settings**: Excellent dev/release/dev-fast configuration
- **Generics at boundaries**: Proper type erasure, no bloat
- **#[inline] usage**: Sparse and targeted (10 uses in 27k LOC)
- **impl Trait usage**: Correct for iterator returns
- **Module organization**: Clean separation of concerns
- **Error handling**: Structured errors with proper context
- **Per-package optimization**: Smart overrides for hot paths

---

## ARCHITECTURE SCORE: 8.5/10

**Verdict:** Production-ready foundation with cleanup opportunities. Critical items (C1-C3) should be fixed before any release. High priority items significantly improve build times and test reliability.
