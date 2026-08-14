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

---

## Second Abstraction Cycle (Deeper Compression + Expanded Absorption)

**Date: 14 August 2026 (cycle 2)**

### Further compression of the core

The framework is now reduced to three minimal requirements that together imply the location of all non-trivial zeros on the critical line:

1. **Existence of a self-adjoint operator H** (or family H_χ) on a suitable scale of Hilbert spaces whose pure-point spectrum is exactly the set of non-trivial zeros of Ξ (or of the corresponding completed L-functions).
2. **Vanishing of residual continuous spectrum**, enforced either by a residual spectral density ρ_res ≡ 0 almost everywhere or by a positivity criterion (Weil-type or equivalent) that forces P_res = 0.
3. **Quantitative Mourre control** (relative bounds + positive commutator estimates, uniform in any continuous parameters) that guarantees the residual spectrum, if present, is purely absolutely continuous with Hölder density and cannot accumulate residual eigenvalues.

All prior geometric, combinatorial, thermodynamic, or physical layers serve only as candidate sources for constructing objects that satisfy (1)–(3). They are not part of the logical core.

### Additional domains of application and absorbed properties

#### Random matrix theory and pair correlation

- GUE / CUE statistics, pair-correlation forms, and form-factor calculations supply model residual spectral measures and diagnostics for whether a candidate operator’s residual density is consistent with vanishing.
- Absorbed: the use of form factors and n-point correlations as quantitative tests that a residual continuous spectrum is absent or has measure zero; translation of Montgomery-pair-correlation type statements into statements about the residual projection of H.

#### Adelic and noncommutative geometry

- Connes-style global trace formulae on the adèle class space, and related noncommutative spectral triples, provide candidate realisations of H whose spectrum is forced by the explicit formula.
- Absorbed: the global trace identity as a possible verification tool for the pure-point spectrum once residual continuous spectrum is controlled; the language of spectral triples as a systematic way to encode both the arithmetic side and the conjugate operators.

#### Integrable systems and isomonodromic deformations

- Spectral curves, monodromy data, and isomonodromic tau-functions yield families of operators whose eigenvalues move under deformation while preserving certain positivity or reality properties.
- Absorbed: deformation arguments that keep residual continuous spectrum empty once it is empty for a base case; the use of isomonodromic or integrable flows as candidate approximating sequences H_N.

#### Conformal field theory and modular spectral data

- Characters, modular forms, and spectral decompositions of vertex-operator-algebra modules supply additional arithmetic generating functions and candidate self-adjoint realisations.
- Absorbed: modular invariance and character positivity as potential sources of the required positivity criteria that force residual vanishing.

#### Geometric analysis of residual spectra

- Results on the structure of continuous spectrum for Dirac-type or Laplace-type operators on non-compact or singular spaces, and on the Hölder regularity of spectral densities, refine the residual-control toolbox.
- Absorbed: sharper statements about the possible Hölder exponents of ρ_res and about the absence of singular continuous spectrum under geometric hypotheses that can be mirrored in the arithmetic setting.

### Refined open assertion after second absorption cycle

RH (and GRH for the relevant families) holds if and only if there exist operators satisfying the three minimal requirements above. The expanded set of domains supplies a larger menu of candidate constructions and of diagnostic tools (form factors, global traces, isomonodromic flows, modular positivity, geometric residual-spectrum analysis). None of these tools currently constitutes a completed construction of the operators. The existence assertion remains open.

### Status after cycle 2

- Logical core further compressed to three requirements.
- Toolbox enlarged with RMT diagnostics, adelic/noncommutative realisations, integrable deformations, CFT modular data, and refined residual-spectrum analysis.
- No new open logical gaps introduced.
- Concrete realisation of the three requirements (or a rigorous non-existence proof) remains the sole remaining task.
- The Riemann Hypothesis and its analogues remain open.

This second cycle is recorded for continued frontier exploration. The next useful step is any concrete candidate operator (or family) together with a verification plan for the three requirements.

