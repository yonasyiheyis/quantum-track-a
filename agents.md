# agents.md — Quantum Track A Repo (Codex Agent Onboarding)

## Context
This repo is a learning project for **Track A: Quantum Software Engineering**. The human is learning quantum concepts on a strict schedule and is implementing supporting code in **Python + Rust**.

Your job (Codex agent) is to help implement and fix the codebase so that the exercises/tests run cleanly. Prefer small, safe changes with clear tests.

## Repo Layout (expected)
- `python/`
  - `.venv/` (Python virtualenv)
  - `requirements.txt`
  - `qc_math.py` (to be created/verified)
  - `tests/`
- `rust/`
  - `qc_core/` (Rust crate)
    - `src/`
      - `lib.rs`
      - `qc_math.rs`
- `docs/`
  - `progress.md`
- `Makefile` at repo root

## Current Goal (Session 2 deliverable)
Implement the **math core** used to bridge complex amplitudes → probabilities:

- `mag2(z)` : returns magnitude squared |z|^2
- `normalize(vec)` : scales complex vector so Σ|amp|^2 = 1
- `probs(vec)` : returns probabilities from amplitudes (normalize first, then |amp|^2)

Do this in **both**:
- Python (NumPy)
- Rust (`num-complex`)

Then ensure `make test` passes.

## Current Problem
`make test` fails during Python test collection with:

- `ModuleNotFoundError: No module named 'qc_math'`

Even after modifying Makefile to run pytest from inside `python/`:
- `cd python && . .venv/bin/activate && pytest -q`

This strongly suggests that `python/qc_math.py` is missing, misnamed, or not in the working directory expected by pytest.

## Constraints / Preferences
- Keep changes minimal and easy to understand.
- Add/keep unit tests for correctness.
- Do not introduce unnecessary tooling (no heavy packaging unless needed).
- Prefer a solution that makes `make test` reliable from repo root.

## Step-by-step Debug Checklist (do in order)
### 1) Verify file existence and naming
From repo root:
- `ls -la python`
- Confirm there is a file exactly named: `python/qc_math.py`
- Confirm tests file exists: `python/tests/test_qc_math.py`

If `qc_math.py` is missing, create it (see Implementation below).

### 2) Verify pytest working directory / import path
From repo root:
- `cd python && . .venv/bin/activate && python -c "import sys; print(sys.path[:3])"`
- `cd python && python -c "import qc_math; print(qc_math.__file__)"`

If that import fails, the file is not in `python/` or is named wrong.

### 3) Fix Makefile Python test target
Recommended Makefile `test-python` target:
- `cd python && . .venv/bin/activate && pytest -q`

Alternative (more robust) if needed:
- `cd python && . .venv/bin/activate && PYTHONPATH=. pytest -q`

Only add `PYTHONPATH` if necessary.

### 4) Run full suite
From repo root:
- `make test`

## Implementation Requirements

### Python: `python/qc_math.py`
Use NumPy. Functions:

- `mag2(z: complex) -> float`
  - Return real float for |z|^2.
- `normalize(vec) -> np.ndarray`
  - Convert to `np.complex128`.
  - Compute norm2 = Σ |v_i|^2.
  - Raise `ValueError` if norm2 == 0.
  - Return `vec / sqrt(norm2)`.
- `probs(vec) -> np.ndarray`
  - Normalize first, return |amp|^2 as float array.

Suggested implementation:
```python
import numpy as np

def mag2(z: complex) -> float:
    return float(np.real(z * np.conjugate(z)))

def normalize(vec) -> np.ndarray:
    v = np.asarray(vec, dtype=np.complex128)
    norm2 = float(np.sum(np.abs(v) ** 2))
    if norm2 == 0.0:
        raise ValueError("Cannot normalize the zero vector.")
    return v / np.sqrt(norm2)

def probs(vec) -> np.ndarray:
    v = normalize(vec)
    return (np.abs(v) ** 2).astype(np.float64)
```

### Python tests: `python/tests/test_qc_math.py`
Test cases:
- `mag2(3+4j) == 25.0`
- `mag2(1-1j) == 2.0`
- `probs([1,1])` ≈ `[0.5, 0.5]`
- `probs([1+1j, 0])` ≈ `[1.0, 0.0]`
- Sum of probabilities ≈ 1 (tight tolerance)
- `normalize([0,0])` raises `ValueError`

### Rust: `num-complex` + module file
- Add dependency `num-complex = "0.4"` in `rust/qc_core/Cargo.toml`
  - Ensure there is **only one** `[dependencies]` table. Merge if needed.

Create `rust/qc_core/src/qc_math.rs`:
```rust
use num_complex::Complex64;

pub fn mag2(z: Complex64) -> f64 { z.norm_sqr() }

pub fn normalize(vec: &[Complex64]) -> Result<Vec<Complex64>, &'static str> {
    let norm2: f64 = vec.iter().map(|z| z.norm_sqr()).sum();
    if norm2 == 0.0 { return Err("Cannot normalize the zero vector."); }
    let scale = 1.0 / norm2.sqrt();
    Ok(vec.iter().map(|z| *z * scale).collect())
}

pub fn probs(vec: &[Complex64]) -> Result<Vec<f64>, &'static str> {
    let v = normalize(vec)?;
    Ok(v.iter().map(|z| z.norm_sqr()).collect())
}
```

Update `rust/qc_core/src/lib.rs` to:
- `pub mod qc_math;`
- Add tests mirroring the Python tests (use approx comparison).

## Expected Success Criteria
- `make test` passes from repo root.
- Python and Rust tests cover all required behaviors.
- No extra binaries or unused files (repo stays clean).

## Notes about Environment
- OS: Linux
- Rust installed via rustup (cargo/rustc available)
- Python venv: `python/.venv`
- Python version may be from Linuxbrew (e.g., 3.14). Avoid assumptions about system Python; always use the venv interpreter.

## Communication Style
- Make one small change at a time.
- After each change, run tests and record output.
- If you need to choose between two fixes, prefer the simpler one and explain why in a short commit message.
