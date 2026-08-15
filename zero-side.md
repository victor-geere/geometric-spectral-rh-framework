# Zero-Side Candidates

**Date: 15 August 2026**

Candidates whose zeros or pure-point spectrum are the *output* (dual to the prime side). Built from the geometric-spectral framework using a **cone** of test data so that a **closed-form continuous complete function** can be written.

Arithmetic-first: no non-trivial zero list as input. Davenport–Heilbronn filter: prefer Euler-product seeds so constructions discriminate against non-multiplicative DH counterexamples.

---

## 1. Cone and complete-function scaffold

**Cone of admissible test functions**

```
C = { φ ∈ C_c^∞(R) : φ(x)=φ(-x), Weil positive-definite, supp ⊂ [-1,1] }
```

(or a larger tempered cone with fixed decay). This is the Weil positivity cone, read as a geometric constraint on residual windows.

**Continuous complete function (schematic closed form)**

```
Ξ_geo(s) = exp( a + b s + ∫_0^∞ (F_cone(x)/2) (x^{-s/2} + x^{-(1-s)/2}) dx/x )
```

with `F_cone` built from geometric generators (greedy steps, helical carries, correlation masses) restricted to the cone. The symmetric Mellin structure imposes `Ξ_geo(s) = Ξ_geo(1-s)`.

Zeros of `Ξ_geo` are the zero-side output. Arithmetic fidelity means `Ξ_geo` shares zeros with classical `Ξ`.

---

## 2. Candidate Z1 — Cone–Weil geometric completion

**Data.** Greedy/tropical partition generators `{f_j}` on `[0,π]`; weights from residual-controlled arithmetic seed (no zero list).

**Cone.** `C` as above; each `φ ∈ C` defines a geometric Weil form

```
W(φ) = Σ_j w_j |widehat{f_j φ}|^2
```

**Complete function.**

```
Ξ_Z1(s) = exp( ∫_C log(1 + W(φ) c(s,φ)) dμ(φ) ) · s(s-1) π^{-s/2} Γ(s/2)
```

with `c(s,φ)` an archimedean/Mellin factor so the product is entire of order 1 after normalisation.

| Item | Value |
|------|--------|
| Zero side | Zeros of `Ξ_Z1` |
| Parameters | Rank `J`, cone measure `μ`, residual coupling `α ≤ 2/π` |
| Tractability | Finite-rank truncations of `W`; discrete cone = finite test `φ` |

---

## 3. Candidate Z2 — Helical cone flow → completed exponential

**Data.** Helix axiom: places `0..R-1`, carries, clock base `b`.

**Cone.** Nonnegative residual mass with `RMF(ρ) = ∫ ρ(t)/(1+t²) dt ≤ r_0` (local residual non-expansivity cone).

**Complete function.**

```
Ξ_Z2(s) = lim_{R→∞} exp( Σ_{r=0}^R δ^r ∫ K_r(t) ((1/2+it)^{-s} + (1/2+it)^{-(1-s)}) dt )
```

with `K_r` the geometric kernel at rotation `r` and `δ ∈ (0,1)` the RMF decrease factor. Under summability, entire of order ≤ 1 after polynomial prefactors.

| Item | Value |
|------|--------|
| Zero side | Zeros of `Ξ_Z2` |
| Parameters | `b`, `R`, `δ`, residual threshold `τ` |

---

## 4. Candidate Z3 — Correlation-kernel Fredholm determinant

**Data.** Geometric correlation kernel from Layer 1, spectral parameter on the critical line.

**Cone.** Positive-definite kernels `K ⪰ 0` on test vectors in `L²[0,π]`.

**Complete function.**

```
Ξ_Z3(s) = det(I + A(s)) · s(s-1) π^{-s/2} Γ(s/2)
```

where `A(s)` is a trace-class Mellin deformation of the geometric kernel (Fredholm determinant — closed form in the continuous-kernel sense).

| Item | Value |
|------|--------|
| Zero side | Zeros of `det(I+A(s))` (plus trivial archimedean zeros) |
| Parameters | Rank `J`, Mellin weight, residual coupling |
| Tractability | Finite-rank `A` ⇒ exponential polynomial; zeros computable without inserting `γ_k` |

**Recommended first numerical cut.**

---

## 5. Candidate Z4 — Cone Laplace transform of residual mass

**Data.** Residual density `ρ` of map `T`; cone `{ρ ≥ 0 : RMF(ρ) < r_0}`.

**Complete function.**

```
Ξ_Z4(s) = exp( -∫_0^∞ (ρ_*(u)/u) (u^{s/2} + u^{(1-s)/2}) du ) · ξ_arch(s)
```

with `ξ_arch(s) = s(s-1) π^{-s/2} Γ(s/2)`. If residual vanishing holds, nontrivial zeros disappear — so Z4 is bookkeeping unless `ρ_*` carries structured arithmetic data.

| Item | Value |
|------|--------|
| Role | Residual bookkeeping complete function |
| Priority | Secondary |

---

## 6. Candidate Z5 — Dual cosine (zero-side ansatz)

Prime cosine lives on `{log p}`. Zero-side dual:

```
K_zero(t,s) = (1/(2πi)) ∫_{c-i∞}^{c+i∞} L_geo(z) cos(t z) cos(s z) dz
```

with `L_geo` a closed-form geometric Euler product (not the zero list). Residues at zeros of `L_geo` feed the spectral measure of `K_zero`.

**Complete function.** Mellin of the diagonal (regularised):

```
Ξ_Z5(s) = ∫_0^∞ K_zero(t,t) t^{s/2} dt · ξ_arch(s)
```

**Cone.** Restrict `L_geo` so `K_zero ⪰ 0` on the Weil cone.

---

## 7. Comparison

| ID | Complete object | Zero mechanism | Cone | Inserts `γ_k`? | EP / DH-discriminating |
|----|-----------------|----------------|------|----------------|------------------------|
| Z1 | Cone integral of geometric Weil form | Zeros of `Ξ_Z1` | Weil | No | If seed is EP |
| Z2 | Helical exp completion | Zeros of `Ξ_Z2` | RMF | No | Design-dependent |
| Z3 | Fredholm `det(I+A(s))` | Det zeros | PSD | No | If `A` from EP |
| Z4 | Laplace of residual | Only if residual carries zeros | RMF | No | Weak |
| Z5 | Contour dual of geometric `L` | Residues / spectrum of `K_zero` | Weil PSD | No | Yes |

---

## 8. Recommended order of attack

1. **Z3 (finite rank)** — matrix `A(s)`, scan `det(I+A(1/2+it))` vs `t`.
2. **Z1 truncated cone** — few `φ`, rank `J ≤ 16`, DH-null control.
3. **Z5** — only after a concrete geometric Euler product `L_geo` is fixed.

---

## 9. Status

Zero-side candidates are defined. None is verified to equal classical `Ξ`. They avoid the prime-side trap (spectrum = correlations of `cos(t log p)`, not `γ_k`).

**Flag 6 remains open.**
