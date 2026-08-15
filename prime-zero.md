# Prime–Zero Duality in the Geometric Framework

**Research note — formal statements, geometric open questions, status, and solution pathways**

This note unifies the arithmetic (prime) side and the geometric (zero) side of the Guinand–Weil explicit formula inside the rotation-space / coherent-sum framework. It records the precise duality statements, rewrites the remaining open problems as geometric claims about the atlas and coherent interference, summarises current status, and outlines concrete approaches that start from the already-constructed objects.

---

## 1. Zeros versus Primes — the formal statements

### 1.1 Classical duality

Let $\mathcal{S}$ be a suitable space of even test functions $g$ whose Fourier transforms $h=\widehat{g}$ satisfy the growth conditions of the Guinand–Weil formula. The two sides of the explicit formula are the continuous linear functionals

**Zero side**
$$
\mathcal{W}_{\mathrm{Z}}(h)=\sum_{\rho}m_{\rho}\,h(\gamma_{\rho})=\int_{\mathbb{R}}h\,d\mu_{\mathrm{Z}},
$$
where $\mu_{\mathrm{Z}}=\sum_{\rho}m_{\rho}\,\delta_{\gamma_{\rho}}$ is the discrete spectral measure of the ordinates of the non-trivial zeros of $\zeta$.

**Prime side**
$$
\mathcal{W}_{\mathrm{ar}}(g)=\operatorname{Tr}\bigl(\mathbb{W}^{1/2}\,g(\mathbb{A})\,\mathbb{W}^{1/2}\bigr)
=\sum_{n\ge2}\frac{\Lambda(n)}{\sqrt{n}}\bigl(g(\log n)+g(-\log n)\bigr)
$$
(plus the explicit archimedean contribution $\operatorname{Arch}(g)$).

The identity asserts
$$
\mathcal{W}_{\mathrm{Z}}(h)+\mathcal{W}_{\mathrm{ar}}(g)=\operatorname{Arch}(g).
$$

In the geometric-spectral framework the prime side has been realised unconditionally as an operator trace on a von-Mangoldt-weighted $\ell^2$ space (see `prime-side.md`). The zero side is recovered as the distributional dual that makes the identity hold.

### 1.2 Geometric realisation of the zero side

