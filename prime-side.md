# Prime-Side Candidates

**Date: 15 August 2026**

Candidates built from **prime / Euler-product data** (von Mangoldt weights, Dirichlet `L(χ)`, geometric prime seeds). Spectrum encodes correlations of prime-side oscillatory factors — **not** the ordinates `γ_k` of `Ξ`.

Davenport–Heilbronn (1936) filter: constructions that score equally on Euler-product data and on non-multiplicative period-5 DH coefficients are not RH-specific.

---

## 1. What the prime side is

Explicit formula (schematic):

```
Σ_γ φ̂(γ)  =  (archimedean)  −  Σ_{p^k} (log p) p^{-k/2} φ(k log p)  + …
```

- **Zero side:** left-hand sum over ordinates `γ`.
- **Prime side:** right-hand sum over prime powers.

Prime-side operators quantise the right-hand side. Their eigenvalues are *not* the `γ_k` unless a dualisation step is proved.

---

## 2. Cosine kernel (best finite diagnostic)

### Definition

On a grid `t ∈ [0,T]` with `N` points:

```
K(t,s) = Σ_{p^k} (log p) p^{-k/2} cos(t · k log p) cos(s · k log p)
```

Matrix form:

```
K = V W V^T ,   V_{ij} = cos(t_i x_j),   x_j = k log p,   W = diag(w_j),   w_j = (log p) p^{-k/2} > 0
```

### Spectral properties

| Property | Status |
|----------|--------|
| Factorization | Gram `V W V^T` |
| Rank | `≤` number of retained prime powers; numerical rank `≤ min(N, #terms)` |
| Positive semidefinite | Yes on ζ-weights (`w_j ≥ 0`) |
| Self-adjoint | Yes |
| Euler product | Yes |
| Spectrum = `{γ_k}` | **No** |

Eigenvalues measure linear energy of the family `t ↦ cos(t · k log p)` on `[0,T]`. They encode geometry of `{log p}`, not the dual ordinates.

### Continuous limit

```
(𝒦 f)(t) = Σ_{p^k} w_{p^k} ⟨f, cos(x_{p^k} ·)⟩ cos(x_{p^k} t)
```

on `L²[0,T]`. Eigenfunctions are combinations of those cosines; eigenvalues are still not `γ_k`.

### Representative numerics (`N=48`, `T=90`, `P=600`)

- All eigenvalues positive (PSD).
- `λ_max ≈ 116`, trace `≈ 2234`.
- Affine-scaled Hausdorff distance to `{γ_n}` stays `O(1)–O(10)`; no collapse as `P` or `N` grows.
- Best sweep scores ~ scaled Hausdorff `2.2` are scaling artefacts, not term-by-term lock-on.

### Verdict

Clean, tractable, multiplicative prime-side operator. **Not** a Hilbert–Pólya candidate.

---

## 3. Other prime-side families swept

| ID | Construction | Tracks primes? | Tracks `γ_k`? | DH-disc? | Keep as HP? |
|----|--------------|----------------|---------------|----------|-------------|
| T1 | Gram + prime weights (diagonal) | Weakly | No | — | No |
| T5 | Jacobi matrix from prime measure | Yes | No | — | No |
| T2 | Schrödinger `-d²/dx² + V_prime` | Weakly | No | Weak | No |
| T4 | Helix + prime carries | Weakly | No | — | No |
| C1 | Berry–Keating discrete dilation | No | No | No | No |
| C2 | Explicit-formula prime Gram (Gaussians) | Yes | No | Weak | No |
| C3 | Hankel of prime moments | Yes | No | — | No |
| C4 | Cosine kernel (above) | Yes | No | Yes (mild) | No |
| Lχ | Same kernels with Dirichlet `χ mod 5` | Yes (EP) | No | Yes vs DH-null | No |
| DH-null | Period-5 coefficients, no EP | Fake | No | Control | Null only |

---

## 4. Davenport–Heilbronn filter

**Citation.** H. Davenport & H. Heilbronn, *On the zeros of certain Dirichlet series*, J. London Math. Soc. **11** (1936), 181–185, 307–312.

DH series have a zeta-type functional equation but **no Euler product**, and have zeros **off** the critical line.

**Multiplicative form of DH (not an EP):**

```
f(s) = c L(s,χ) + c̄ L(s,χ̄) = c ∏_p (1−χ(p)p^{-s})^{-1} + c̄ ∏_p (1−χ̄(p)p^{-s})^{-1}
```

Sum of Euler products ≠ Euler product.

**Corrected DH (has EP):** take a single factor `L(s,χ)` — then GRH is expected; the counterexample disappears.

**Filter rule.** A prime-side candidate aimed at RH must use structure DH lacks (Euler product / `Λ(n)`). If the same machine scores the same on prime data and on DH periodic coefficients, it is not an RH method.

In sweeps, cosine kernels with ζ / `L(χ)` weights often beat DH-null on scaled Hausdorff to `γ_k` (disc > 1). That confirms multiplicativity is visible — not that spectrum equals the zeros.

---

## 5. Expanded random search (summary)

Six families × random parameters, scored on ζ, `L(χ)`, DH-null:

| Family | Median bestH | Min bestH | Notes |
|--------|--------------|-----------|--------|
| schrod | ~3.2 | ~2.5 | Poor DH discrimination |
| tridiag | ~3.3 | ~3.2 | Geometric |
| cos | ~3.9 | **~2.2** | Best finite diagnostic |
| sin / cexp | ~4.3 | ~3.0 | Similar class |
| gram | ~13 | ~12 | Weak |

**KEEP cuts** (`bestH < 3.5`, `disc ≥ 1.3`): only a few cosine / complex-exp trials; none achieve lock-on.

---

## 6. Structural conclusion

```
Prime-side spectrum  ⊂  correlations of { cos(t log p) } / Gram geometry of {log p}
Zero-side spectrum   ⊂  { γ : Ξ(1/2+iγ)=0 }   (Flag 6 / Hilbert–Pólya)
```

No prime-side candidate in this programme has been shown to equal the zero side. Dualisation (explicit formula as an operator identity with residual vanishing) remains open.

See also: `zero-side.md` for geometric cone completions aimed at the zero side.

**Flag 6 remains open.**
