//! Flag 6 native certificate hooks and orchestrator skeleton.
//!
//! Flag 6 (native): existence of a tempered measure μ whose pure-point
//! support is exactly the non-trivial zeros of Ξ and whose residual
//! continuous part vanishes identically.
//!
//! Only the Analytic variants of the proof enums discharge Flag 6.
//! Numerical / FiniteHeight variants are computational evidence only.

use crate::tempered_measure::{TemperedMeasure, PurePointSpectrum, ContinuousDensity};

/// Proof status for pure-point support matching the full zero set.
#[derive(Clone, Debug)]
pub enum SupportProof {
    /// Analytic argument that support equals the full set of non-trivial zeros.
    Analytic { reference: String },
    /// Only verified up to a finite height.
    FiniteHeight { height: f64 },
    /// No proof available.
    Absent,
}

/// Proof status for residual vanishing.
#[derive(Clone, Debug)]
pub enum ResidualProof {
    /// Analytic argument that residual mass is identically zero.
    Analytic { reference: String },
    /// Numerical evidence below a tolerance.
    Numerical { tol: f64 },
    /// No proof available.
    Absent,
}

/// Certificate that would discharge Flag 6 when both proofs are Analytic.
pub trait Flag6Certificate {
    fn measure(&self) -> &dyn TemperedMeasure;
    fn pure_point_support_exact(&self) -> SupportProof;
    fn residual_vanishes_identically(&self) -> ResidualProof;

    /// True only when both proofs are Analytic.
    fn captures_flag6(&self) -> bool {
        matches!(self.pure_point_support_exact(), SupportProof::Analytic { .. })
            && matches!(self.residual_vanishes_identically(), ResidualProof::Analytic { .. })
    }
}

/// Report produced by support matching at finite height.
#[derive(Clone, Debug)]
pub struct SupportMatchReport {
    pub height: f64,
    pub hausdorff_distance: f64,
    pub missing_zeros: usize,
    pub spurious_locations: usize,
}

/// Compare pure-point support of a measure against known zeros up to a height.
pub fn match_pure_point_support(
    mu: &dyn TemperedMeasure,
    known_zeros: &[f64],
    height: f64,
    tol: f64,
) -> SupportMatchReport {
    let pp = mu.pure_point_part().truncate_to_height(height);
    let known: Vec<f64> = known_zeros
        .iter()
        .copied()
        .filter(|z| z.abs() <= height)
        .collect();

    let hausdorff = pp.distance_to_known_zeros(&known);

    let mut missing = 0;
    for k in &known {
        let found = pp.locations.iter().any(|loc| (loc - k).abs() < tol);
        if !found {
            missing += 1;
        }
    }

    let mut spurious = 0;
    for loc in &pp.locations {
        let found = known.iter().any(|k| (loc - k).abs() < tol);
        if !found {
            spurious += 1;
        }
    }

    SupportMatchReport {
        height,
        hausdorff_distance: hausdorff,
        missing_zeros: missing,
        spurious_locations: spurious,
    }
}

/// Numerical residual-vanishing check.
pub fn residual_vanishes_numerically(mu: &dyn TemperedMeasure, tol: f64) -> bool {
    match mu.absolutely_continuous_density() {
        None => true,
        Some(dens) => dens.is_numerically_vanishing(tol),
    }
}

/// Skeleton orchestrator for Flag 6 native checks.
pub struct Flag6NativeOrchestrator {
    pub known_zeros: Vec<f64>,
    pub numerical_tol: f64,
}

impl Flag6NativeOrchestrator {
    pub fn new(known_zeros: Vec<f64>, numerical_tol: f64) -> Self {
        Self {
            known_zeros,
            numerical_tol,
        }
    }

    pub fn run_support_check(
        &self,
        mu: &dyn TemperedMeasure,
        height: f64,
    ) -> SupportMatchReport {
        match_pure_point_support(mu, &self.known_zeros, height, self.numerical_tol)
    }

    pub fn run_residual_check(&self, mu: &dyn TemperedMeasure) -> bool {
        residual_vanishes_numerically(mu, self.numerical_tol)
    }

    /// Structured status report. Never claims RH is proved unless certificate captures Flag 6.
    pub fn report(&self, cert: Option<&dyn Flag6Certificate>, mu: &dyn TemperedMeasure, height: f64) -> String {
        let support = self.run_support_check(mu, height);
        let residual_ok = self.run_residual_check(mu);

        let mut out = String::new();
        out.push_str("=== Flag 6 Native Status Report ===\n");
        out.push_str(&format!("Support check up to height {:.3}:\n", height));
        out.push_str(&format!("  Hausdorff distance : {:.6e}\n", support.hausdorff_distance));
        out.push_str(&format!("  Missing zeros      : {}\n", support.missing_zeros));
        out.push_str(&format!("  Spurious locations : {}\n", support.spurious_locations));
        out.push_str(&format!("Residual numerically vanishing (tol={:.1e}): {}\n", self.numerical_tol, residual_ok));

        match cert {
            Some(c) if c.captures_flag6() => {
                out.push_str("Flag 6 certificate: ANALYTIC — existence claim discharged.\n");
            }
            Some(_) => {
                out.push_str("Flag 6 certificate: present but not fully Analytic.\n");
                out.push_str("Flag 6 remains open.\n");
            }
            None => {
                out.push_str("No Flag 6 certificate registered.\n");
                out.push_str("Flag 6 remains open.\n");
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tempered_measure::from_known_zeros;

    #[test]
    fn pure_atomic_has_vanishing_residual() {
        let mu = from_known_zeros(&[14.134725, 21.022040]);
        assert!(residual_vanishes_numerically(&mu, 1e-12));
    }
}
