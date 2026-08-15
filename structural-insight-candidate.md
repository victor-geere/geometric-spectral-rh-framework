# Structural Insight Candidate

**Attempts at a non-circular algebraic/analytic reason that could force coherent cancellation onto the critical line**

This note records successive attempts to supply the kind of structural insight that is still missing from the pathways listed in `prime-zero.md`. The goal is an identity or rigidity statement that forces the residual spectral mass of the coherent sums to lie on the critical line *without* presupposing the Riemann Hypothesis or merely reformulating the classical explicit formula.

---

## Attempt 1 — Angle-averaged Gram kernel K_φ

Fejér (and more generally any continuous positive-definite) densities φ produce kernels
$$
K_\phi(s,s') = \sum_m m^{-(s+\overline{s}')}\widehat\phi(m)
$$
that are positive-semidefinite on every vertical line Re s > 1/2 by elementary Fourier analysis, independently of zeros. The construction fails to force support onto the critical line because positivity only controls average size; no independent rigidity links the kernels to the prime-side trace strongly enough to annihilate off-critical mass.

---

## Attempt 2 — Averaged logarithmic potential

The potential
$$
\Psi_\phi(s) = \frac1{2\pi}\int \log|F(\theta;s)|\,\phi(\theta)\,d\theta
$$
is harmonic for σ > 1 and its Laplacian recovers the averaged zero measure. Matching to the prime-side trace produces a boundary-value problem, but uniqueness is too weak without already knowing the zeros are those of ζ. Circularity persists.

---

## Attempt 3 — Cosine kernel

Replace the complex exponential by its real part:
$$
C(\theta;s) = \operatorname{Re} F(\theta;s) = \sum_{m=1}^\infty m^{-\sigma}\cos(m\theta - t\log m).
$$
Form the cosine kernel
$$
K^\cos_\phi(s,s') = \frac1{2\pi}\int_0^{2\pi} C(\theta;s)\,C(\theta;s')\,\phi(\theta)\,d\theta.
$$
For Fejér densities φ_N one obtains (up to a factor 1/2) the same series with non-negative coefficients. Positive-semidefiniteness on every vertical line Re s > 1/2 follows by the identical elementary argument and is independent of zero-free information.

The cosine kernel is real and even, aligning with the symmetry of the Guinand–Weil formula and the prime-side trace. Nevertheless the same obstruction remains: positivity controls average size of the real parts only; Davenport–Heilbronn-type off-line zeros survive after taking real parts and averaging; no new algebraic relation forces the residual measure onto the critical line.

**Status of Attempt 3:** recorded. Positive-definiteness succeeds unconditionally; rigidity fails for the same structural reason as Attempts 1 and 2.

---

## Next direction under investigation

Shift the critical line to Re w = 0 (via w = s − 1/2) so that the functional equation becomes w ↔ −w and zeros appear as twin pairs. Investigate whether offline twin zeros cancel each other in a suitably defined coherent sum or cosine kernel after the shift.

---

*Related files:* `prime-zero.md`, `prime-side.md`, `zero-side.md`.
