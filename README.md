# Geometric-Spectral Framework for the Riemann Hypothesis

**Terminal formal reduction**

*Private working document*

Date: 13 August 2026

## Status in one sentence

The framework has been reduced to a single open existence assertion in functional analysis. If that assertion is true, the Riemann Hypothesis follows by standard spectral theory. The assertion has not been proved.

## Unconditional analytic facts

**Purification identity.** For every integer N ≥ 2,

```
∑_{k=1}^{N-1} Li_s(e^{2π i k / N}) = (N^{1-s} - 1) ζ(s).
```

The zeros of the elementary factor N^{1-s} - 1 lie on the line Re(s) = 1 and are therefore outside the critical strip. Every zero of the left-hand side that lies in the critical strip is a zero of ζ.

**Completed xi-function.** The non-trivial zeros of

```
Ξ(s) = (1/2) s(s-1) π^{-s/2} Γ(s/2) ζ(s)
```

are precisely the non-trivial zeros of ζ. The elementary factors contribute only the points s = 0, s = 1 and the non-positive even integers.

## Conditional implications from Mourre theory

Assume a self-adjoint operator H on a scale of Hilbert spaces {ℋ^s} together with conjugate operators satisfying relative bounds and quantitative Mourre estimates (including at thresholds), and assume a residual projection P_res that isolates the continuous spectrum orthogonal to the pure-point spectrum. Then the following are theorems of abstract Mourre theory:

1. The residual continuous spectrum is purely absolutely continuous (singular continuous spectrum is empty).
2. The limiting absorption principle holds on every Mourre interval.
3. Residual eigenvalues (if any) have finite multiplicity and cannot accumulate inside a Mourre interval.
4. The residual spectral measure admits a locally Hölder density ρ_res.
5. If that density vanishes almost everywhere, then P_res = 0 and the residual continuous spectrum is empty.
6. Strong resolvent convergence of approximating operators H_N that satisfy uniform Mourre estimates cannot create singular continuous spectrum in the limit.

## The open existence assertion

The Riemann Hypothesis is true if there exist:

1. a scale of Hilbert spaces {ℋ^s}_{s∈ℝ} with continuous dense embeddings ℋ^s ↪ ℋ^t for s > t,
2. a self-adjoint operator H on ℋ^0 whose pure-point spectrum coincides with the non-trivial zeros of Ξ and whose residual continuous spectrum is empty,
3. a family of self-adjoint conjugate operators {X} satisfying the relative bounds

   ```
   ||X ψ|| ≤ a_X ||H ψ|| + b_X ||ψ||,   a_X < 1,
   ```

   and the quantitative Mourre estimates

   ```
   E_H(I) i[H, X] E_H(I) ≥ θ_X E_H(I) + K_X
   ```

   (θ_X > 0, K_X compact from ℋ^1 to ℋ^{-1}) on a cover of the real line by intervals I, including thresholds,
4. a sequence of self-adjoint approximating operators H_N that converge to H in the strong resolvent sense and satisfy the same Mourre estimates with constants uniform in N.

If such objects exist, then by the unconditional facts and the Mourre-theoretic implications listed above every non-trivial zero of ζ is a real eigenvalue of a self-adjoint operator (hence lies on the critical line) and there are no other non-trivial zeros. This is the Riemann Hypothesis.

## What is not required

The many geometric, thermodynamic, large-deviation, ultrametric, motivic, Floer, Hecke, tropical, soft-charge, anomalous, observer-dependent, non-perturbative, quantum and landscape structures introduced during the expansion process are optional compatibility conditions. They are not needed for the implication “existence of the operators ⇒ RH” and do not help to construct the operators.

## Final status

- The purification identity and the elementary factors of Ξ are unconditional theorems.
- The implication from the existence of the operators to the Riemann Hypothesis is a rigorous consequence of the spectral theorem and abstract Mourre theory.
- The existence of the operators themselves remains unproved.

The geometric-spectral framework has been reduced to a single, precisely stated open problem in functional analysis. That problem, if solved affirmatively, would prove the Riemann Hypothesis. It has not been solved. The Riemann Hypothesis remains open.

No further expansion of formal layers is useful. The remaining work is the concrete construction (or the proof of non-existence) of the operators.
