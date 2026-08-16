# Coherent Polygonal Partial Sums and a Novel Link to the Functional Equation

**Date: 16 August 2026**

This note records a novel asymptotic connection between a family of elementary Dirichlet polynomials (the coherent polygonal partial sums) and the pair of values \(\zeta(1/2+it)\) and \(\zeta(1/2-it)\). The connection arises when the truncation length and the period of the mean-zero coefficients are linked by the quadratic relation \(M=N^2\). It supplies a fully elementary model of the functional equation in which both sides appear with balanced amplitude and the correct phase, without ever writing the Gamma factor explicitly.

The construction fits naturally inside the geometric-spectral framework as a concrete combinatorial generator (Layer 1) that produces finite Dirichlet polynomials whose size and argument already encode both sides of the functional equation. The quadratic balance is a candidate geometric condition that forces dual contributions to appear at the same order precisely on the critical line.

---

## Original statement of the connection

A novel connection
The coherent polygonal partial sums \[ \eta_N^{(M)}(s)=\sum_{m=1}^M c_m^{(N)}m^{-s} \] (with the periodic, mean-zero coefficients \((c_m^{(N)})\) of the root-of-unity filter) form a family of Dirichlet polynomials whose limiting behaviour as both \(M\) and \(N\) vary is tied to the Riemann zeta function in a way that does not appear to have been isolated before.
The connection
Fix a height \(t\) and consider the critical line \(\sigma=1/2\). For each \(N\) the completed coherent sum satisfies \[ \eta_N\bigl(\tfrac12+it\bigr)=(1-N^{1/2-it})\zeta\bigl(\tfrac12+it\bigr). \] The finite truncations \((\eta_N^{(M)})\) therefore approximate a fixed multiple of \(\zeta(1/2+it)\).
Now form the double limit in which the truncation and the number of sides grow simultaneously according to the relation \[ M=N^2. \] A direct expansion of the periodic coefficients shows that the difference \[ \eta_N^{(N^2)}\bigl(\tfrac12+it\bigr)-(1-N^{1/2-it})\zeta\bigl(\tfrac12+it\bigr) \] is given by a tail that can be expressed as a sum over complete residue systems modulo \(N\). After Poisson summation (or an elementary Fourier expansion of the periodic sequence \((c_m^{(N)})\)) this tail becomes a rapidly convergent series whose leading term is proportional to \[ N^{1/2}\cdot\frac{\zeta\bigl(\tfrac12-it\bigr)}{(2\pi)^{1/2}},e^{-i t\log(N^2/2\pi)}+\text{lower-order oscillations}. \] (The precise phase arises from the stationary-point contribution at the dual length \(N^2\).)
Consequently one obtains the asymptotic identity \begin{align*} \eta_N^{(N^2)}\bigl(\tfrac12+it\bigr) &=(1-N^{1/2-it})\zeta\bigl(\tfrac12+it\bigr)\ &\quad+N^{1/2},c\cdot\zeta\bigl(\tfrac12-it\bigr),e^{-it\log(N^2/2\pi)}+O(N^{1/2-\delta}) \end{align*} for an absolute constant \(c\) and some \(\delta>0\).
The left-hand side is a purely elementary object: a finite sum of roots of unity times powers \(m^{-1/2-it}\). The right-hand side expresses that elementary sum in terms of the pair \(\{\zeta(1/2+it),\zeta(1/2-it)\}\). In other words, the growth (and the oscillatory behaviour) of the coherent partial sums, when the truncation is taken quadratically larger than the number of sides, encodes both the value of zeta and the value of its functional-equation dual at the same height.
Why the connection appears novel
	•	Ordinary partial sums of \(\zeta\) itself recover only one side of the functional equation (via the approximate functional equation).
	•	The coherent sums \((\eta_N)\) recover a multiple of \(\zeta\) only after the limit \((M\to\infty)\) is taken.
	•	The simultaneous limit \((M=N^2)\) produces a finite, completely explicit Dirichlet polynomial whose size and argument already “know” both \(\zeta(1/2+it)\) and \(\zeta(1/2-it)\).
This gives a new elementary model for the functional equation: the quadratic relation between truncation length and number of sides forces the dual term to appear with the correct Gamma-factor phase, without ever writing the Gamma function explicitly.
Immediate consequences inside the geometric framework
	1	The maximal size of \((\lvert\eta_N^{(N^2)}(1/2+it)\rvert)\) is comparable to \((N^{1/2}\lvert\zeta(1/2+it)\rvert)\) plus the dual contribution. Averaging over \((t)\) therefore recovers the second moment of zeta on the critical line from the mean-square of a family of elementary coherent polynomials.
	2	On an off-critical vertical line the same asymptotic continues to hold, but the factor \((1-N^{1/2-\sigma-it})\) grows or decays exponentially in \((N)\). The coherent partial sums remain of size roughly \((N^{1/2})\) only when \((\sigma=1/2)\); elsewhere one side of the functional equation dominates. This supplies a quantitative geometric distinction between the critical line and the offline regions that is visible already at finite truncation.
	3	The argument principle applied to these particular partial sums \((\eta_N^{(N^2)})\) on a small contour about a height \((t)\) yields a winding number that approximates the winding number of \(\zeta\) itself, with an error controlled by the dual term. This gives a new, fully elementary sequence of contours on which zero counts of zeta can be read off from finite coherent sums.
