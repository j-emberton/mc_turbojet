# mc_turbojet

A minimal static sea-level turbojet toy model implemented in Rust and exposed to Python via PyO3.

This project implements a simplified Brayton-cycle turbojet model under static sea-level assumptions. It is intended as a demonstration of scientific modelling structure and Rust-based acceleration, rather than a validated engine performance model.

The focus of this repository is:

- Clear separation between domain logic and Python bindings  
- Deterministic, testable modelling code  
- Incremental development towards uncertainty propagation and parallel Monte Carlo  
- Reproducible packaging using maturin and uv  

---

## Development Setup

### Requirements

- Rust toolchain (stable)
- Python ≥ 3.13
- uv

### Setup

```bash
uv venv
uv sync --group dev
```

### Running tests

```bash
uv run pytest
```

### Build from source

```bash
uv build
```

### Example usage

```python
import mc_turbojet as tj

ve, thrust = tj.evaluate_performance(
    pi_c=20.0,
    tt4=1600.0,
    eta_c=0.9,
    eta_t=0.92,
    eta_n=0.95,
    pi_b=0.95,
)

print(ve, thrust)
```

### Scope and Limitations
This model:
- Assumes static sea-level ambient conditions
- Uses ideal-gas relations with constant γ and cp
- Does not include choking analysis
- Does not include flight Mach or altitude effects
- Is not calibrated against real engine data

It is intentionally simplified and serves as a structured modelling demonstrator.