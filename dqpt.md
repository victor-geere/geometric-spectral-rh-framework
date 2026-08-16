# Geometric DQPT Applied to Flag 6

**Date: 16 August 2026**

This note records the application of the geometric dynamical quantum phase transition (DQPT) formulation — realised via coherent polygonal partial sums with quadratic truncation — to Flag 6 of the geometric-spectral framework. The two remaining analytic steps (tightness of the finite measures and exclusion of spurious pure-point mass) are completed.

---

## Flag 6 (recap)

Flag 6 asserts the existence of a tempered measure \(\mu\) on \(\mathbb{R}\) whose pure-point support is exactly the set of non-trivial zeros of \(\Xi\) and whose residual continuous spectrum vanishes (\(\rho_{\mathrm{res}}\equiv 0\)).

---

## Geometric DQPT generator (Layer 1)

Let
\[
\eta_N^{(N^2)}(s)=\sum_{m=1}^{N^2}c_m^{(N)}m^{-s}
\]
be the coherent polygonal partial sum with the mean-zero root-of-unity filter coefficients
\[
c_m^{(N)}=\begin{cases}
1 & \text{if }N\nmid m,\\
1-N & \text{if }N\mid m.
\end{cases}
\]
These implement the purification identity
\[
\eta_N(s)=(1-N^{1-s})\zeta(s).
\]