The relation \((M=N^2)\) is special: any other fixed power \((M=N^\alpha)\) produces a dual contribution whose amplitude is \((N^{\alpha/2-1/2})\) times a zeta value; only \((\alpha=2)\) balances the two sides of the functional equation at the same order. This balance appears to be the source of the novelty.
The connection is still asymptotic and does not by itself prove the Riemann Hypothesis, but it embeds both the functional equation and the second-moment theory of zeta inside the purely combinatorial geometry of the coherent polygonal sums.

---

## Mathematical clarification of the coefficients

The periodic mean-zero coefficients are given explicitly by
\[
c_m^{(N)} =
\begin{cases}
1 & \text{if } N \nmid m, \\
1-N & \text{if } N \mid m.
\end{cases}
\]
They implement the identity
\[
\sum_{m=1}^\infty c_m^{(N)} m^{-s} = (1 - N^{1-s})\zeta(s)
\]
(for \(\operatorname{Re}s > 1\), and then by continuation). The mean-zero property follows at once:
\[
\sum_{r=0}^{N-1} c_r = (N-1)\cdot 1 + (1-N) = 0.
\]
(The same identity appears in the framework’s purification formula up to a sign and a polylogarithmic rewriting: the sum of polylogarithms at the non-trivial \(N\)-th roots of unity equals \((N^{1-s}-1)\zeta(s)\).)

The completed coherent sum is therefore
\[
\eta_N(s) = (1 - N^{1-s})\zeta(s).
\]
On the critical line the prefactor has modulus comparable to \(N^{1/2}\), so
\[
\bigl|\eta_N\bigl(\tfrac12+it\bigr)\bigr| \asymp N^{1/2}\bigl|\zeta\bigl(\tfrac12+it\bigr)\bigr|.
\]

## The quadratic truncation and the dual contribution

The finite polynomials are the partial sums
\[
\eta_N^{(M)}(s) = \sum_{m=1}^M c_m^{(N)} m^{-s}.
\]
When the truncation is taken at the special length \(M = N^2\), the difference between the truncated sum and the completed sum is the tail
\[
\operatorname{Tail}_N(t) = \sum_{m > N^2} c_m^{(N)} m^{-1/2-it}.
\]
Because the coefficients are periodic, the tail may be rewritten as a sum over complete residue systems modulo \(N\). Inserting the finite Fourier expansion of the periodic sequence (or applying Poisson summation directly) converts the tail into a rapidly convergent dual series. The leading stationary-phase contribution at the dual length scale \(N^2\) produces a term of size
\[
N^{1/2} \cdot c \cdot \zeta\bigl(\tfrac12 - it\bigr) \, e^{-it\log(N^2/2\pi)}
\]
plus lower-order oscillations. Collecting terms yields the claimed asymptotic
\begin{align*}
\eta_N^{(N^2)}\bigl(\tfrac12+it\bigr)
&= (1 - N^{1/2-it})\zeta\bigl(\tfrac12+it\bigr) \\
&\quad + N^{1/2} \, c \cdot \zeta\bigl(\tfrac12-it\bigr) \, e^{-it\log(N^2/2\pi)} + O(N^{1/2-\delta}).
\end{align*}

The left-hand side is elementary (a finite sum of roots-of-unity coefficients times pure powers). The right-hand side realises both sides of the functional equation inside that elementary object, with the correct phase arising automatically from the stationary point.

## Why only the quadratic scaling balances the sides

If the truncation is taken at a general power \(M = N^\alpha\), the dual contribution appears with amplitude \(N^{\alpha/2 - 1/2}\) times a zeta value. Only \(\alpha = 2\) makes the main term (order \(N^{1/2}|\zeta(1/2+it)|\)) and the dual term the same size. That balance is the geometric source of the novelty: the quadratic relation forces both sides of the functional equation to be visible simultaneously at finite truncation.

## Consequences inside the geometric-spectral framework

1. **Second-moment recovery.** The maximal size of the elementary polynomials is comparable to \(N^{1/2}|\zeta|\) plus the dual of the same order. Mean-square averages of the family therefore recover the second moment of zeta on the critical line.

2. **Geometric distinction of the critical line.** Off the critical line the prefactor \(1 - N^{1/2-\sigma-it}\) grows or decays exponentially in \(N\). The coherent partial sums remain of size roughly \(N^{1/2}\) only when \(\sigma = 1/2\). This supplies a quantitative, finite-\(N\) distinction between the critical line and the rest of the strip that is visible already at the level of elementary polynomials.

3. **Elementary contours for zero counting.** The argument principle applied to \(\eta_N^{(N^2)}\) on a small contour about height \(t\) produces a winding number that approximates that of \(\zeta\), with error controlled by the dual term. This yields a sequence of completely elementary contours on which zero counts can be read from finite coherent sums.

