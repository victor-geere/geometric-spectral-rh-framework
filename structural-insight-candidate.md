# Structural Insight Candidate

**Attempt at a non-circular algebraic/analytic reason that could force coherent cancellation onto the critical line**

This note records one concrete attempt to supply the kind of structural insight that is still missing from the pathways listed in `prime-zero.md`. The goal is an identity or rigidity statement that forces the residual spectral mass of the coherent sums to lie on the critical line *without* presupposing the Riemann Hypothesis or merely reformulating the classical explicit formula.

---

## Candidate construction

Consider the family of functions on the circle of angles
$$
F(\theta;s) := \operatorname{Li}_s(e^{i\theta}) = \sum_{m=1}^\infty m^{-s}e^{im\theta},\qquad \theta\in[0,2\pi).
$$
For fixed $s=\sigma+it$ with $\sigma>1$ this is an ordinary Fourier series. The coherent sum $\eta_N(s)$ is precisely the projection of $F(\,\cdot\,;s)$ onto the characters that are multiples of $N$ (root-of-unity filter).

Now form the $L^2$-inner product on the circle against a fixed positive test density $\phi(\theta)\ge 0$:
$$
K_\phi(s,s') := \frac1{2\pi}\int_0^{2\pi} F(\theta;s)\,\overline{F(\theta;\overline{s}')}\,\phi(\theta)\,d\theta.
$$
By orthogonality of the characters one obtains the explicit diagonalisation
$$
K_\phi(s,s') = \sum_{m=1}^\infty m^{-(s+\overline{s}')}\widehat\phi(m),
$$
where $\widehat\phi(m)$ are the Fourier coefficients of $\phi$. When $\phi$ is a Fejér or Poisson kernel concentrated near the $N$-th roots of unity, $K_\phi$ becomes a smoothed version of the coherent sum of the individual Gram kernels $f_\theta(s)\overline{f_\theta(\overline{s}')}$.

**Candidate claim.**  
If one can exhibit a positive test density $\phi$ (or a sequence of them) such that the associated integral operator with kernel $K_\phi(s,s')$ on a suitable vertical line or strip is positive-definite *independently of any zero-free information*, and if the only measures compatible with both this positivity and the already-constructed prime-side trace are those supported on $\operatorname{Re}s=1/2$, then the coherent cancellation would be forced by the positivity of $K_\phi$ rather than by an a-priori appeal to the closed form $\eta_N=(1-N^{1-s})\zeta$.

In other words, the geometric average over angles of the individual rotation kernels would itself supply a positive-definite object whose spectral support is rigid.

---

## Why the candidate looks promising at first glance

- It stays entirely inside the rotation-space geometry already present in the framework.
- The diagonalisation is elementary (Fourier analysis on the circle) and does not invoke RH.
- Positivity of kernels built from Dirichlet series with non-negative coefficients is classical and unconditional for $\sigma>1/2$ in many cases.
- It converts the geometric cancellation into a question about uniqueness of spectral measures compatible with a positive kernel plus the prime-side operator, which is the sort of rigidity that could close a gap.

---

## Critical evaluation — why it does not yet close the gap

1. **The positivity is too weak.**  
   For the natural choices of $\phi$ the kernel $K_\phi$ is positive-definite on $\operatorname{Re}s>1/2$ by the same reason that $\zeta(2\sigma)>0$, but this does not constrain the *zeros* of the individual $f_\theta$ or of their coherent sum. It only controls average size.

2. **Compatibility with the prime-side trace still requires the classical identity.**  
   To force the support onto the critical line one still needs a uniqueness argument that identifies the measure with $\mu_{\mathrm{Z}}$. That identification currently runs through the explicit formula or the closed form of $\eta_N$, both of which already know that the zeros are those of $\zeta$. The argument therefore remains circular with respect to RH.

3. **No new algebraic relation appears.**  
   The Fourier diagonalisation is the ordinary root-of-unity filter written in continuous language. It does not produce a factorisation of the form “positive square + remainder that vanishes for algebraic reasons independent of RH.”

4. **Davenport–Heilbronn counter-examples survive.**  
   Linear combinations of $L$-functions (precisely the individual $f_\theta$ for rational $\theta$) are known to possess zeros with $\operatorname{Re}s>1/2$. Any kernel built by averaging them will inherit the possibility of off-line mass unless an additional rigidity that kills those zeros is supplied. The candidate does not supply that rigidity.

---

## Conclusion of the attempt

The construction yields a geometrically natural positive kernel associated with the rotation space, and it makes the coherent sum appear as a projection of that kernel. This is a modest organisational improvement and could be useful for numerical or spectral-triple purposes. It does **not**, however, furnish an independent algebraic or analytic reason that forces the residual mass onto the critical line. The missing ingredient remains a rigidity or uniqueness theorem that is stronger than the classical explicit formula and that does not presuppose the conclusion.

Further attempts would need to locate a genuinely new relation — for example an unexpected differential equation satisfied by the coherent density, a representation-theoretic interpretation in which unitarity forces the critical line, or a modular constraint on the turn-drift phases that is invisible in the ordinary Dirichlet series. No such relation is visible from the present geometric data.

---

*Related files:* `prime-zero.md`, `prime-side.md`, `zero-side.md`.
