# Status — Spectral Zeta Functions

**Date: 14 August 2026**

Investigation of spectral zeta functions in the context of the geometric-spectral framework and Flag 6 (native infinity form).

---

## 1. Definition

Let \(H\) be a positive self-adjoint operator (or an operator with spectrum bounded away from zero after a shift) on a Hilbert space, with discrete spectrum \(\{\lambda_n\}_{n=1}^\infty\) (counted with multiplicity). The **spectral zeta function** of \(H\) is

```
ζ_H(s) = Σ_{n=1}^∞ λ_n^{-s},
```

initially for \(\operatorname{Re}s\) large enough that the series converges, and then continued meromorphically wherever possible.

More generally, if \(\mu\) is the spectral measure of \(H\),

```
ζ_H(s) = ∫_{σ(H)\\{0}} λ^{-s} dμ(λ).
```

When residual continuous spectrum is present the integral acquires an absolutely continuous contribution; when residual spectrum vanishes the spectral zeta reduces to a pure discrete sum.

---

## 2. Classical properties

- **Meromorphic continuation.** For elliptic operators on compact manifolds (or suitable non-compact settings with controlled continuous spectrum) \(\zeta_H(s)\) admits a meromorphic continuation to \(\mathbb{C}\). Poles are determined by the heat-kernel asymptotics (Seeley–DeWitt coefficients).
- **Regularized determinant.**

  ```
  det_ζ H = exp(−ζ_H'(0)).
  ```

  This is the standard zeta-regularized determinant used in geometry and quantum field theory.
- **Functional equations.** For many geometric operators the spectral zeta satisfies a functional equation relating \(s\) to a complementary value (often involving the dimension and the principal symbol). The completed Riemann xi-function \(\Xi(s)\) is the prototype arithmetic analogue.
- **Relation to the heat kernel.**

  ```
  ζ_H(s) = 1/Γ(s) ∫_0^∞ t^{s−1} Tr(e^{−tH}) dt
  ```

  (for \(\operatorname{Re}s\) large). Small-\(t\) asymptotics control the poles; large-\(t\) asymptotics control the pure-point contribution.

---

## 3. Link to the Riemann zeta function and to Flag 6

If there existed a Hilbert–Pólya operator \(H\) whose pure-point spectrum is exactly \(\{\operatorname{Im}\rho\}\) (the ordinates of the non-trivial zeros of \(\Xi\)) and whose residual continuous spectrum is empty, then the spectral zeta of that operator would be essentially

```
ζ_H(s) = Σ_ρ (Im ρ)^{−s}
```

(up to trivial shifts and multiplicities). In that case:

- the pure-point support condition of Flag 6 is precisely the statement that the spectrum of \(H\) is the zero set;
- residual vanishing is the statement that \(\zeta_H\) receives no continuous-spectrum contribution;
- the arithmetic zeta \(\zeta(s)\) (or \(\Xi(s)\)) and the spectral zeta \(\zeta_H(s)\) become dual objects linked by an explicit formula / trace formula.

Conversely, any construction that produces a measure \(\mu\) satisfying Flag 6 immediately yields a spectral zeta

```
ζ_μ(s) = ∫ |t|^{−s} dμ(t)
```

whose poles and zeros encode the zero set of \(\Xi\).

---

## 4. Relation to the geometric-spectral framework

| Framework object | Spectral-zeta counterpart |
|------------------|---------------------------|
| Map \(\mathcal{T}\) and its fixed points | Candidate spectral measures \(\mu\); \(\zeta_\mu\) is the associated spectral zeta |
| Finite-rank truncations / \(\Phi^\star_{N,M,K}\) | Finite spectral zeta sums over the realised pure-point locations |
| Residual density \(\rho\) | Continuous contribution to \(\zeta_H(s)\) |
| Mourre estimate | Guarantees that the continuous contribution is absolutely continuous and, if residual mass vanishes, disappears from \(\zeta_H\) |
| Helix / clock arithmetic | Discrete approximation to the spectrum that can be fed into a truncated spectral zeta |
| Flag 6 (native) | Existence of \(\mu\) such that \(\zeta_\mu\) is a pure discrete sum over the ordinates of the zeros of \(\Xi\) |

Inside the framework the spectral zeta therefore appears as the generating function of the pure-point data once residual continuum has been eliminated.

---

## 5. Analytic strategies that involve spectral zeta functions

1. **Zeta-regularized determinants and positivity**  
   Positivity or reality properties of \(\det_\zeta(H+z)\) (or of related Hadamard products) can force eigenvalues onto the real line / critical line.

2. **Trace formulae**  
   Equality of a geometric/spectral side \(\operatorname{Tr}f(H)\) with an arithmetic side (explicit formula involving zeros of \(\Xi\)) is equivalent to the spectral measure of \(H\) being supported on the zeros. Spectral zeta values are the moments / Mellin transforms of that measure.

3. **Heat-kernel / spectral asymptotics**  
   Matching the Weyl law of a candidate operator to the known density of zeros of \(\Xi\) is a necessary condition for a Hilbert–Pólya realisation; the spectral zeta encodes the same asymptotic data via its poles.

4. **Mellin transform of residual density**  
   The continuous contribution to \(\zeta_H(s)\) is the Mellin transform of the residual density. Vanishing of residual mass (Flag 6) is equivalent to the absence of that continuous contribution.

---

## 6. Status and caution

- Spectral zeta functions are rigorously defined for large classes of operators and are standard tools in global analysis.
- No spectral zeta arising from a fully constructed self-adjoint operator is presently known to have pure-point support exactly equal to the non-trivial zeros of \(\Xi\).
- Several recent preprints claim Hilbert–Pólya-type operators or spectral realisations; none has been accepted as a completed proof of RH. The framework treats them as candidate constructions that still require verification of residual vanishing and exact support matching (i.e., Flag 6).

---

## 7. Immediate use inside the present architecture

- The finite models already produce truncated spectral zeta sums

  ```
  ζ_K(s) = Σ_{k=1}^K (γ_k)^{−s}
  ```

  where \(\gamma_k\) are the realised pure-point locations.
- Residual snapshots supply a numerical continuous contribution that can be Mellin-transformed and monitored as parameters grow.
- A Lyapunov functional or tightness certificate can be rephrased as control on the continuous part of a spectral zeta.
- Once a candidate limiting measure \(\mu\) is obtained, \(\zeta_\mu(s)\) becomes the concrete analytic object whose pure discreteness would discharge Flag 6.

---

## Summary

Spectral zeta functions convert the spectral measure of a candidate operator (or of the measure \(\mu\) of Flag 6) into a meromorphic generating function. Residual vanishing is equivalent to the spectral zeta being a pure discrete sum over the ordinates of the zeros of \(\Xi\). They therefore sit at the precise interface between the geometric-spectral constructions of the framework and the native analytic statement of Flag 6.

**Flag 6 remains open.**