Each single-rotation polylogarithm $f_{\theta}(s)=\operatorname{Li}_{s}(e^{i\theta})$ carries its own spectral measure $\mu_{\theta}$ of zeros. The coherent sum over the nontrivial $N$-gon rotations
$$
\eta_{N}(s)=\sum_{k=1}^{N-1}f_{2\pi k/N}(s)=(1-N^{1-s})\zeta(s)
$$
induces the coherent spectral measures
$$
\mu_{N}=\sum_{k=1}^{N-1}\mu_{2\pi k/N}.
$$
After normalisation these converge distributionally to $\mu_{\mathrm{Z}}$ inside the critical strip: the off-critical mass cancels by the root-of-unity filter and the surviving mass coincides with the zeros of $\zeta$. Equivalently, the coherent logarithmic derivatives recover the Stieltjes transform of $\mu_{\mathrm{Z}}$:
$$
\lim_{N\to\infty}\sum_{k=1}^{N-1}\frac{f'_{2\pi k/N}}{f_{2\pi k/N}}(s)
=\frac{\zeta'}{\zeta}(s)+\frac{N^{1-s}\log N}{1-N^{1-s}}+O(1).
$$

Thus the zero side is no longer abstract: it is the coherent limit of rotation-space spectra.

### 1.3 Unified identity

Combining the two realisations yields the hybrid explicit formula
$$
\lim_{N\to\infty}\sum_{k=1}^{N-1}\sum_{\substack{z\in Z(f_{2\pi k/N})\\\frac12<\operatorname{Re}z<1}}m_{z}\,h(\operatorname{Im}z)
=\operatorname{Arch}(g)-\operatorname{Tr}\bigl(\mathbb{W}^{1/2}\,g(\mathbb{A})\,\mathbb{W}^{1/2}\bigr).
$$
The left-hand side is geometric (coherent atlas data); the right-hand side is arithmetic (operator trace already constructed).

---

## 2. Geometric formulation of the open questions

After the prime-side operator and the coherent-sum model of $\mu_{\mathrm{Z}}$ are in place, two classical problems remain. Inside the geometric framework they become statements about cancellation and quantisation on the space of angles.

### 2.1 Support on the critical line (Riemann Hypothesis)

**Geometric statement.**  
Does the coherent interference annihilate every off-critical mass in the rotation-space atlas?

- For every fixed rectangular window $W$ with $\operatorname{Re}s\neq\frac12$ inside the critical strip, does the certified argument-principle count of $\eta_{N}$ around $\partial W$ tend to zero as $N\to\infty$?
- Equivalently, after all destructive interference among the individual measures $\mu_{\theta}$, is
  $$
  \operatorname{supp}(\mu_{\mathrm{Z}})\subset\bigl\{\sigma=\tfrac12\bigr\}?
  $$

The individual atlases exhibit abundant zeros with $\sigma>1/2$ (including zeros in the absolute-convergence region). The coherent sum is the geometric mechanism conjectured to erase them completely. The question is whether the observed cancellation is exhaustive at every finite height.

### 2.2 Existence of a geometric self-adjoint operator (Hilbert–Pólya)

**Geometric statement.**  
Does there exist a self-adjoint operator $H_{\mathrm{geo}}$ constructed from the rotation-space data (cells, certified counts, turn-drift geometry, root-of-unity filter) whose spectral measure coincides with the coherent limit $\mu_{\mathrm{Z}}$?

- Can the atlas and the coherent interference pattern be quantised so that the eigenvalues of the resulting operator are precisely the ordinates of the surviving critical zeros?
- Equivalently, does a geometric kernel or Fredholm determinant built from the family $\{f_{\theta}\}$ or from the coherent sums $\eta_{N}$ admit a self-adjoint realisation supported exactly on the critical line?

Any such operator automatically forces real spectrum, so existence implies the support statement. The converse is not automatic: a discrete real set arising as a coherent limit need not be the spectrum of a natural geometric operator attached to the atlas.

### 2.3 Unified geometric claim

After coherent summation over all nontrivial $N$-gon rotations, the residual spectral mass of the atlas is supported exactly on the critical line, and that residual mass is the spectrum of a self-adjoint geometric operator constructed from the atlas data.

---

## 3. Status of the problems

| Object / Statement | Status | Notes |
|---|---|---|
| Prime-side operator trace $\mathcal{W}_{\mathrm{ar}}$ | **Unconditional / proven** | Constructed in `prime-side.md`; kernel, flow, generator, boundary law all rigorous |
| Classical Guinand–Weil identity | **Unconditional / proven** | Holds once the prime side is realised |
| Coherent recovery of zero locations | **Unconditional / proven** | Root-of-unity filter + analytic continuation of $\eta_{N}$ |
| Coherent recovery of spectral measure / residues | **Unconditional / proven** | Logarithmic derivatives of the coherent sum; argument-principle certification |
| Support of $\mu_{\mathrm{Z}}$ on the critical line | **Open = RH** | Geometric cancellation may be incomplete |
| Existence of geometric self-adjoint $H_{\mathrm{geo}}$ | **Open = Hilbert–Pólya** | Stronger structural demand; implies RH |
| Weil positivity of the zero-side functional | **RH-equivalent** | Equivalent to both open questions via classical criteria |
| Numerical cancellation in finite atlases | **Strongly supported** | Observed to high precision; not a proof |

The framework therefore renders both the arithmetic and the geometric sides of the explicit formula fully explicit and unconditional. The only remaining questions are the two classical ones, now expressed as geometric claims about total cancellation and quantisability of the atlas.

---

## 4. Approaches to solutions from the current framework

The existing objects—rotation-space atlas, coherent sums, certified cells, turn-drift geometry, and the prime-side operator—supply concrete levers.

### 4.1 Strengthening the cancellation (attack on support / RH)

1. **High-precision coherent atlas sweeps**  
   Extend the existing atlas machinery to larger $N$, higher $t$, and thinner cells. Certify the total coherent mass in every off-critical cell via series-validated argument principle. Accumulate quantitative bounds on residual off-line mass as a function of $N$ and height.

2. **Uniform height estimates for the first off-line zero**  
   Track the lowest zero of $f_{\theta}$ with $\sigma>1/2$ as $\theta\to\pi$. If this height tends to infinity sufficiently fast and uniformly, the coherent sum cannot leave residual mass at finite height.

3. **Positivity of geometric kernels**  
   Form correlation kernels or Fredholm determinants from the family $\{f_{\theta}\}$ (or from the coherent logarithmic derivatives). Prove (or numerically certify to high order) that these kernels remain positive-semidefinite precisely when the coherent mass is supported on the critical line. This converts the support question into a positivity statement already native to the framework.

4. **Link to greedy-harmonic / spectral-triple positivity**  
   Import the correlation-kernel technology of the greedy-harmonic decomposition. Show that the coherent rotation kernel is a limit case of those kernels; transfer known positivity or eigenvalue-suppression results.

### 4.2 Constructing a geometric operator (attack on Hilbert–Pólya)

1. **Quantisation of the atlas**  
   Treat the certified cells and their winding numbers as the combinatorial skeleton of a Hilbert space. Define a geometric transfer operator or a quantised turn-drift operator whose matrix elements are built from the coherent sums. Prove self-adjointness and identify its spectrum with the coherent limit measure.

2. **Spectral triple from rotation space**  
   Assemble a spectral triple whose Dirac operator is assembled from the coherent logarithmic derivatives or from the root-of-unity filter. Show that the associated spectral measure recovers $\mu_{\mathrm{Z}}$ and that the triple is even / real in the required sense.

3. **Boundary regularity of the coherent flow character**  
   Form the analogue of the prime-side flow character using coherent sums instead of von-Mangoldt weights. Prove that this character continues to depth exactly $1/2$ if and only if the coherent mass is real, and that the resulting boundary operator is self-adjoint.

4. **Hybrid arithmetic–geometric operator**  
   Couple the already-constructed prime-side generator $\mathbb{A}$ to a geometric operator built from the atlas via the explicit formula. Seek a self-adjoint extension whose spectrum on the zero side matches the coherent limit.

### 4.3 Numerical and certification pathway (near-term)

- Produce count-complete coherent atlases for successive $N$ up to several hundred and $t$ up to several thousand.
- Extract empirical spectral form factors from the residual critical mass and compare with GUE.
- Compute the coherent logarithmic derivative on vertical lines and verify residue recovery of known zeros to high precision.
- Test positivity of finite sections of geometric kernels derived from the atlas; look for eigenvalue suppression exactly at the ordinates of known zeros.

These computations stay entirely inside the existing codebase (polylog evaluation, argument-principle certification, atlas tiling) and convert the geometric open questions into a sequence of rigorously certifiable numerical statements that can later be turned into analytic bounds.

---

## 5. Summary

The prime side is an unconditional operator trace. The zero side is the coherent limit of rotation-space spectral measures. Their duality is the classical explicit formula, now realised geometrically on one side and arithmetically on the other. The only open questions are whether the coherent cancellation is total (support on the critical line = RH) and whether the residual critical mass is the spectrum of a self-adjoint geometric operator built from the atlas (Hilbert–Pólya). Both questions admit direct attacks that begin from the already-constructed atlas, coherent sums, and prime-side operator; the pathways above convert them into concrete analytic, geometric, and computational tasks inside the present framework.

*End of prime-zero.md.*