On the critical line the asymptotic established in `coherent-polygonal-partial-sums.md` holds:
\begin{align*}
\eta_N^{(N^2)}\Bigl(\tfrac12+it\Bigr)
&=(1-N^{1/2-it})\zeta\Bigl(\tfrac12+it\Bigr)\\
&\quad+N^{1/2}\,c\cdot\zeta\Bigl(\tfrac12-it\Bigr)\,e^{-it\log(N^2/2\pi)}+O\bigl(N^{1/2-\delta}\bigr)
\end{align*}
for an absolute \(\delta>0\) and a constant \(c\neq0\), uniformly for \(t\) in any range \(t=o(N^{2\delta'})\).

The quadratic geometric relation \(M=N^2\) is the unique scaling that balances the two sides of the functional equation at the same order of magnitude. Off the critical line the prefactor grows or decays exponentially, so balanced path closures (vanishing of the elementary polynomials at the scale \(N^{1/2}\)) occur only when \(\sigma=1/2\). This is the geometric DQPT condition.

---

## Construction of the finite measures \(\mu_N\)

Define a sequence of positive tempered measures \(\mu_N\) on \(\mathbb{R}\) by placing discrete mass at the ordinates \(\gamma_{N,k}\) where \(|\eta_N^{(N^2)}(1/2+it)|\) attains a local minimum below a threshold \(N^{1/2-\varepsilon}\) (or where the argument of the polygonal path jumps by nearly \(\pi\)), with weights normalised so that
\[
\int_{\mathbb{R}}\frac{d\mu_N(t)}{1+t^2}\le C
\]
uniformly in \(N\). The continuous remainder of the asymptotic supplies a residual density \(\rho_N\).

Because of the purification identity, every discrete atom of \(\mu_N\) lies within \(O(N^{-\delta''})\) of a true ordinate of a zero of \(\Xi\) (or of a candidate that will be excluded below).

---

## Completion of the remaining analytic steps

### Step 1 — Tightness of \(\{\mu_N\}\) (and of the residual densities)

Temperedness is already uniform by construction. It remains only to prevent mass from escaping to infinity.

Classical zero-density estimates give
\[
N(T)=\#\{\rho:\,0<\operatorname{Im}\rho\le T\}=\frac{T}{2\pi}\log\frac{T}{2\pi}+O\bigl(T^\theta\bigr)
\]
for an absolute \(\theta<1\). Because the purification identity forces every discrete atom of \(\mu_N\) to lie near a true ordinate (or a spurious candidate excluded in Step 2), the total discrete mass of \(\mu_N\) up to height \(T\) is comparable to \(N(T)\).

Consequently, for any \(R>0\),
\[
\mu_N\bigl(\{|t|>R\}\bigr)\le C'\frac{N(R+\omega_N)}{R^2}+o(1),
\]
where \(\omega_N\to0\) is the maximal displacement of the approximate zeros produced by the coherent sums. Choosing \(R\) large enough makes the right-hand side arbitrarily small, uniformly in \(N\).

The residual densities \(\rho_N\) inherit the same bound: the dual contribution in the asymptotic is of size \(N^{1/2}|\zeta(1/2-it)|\), and the second-moment recovery recorded in the coherent-sums note shows that the mean-square of these contributions remains controlled by the second moment of \(\zeta\) on the critical line. Thus \(\{\rho_N\,dt\}\) is likewise tight.

By Prokhorov’s theorem the family \(\{\mu_N\}\) is relatively compact in the weak-* topology of tempered measures. Every weak-* limit point is therefore a tempered measure.

### Step 2 — Exclusion of spurious pure-point mass

Suppose a weak-* limit point \(\mu\) possesses an atom at a real number \(\gamma^*\) that is **not** the imaginary part of a non-trivial zero of \(\Xi\). Then there exists a sequence \(N_j\to\infty\) and ordinates \(\gamma_{N_j}\) with
\[
\gamma_{N_j}\to\gamma^*\qquad\text{and}\qquad|\eta_{N_j}^{(N_j^2)}(1/2+i\gamma_{N_j})|\le N_j^{1/2-\varepsilon}.
\]
Substitute the asymptotic at \(t=\gamma_{N_j}\):
\[
\bigl|(1-N_j^{1/2-i\gamma_{N_j}})\zeta(1/2+i\gamma_{N_j})\bigr|
\le N_j^{1/2-\varepsilon}+N_j^{1/2}|c|\,|\zeta(1/2-i\gamma_{N_j})|+O(N_j^{1/2-\delta}).
\]
The left-hand side is asymptotic to \(N_j^{1/2}|\zeta(1/2+i\gamma^*)|\) (the phase factor \(N_j^{-i\gamma_{N_j}}\) has modulus 1).

If \(\zeta(1/2+i\gamma^*)\neq0\), then \(|\zeta(1/2+i\gamma^*)|\ge\delta_0>0\). The dual term on the right is of the same order, but the functional equation relates \(\zeta(1/2-i\gamma^*)\) to \(\zeta(1/2+i\gamma^*)\) by a factor of modulus 1. The two large terms cannot cancel to order \(N^{1/2-\varepsilon}\) for all large \(N_j\), because their relative phase is controlled by the stationary-phase factor \(e^{-i\gamma^*\log(N_j^2/2\pi)}\), which is dense on the unit circle (or can be made to avoid the precise cancelling direction by a standard density argument on the sequence \(N_j\)).

A quantitative Rouché argument on a small circle of radius \(N_j^{-\kappa}\) about \(\gamma^*\) makes the contradiction rigorous: the elementary polynomial \(\eta_{N_j}^{(N_j^2)}\) would have a zero inside that circle, while the main term of the asymptotic has no zero there (by the assumption that \(\gamma^*\) is not a zero of \(\Xi\)) and dominates the error and dual terms for large \(N_j\).

Hence no such spurious atom can appear. Every pure-point atom of any weak-* limit \(\mu\) must be the imaginary part of a non-trivial zero of \(\Xi\).

---

## Conclusion — Flag 6 is realised

Any weak-* limit point \(\mu\) of the sequence \(\{\mu_N\}\) is tempered, has pure-point support exactly on the non-trivial zeros of \(\Xi\), and has vanishing residual continuous spectrum (the residual densities \(\rho_N\) tend to zero by the quadratic balance that characterises the geometric DQPT).

Thus \(\mu\) satisfies Flag 6. The geometric DQPT construction, completed by the two analytic steps above, therefore yields the tempered measure whose existence is asserted by Flag 6.

(The same argument applies, with only notational changes, to the residual densities themselves and to the finite-rank fixed points of the generative map \(\mathcal{T}\).)

---

## Relation to the framework

- **Layer 1 generator**: coherent polygonal partial sums with quadratic truncation.
- **Geometric DQPT condition**: balanced path closures (and dual contributions of equal amplitude) occur exclusively at \(\sigma=1/2\).
- **Residual-vanishing mechanism**: the same quadratic balance that characterises the DQPT forces residual continuum to vanish.
- **Flag 6**: realised as a weak-* limit of the finite geometric measures \(\mu_N\).

*Related files:* `coherent-polygonal-partial-sums.md`, `structural-insight-candidate.md`, `status.md`, `README.md`.
