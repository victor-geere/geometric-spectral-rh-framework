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

---

## Abstracted Core of the Framework (Frontier Extension)

**Date of abstraction: 14 August 2026**

The geometric-spectral framework is abstracted as follows.

### Core structural elements

1. **Purification / projection identities** that isolate an arithmetic object (ζ, L-functions, or analogous generating functions) from elementary factors whose zeros lie outside the region of interest.
2. **Geometric or combinatorial constructions** (greedy partitions, centred step functions, correlation kernels of rank-bounded type, transfer operators, helix or quaternionic lifts, balance conditions on model manifolds) that produce candidate operators or quadratic forms.
3. **Spectral reduction** of those constructions to a self-adjoint operator H (or a family of operators) on a scale of Hilbert spaces, together with a residual continuous-spectrum projection P_res.
4. **Conjugate operators and quantitative Mourre estimates** that control the residual continuous spectrum, force absolute continuity, and supply limiting-absorption principles.
5. **Positivity or vanishing criteria** (Weil-type quadratic forms, residual spectral density ρ_res ≡ 0, eigenvalue suppression of finite-dimensional approximations) that force the pure-point spectrum to coincide with the arithmetic zeros and eliminate residual continuous spectrum.
6. **Approximating sequences** H_N with uniform Mourre constants that converge in the strong resolvent sense, ensuring the limiting operator inherits the spectral structure.

Any concrete geometric, physical or combinatorial layer is retained only insofar as it assists the construction or the verification of the Mourre data and the residual-vanishing condition. Layers that do not contribute to these data may be discarded without loss of logical force.

### Absorption of properties from applications

#### Mathematics

- **Dirichlet L-functions and higher L-functions.** Twisted transfer operators or character-dependent kernels yield families of operators H_χ. The framework absorbs multi-parameter Mourre estimates (uniform in the character) and deformation arguments that preserve residual-vanishing once it holds for the base case ζ.
- **Spectral geometry of compact and non-compact manifolds.** Eigenvalue distribution, residual continuous spectrum of Laplace-type or Dirac-type operators, and geometric balance conditions (curvature, spin structures, effective dimension) supply concrete models for the scale {ℋ^s} and for the residual projection. Spectral rigidity theorems (injectivity of spectral maps, uniqueness of constant profiles) are absorbed as tools for uniqueness of the pure-point spectrum once residual continuous spectrum is empty.
- **Operator-theoretic positivity criteria.** de Branges spaces, Hilbert–Pólya operators, and explicit-formula quadratic forms are treated uniformly: positivity of a sufficiently rich family of test functions, or vanishing of a residual density, is the operative condition.

#### Physics

- **Quantum Hamiltonians with mixed spectrum.** Scattering theory and Mourre theory for Schrödinger operators with continuous spectrum supply the precise language of conjugate operators, thresholds, and Hölder continuity of residual spectral densities. These are absorbed directly into the arithmetic setting as the control mechanism for residual continuous spectrum.
- **Black-hole quasinormal modes and geometric spectroscopy.** Inverse-resonance maps, Jacobian rank conditions, Lipschitz stability of multipolar reconstructions, and conditioning diagnostics are absorbed as methods for verifying that a spectral map (zeros ↔ eigenvalues) is locally injective and stable. Residual continuous-spectrum control becomes a stability statement under geometric deformations of the potential barrier.
- **Open quantum systems and non-Markovian dynamics.** Geometric spectral densities (Weyl asymptotics on model manifolds) that generate algebraic bath correlations and fractional master equations supply examples of residual continuous spectrum with controlled Hölder densities. The framework absorbs the constructive embedding of such residual spectra into augmented Lindblad or auxiliary-oscillator systems, providing a model for how residual arithmetic continuous spectrum might be regularised or projected out.
- **Spectral rigidity and deformation flows.** Quantized geometric modes, energy functionals that force relaxation to maximal symmetry, and dual decay of spectral energy and entropy are absorbed as dynamical mechanisms that can drive residual continuous spectrum to zero or force pure-point spectrum onto a critical locus.

### Updated open assertion after absorption

The Riemann Hypothesis (and its analogues for families of L-functions) follows if there exist operators H (or H_χ) satisfying the abstracted conditions above, with the additional structural properties imported from the applications:

- uniform quantitative Mourre estimates in continuous or discrete families (characters, deformation parameters),
- residual spectral densities that are Hölder continuous and can be shown to vanish by positivity, rigidity, or inverse-resonance stability,
- approximating sequences whose spectral maps remain injective and conditioned under the geometric or arithmetic deformations under consideration.

The existence of such operators remains the single open assertion. All geometric and physical layers are now understood as optional sources of candidate constructions or of analytic tools that strengthen the Mourre and residual-control data. They do not replace the existence requirement.

### Status of the extended framework

- The abstraction preserves the logical reduction: existence of the controlled self-adjoint operators ⇒ location of all arithmetic zeros on the critical line (or the corresponding spectral locus).
- Cross-domain absorption enlarges the toolbox for constructing or verifying the Mourre data and the residual-vanishing condition without introducing new open logical gaps.
- The concrete construction of the operators (or a proof that no such operators exist) remains the sole remaining task.
- No claim is made that the existence assertion has been settled. The Riemann Hypothesis and its analogues remain open.

This extension is recorded for frontier exploration. Further concrete constructions that realise the abstracted data are the next useful step.
