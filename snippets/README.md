# Implementation Snippets

Concrete Rust code corresponding to the Flag 6 native-infinity formulation and the analytic certificate hooks.

| File | Contents |
|------|----------|
| `tempered_measure.rs` | `TemperedMeasure` trait, `PurePointSpectrum`, `ContinuousDensity`, `AtomicMeasure`, `MixedMeasure`, `from_known_zeros` |
| `flag6_certificate.rs` | `SupportProof` / `ResidualProof` enums, `Flag6Certificate` trait, support matching, residual checks, `Flag6NativeOrchestrator` |

## Honesty boundary

- These snippets implement **data structures, numerical checks, and certificate hooks**.
- They do **not** prove Flag 6.
- Only a `Flag6Certificate` whose `SupportProof` and `ResidualProof` are both `Analytic` would discharge the existence claim.
- All default paths leave Flag 6 open.

## Usage sketch

```rust
use tempered_measure::from_known_zeros;
use flag6_certificate::Flag6NativeOrchestrator;

let zeros = vec![14.134725, 21.022040, 25.010858];
let mu = from_known_zeros(&zeros);
let orch = Flag6NativeOrchestrator::new(zeros.clone(), 1e-10);
println!("{}", orch.report(None, &mu, 30.0));
```

Further snippets (Φ emitter, helix axiom, Cadical FFI) can be added alongside these core measure types.