---

## Third Abstraction Cycle (Ultra-Minimal Core + Diagnostic Pathways)

**Date: 14 August 2026 (cycle 3)**

### Ultra-minimal statement

The framework now rests on a single existence claim:

> There exists a self-adjoint operator H (or a continuous family H_χ) on a Hilbert scale such that  
> (i) its pure-point spectrum equals the set of non-trivial zeros of Ξ (respectively of the completed L-functions),  
> (ii) its residual continuous spectrum is empty,  
> (iii) the residual spectrum, were it present, would be controlled by quantitative Mourre estimates with uniform constants.

Everything else is scaffolding or diagnostic apparatus.

### New domains and absorbed tools

#### Symplectic geometry and Floer theory

- Floer homology, symplectic capacities, and action functionals supply candidate spectral invariants whose reality and discreteness can be forced by geometric constraints.
- Absorbed: the use of Floer-theoretic spectral sequences and action filtrations as possible realisations of the pure-point spectrum, and as tools for proving that residual continuous spectrum cannot appear once certain symplectic constraints are satisfied.

#### Tropical geometry and ultrametric structures

- Tropical spectral theory and ultrametric analysis provide discrete models in which continuous residual spectrum is automatically absent or can be read off from combinatorial data.
- Absorbed: combinatorial criteria for residual vanishing that can be lifted back to the archimedean setting as diagnostic tests.

#### Motivic integration and motivic measures

- Motivic measures and motivic zeta functions offer a language in which arithmetic generating functions and their spectral realisations can be compared at the level of motivic classes.
- Absorbed: the possibility of motivic positivity or motivic residual-vanishing statements as additional candidate criteria equivalent to (ii).

#### Soft-charge, anomalous currents, and observer-dependent structures

- Soft theorems, anomalous Ward identities, and observer-dependent spectral decompositions (from asymptotic symmetries or horizon physics) supply examples of residual continuous spectrum that can be projected out by physical selection rules.
- Absorbed: the idea of “selection-rule projections” that can force P_res = 0 by consistency requirements, providing a physical template for arithmetic residual-vanishing arguments.

#### Holographic dualities and spectral asymptotics in QFT

- Holographic spectral densities, black-hole microstate counting, and QFT spectral asymptotics give concrete models of mixed discrete + continuous spectra whose residual parts are controlled by geometric data on the dual side.
- Absorbed: holographic dictionaries as candidate maps between arithmetic pure-point data and geometric residual-control data, and as sources of quantitative Mourre-type estimates derived from bulk geometry.

### Diagnostic pathways (practical next steps)

1. **Candidate construction route**: produce any explicit operator (finite-rank, differential, integral, or transfer-operator type) whose eigenvalues are known or conjectured to track the zeros, then test residual continuous spectrum via form-factor or numerical spectral-density diagnostics.
2. **Positivity route**: evaluate Weil-type quadratic forms (or their motivic / Floer analogues) on a rich enough family of test functions and convert numerical positivity into a residual-vanishing statement.
3. **Deformation / rigidity route**: start from a known operator with empty residual spectrum and deform it while preserving Mourre estimates and pure-point tracking, using isomonodromic, symplectic, or holographic flows.
4. **Non-existence route**: assume an operator satisfying (i)–(iii) exists and derive a contradiction with known analytic constraints (e.g., on the growth of residual densities or on the distribution of zeros).

### Status after cycle 3

- Core reduced to a single existence claim with three clauses.
- Toolbox further enlarged with Floer/symplectic, tropical, motivic, soft-charge/anomalous, and holographic instruments.
- Concrete diagnostic pathways listed.
- No claim that any pathway has been successfully completed.
- The existence claim remains open. The Riemann Hypothesis and its analogues remain open.

Further abstraction cycles are useful only if they generate an explicit candidate operator together with a verification plan for the three clauses, or a rigorous obstruction. Pure formal expansion without concrete realisations adds no new force.