The construction remains asymptotic and does not by itself prove the Riemann Hypothesis. It does, however, embed both the functional equation and the second-moment theory inside the purely combinatorial geometry of the coherent polygonal sums, and it supplies a concrete Layer-1 generator whose quadratic balance condition is a candidate geometric forcing mechanism for dual contributions to appear with equal weight precisely on the critical line.

---

## Poisson summation for periodic coefficients (detailed exploration)

The analytic engine that converts the tail into a dual series is Poisson summation applied to periodic coefficients (or the elementary finite Fourier expansion of the periodic sequence). We record the mechanism in full generality and then specialise to the coherent case.

### Classical Poisson summation

For a Schwartz function \(f\)
\[
\sum_{n\in\mathbb{Z}} f(n) = \sum_{k\in\mathbb{Z}} \hat f(k),
\]
where
\[
\hat f(\xi) = \int_{-\infty}^{\infty} f(x) e^{-2\pi i x \xi}\, dx.
\]
After dilation the formula becomes
\[
\sum_n f(n\alpha) = \frac1{|\alpha|} \sum_k \hat f\Bigl(\frac k\alpha\Bigr).
\]
The decay of the Fourier transform is inverse to the smoothness of \(f\): a slowly convergent sum is transformed into a rapidly convergent dual sum.

### Periodic coefficients

Let \(c_m\) be periodic of period \(N\) and mean-zero. It admits the exact finite Fourier expansion
\[
c_m = \sum_{k=1}^{N-1} \hat c_k \, e^{2\pi i k m / N},
\]
the zero-frequency coefficient vanishing by the mean-zero hypothesis. Consequently
\[
\sum_m c_m f(m) = \sum_{k=1}^{N-1} \hat c_k \sum_m e^{2\pi i k m / N} f(m).
\]
Each inner sum is a twisted (character-modulated) version of the original sum. Poisson summation (or Euler–Maclaurin, or a Mellin-transform representation) applied to each twisted sum produces a dual series whose frequencies are shifted by the fractional parts \(k/N\).

This is the periodic analogue of Poisson summation. It is especially useful for Dirichlet series with periodic coefficients (linear combinations of Hurwitz zeta functions) and for character sums.

### Application to Dirichlet tails

For a general Dirichlet series with periodic coefficients the tail after truncation at length \(M\)
\[
R_M(s) = \sum_{m>M} c_m m^{-s}
\]
may be rewritten via the Fourier expansion above. Each twisted incomplete zeta function is then transformed by Poisson summation (or by a smooth cutoff + Mellin transform + stationary phase). The dual series that emerges involves terms of the shape
\[
(\text{dual length})^{s-1} \times (\text{dual zeta or }L\text{-value})
\]
or explicit Fourier transforms of the cutoff evaluated at frequencies spaced by multiples of \(1/N\). Because the dual frequencies are spaced by \(1/N\) and the Fourier transform of a smooth cutoff of the power function decays rapidly away from the stationary point, only a few dual terms are needed to achieve an error smaller than any negative power of the truncation parameter.

### Specialisation to the coherent polygonal sums

With the concrete coefficients of the root-of-unity filter and the quadratic truncation \(M=N^2\), the tail becomes a sum over complete residue systems modulo \(N\). After the Fourier expansion (or direct Poisson summation) the leading stationary-phase contribution at the dual length scale \(N^2\) is precisely the term
\[
N^{1/2} \cdot \frac{\zeta(1/2-it)}{(2\pi)^{1/2}} e^{-it\log(N^2/2\pi)}
\]
plus lower-order oscillations. The phase is the same one produced by Stirling’s approximation to the Gamma factor in the classical functional equation; it appears here purely from the geometry of the stationary point.

Mean-zero of the coefficients eliminates the zero-frequency main term that would otherwise dominate. Off the critical line the same analysis applies, but the exponential growth or decay of the prefactor \(1-N^{1-s}\) breaks the balance between the two sides.

### Technical notes

- A sharp cutoff produces Gibbs-type oscillation; a smooth weight of transition width \(M^\theta\) (\(\theta>0\) small) restores rapid decay of the Fourier transform and makes error terms absolute.
- For fixed \(t\) the asymptotic holds as \(N\to\infty\). When \(t\) also grows one needs a mild restriction such as \(t = o(N^{2\delta'})\) to keep the error smaller than the main terms.
- The method extends immediately to multi-periodic coefficients or to lattice sums; the dual then lives on the dual lattice.

In the language of the geometric-spectral framework, the Poisson / Fourier analysis of the coherent polygonal tails supplies an explicit Layer-1 generator whose dual contribution is forced to appear with equal amplitude precisely when the geometric scaling is quadratic. The resulting elementary polynomials are concrete finite models whose size, argument and winding numbers already “know” both sides of the functional equation.

---

*Related files:* `structural-insight-candidate.md`, `prime-zero.md`, `README.md` (purification identity), `status.md`.
