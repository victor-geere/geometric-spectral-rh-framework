//! Tempered measure data structures for Flag 6 (native infinity form).
//!
//! These types approximate tempered positive measures on ℝ.
//! Exact residual vanishing and exact pure-point support equality
//! remain analytic statements; the code only supplies numerical checks
//! and certificate hooks.

use std::f64::consts::PI;

/// Weight used for temperedness: 1/(1+t²).
pub fn temper_weight(t: f64) -> f64 {
    1.0 / (1.0 + t * t)
}

/// Pure-point spectrum: locations (imaginary parts on the critical line) and masses.
#[derive(Clone, Debug, Default)]
pub struct PurePointSpectrum {
    pub locations: Vec<f64>,
    pub masses: Vec<f64>,
}

impl PurePointSpectrum {
    pub fn support(&self) -> &[f64] {
        &self.locations
    }

    pub fn total_mass(&self) -> f64 {
        self.masses.iter().sum()
    }

    pub fn truncate_to_height(&self, height: f64) -> PurePointSpectrum {
        let mut locations = Vec::new();
        let mut masses = Vec::new();
        for (loc, mass) in self.locations.iter().zip(self.masses.iter()) {
            if loc.abs() <= height {
                locations.push(*loc);
                masses.push(*mass);
            }
        }
        PurePointSpectrum { locations, masses }
    }

    /// Hausdorff-style distance to a set of known ordinates (finite height).
    pub fn distance_to_known_zeros(&self, known: &[f64]) -> f64 {
        if self.locations.is_empty() && known.is_empty() {
            return 0.0;
        }
        let mut max_dist = 0.0_f64;
        for loc in &self.locations {
            let d = known
                .iter()
                .map(|k| (loc - k).abs())
                .fold(f64::INFINITY, f64::min);
            max_dist = max_dist.max(d);
        }
        for k in known {
            let d = self
                .locations
                .iter()
                .map(|loc| (loc - k).abs())
                .fold(f64::INFINITY, f64::min);
            max_dist = max_dist.max(d);
        }
        max_dist
    }
}

/// Absolutely continuous density (residual continuum).
#[derive(Clone, Debug)]
pub struct ContinuousDensity {
    /// Sample points and density values (simple piecewise representation).
    pub samples: Vec<(f64, f64)>,
}

impl ContinuousDensity {
    pub fn residual_mass(&self, weight: fn(f64) -> f64) -> f64 {
        // Trapezoidal rule against the temper weight.
        if self.samples.len() < 2 {
            return 0.0;
        }
        let mut mass = 0.0;
        for w in self.samples.windows(2) {
            let (t0, r0) = w[0];
            let (t1, r1) = w[1];
            let dt = t1 - t0;
            let avg = 0.5 * (r0 * weight(t0) + r1 * weight(t1));
            mass += avg * dt;
        }
        mass.abs()
    }

    pub fn is_numerically_vanishing(&self, tol: f64) -> bool {
        self.residual_mass(temper_weight) < tol
    }
}

/// Core trait for tempered measures appearing in Flag 6.
pub trait TemperedMeasure {
    fn pure_point_part(&self) -> &PurePointSpectrum;
    fn absolutely_continuous_density(&self) -> Option<&ContinuousDensity>;
    fn total_mass_against_weight(&self, weight: fn(f64) -> f64) -> f64;
    fn is_tempered(&self) -> bool {
        self.total_mass_against_weight(temper_weight).is_finite()
            && self.total_mass_against_weight(temper_weight) < f64::INFINITY
    }
}

/// Finite atomic pure-point measure (no residual continuum).
#[derive(Clone, Debug, Default)]
pub struct AtomicMeasure {
    pub pure_point: PurePointSpectrum,
}

impl TemperedMeasure for AtomicMeasure {
    fn pure_point_part(&self) -> &PurePointSpectrum {
        &self.pure_point
    }

    fn absolutely_continuous_density(&self) -> Option<&ContinuousDensity> {
        None
    }

    fn total_mass_against_weight(&self, weight: fn(f64) -> f64) -> f64 {
        self.pure_point
            .locations
            .iter()
            .zip(self.pure_point.masses.iter())
            .map(|(t, m)| m * weight(*t))
            .sum()
    }
}

/// Mixed measure: pure-point plus optional absolutely continuous residual.
#[derive(Clone, Debug)]
pub struct MixedMeasure {
    pub pure_point: PurePointSpectrum,
    pub residual: Option<ContinuousDensity>,
}

impl TemperedMeasure for MixedMeasure {
    fn pure_point_part(&self) -> &PurePointSpectrum {
        &self.pure_point
    }

    fn absolutely_continuous_density(&self) -> Option<&ContinuousDensity> {
        self.residual.as_ref()
    }

    fn total_mass_against_weight(&self, weight: fn(f64) -> f64) -> f64 {
        let pp: f64 = self
            .pure_point
            .locations
            .iter()
            .zip(self.pure_point.masses.iter())
            .map(|(t, m)| m * weight(*t))
            .sum();
        let ac = self
            .residual
            .as_ref()
            .map(|d| d.residual_mass(weight))
            .unwrap_or(0.0);
        pp + ac
    }
}

/// Build an atomic measure from the first K known zeros (unit masses).
pub fn from_known_zeros(zeros: &[f64]) -> AtomicMeasure {
    AtomicMeasure {
        pure_point: PurePointSpectrum {
            locations: zeros.to_vec(),
            masses: vec![1.0; zeros.len()],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_is_tempered() {
        let mu = from_known_zeros(&[14.134725, 21.022040, 25.010858]);
        assert!(mu.is_tempered());
        assert!(mu.absolutely_continuous_density().is_none());
    }

    #[test]
    fn residual_mass_positive() {
        let dens = ContinuousDensity {
            samples: vec![(-10.0, 0.1), (0.0, 0.2), (10.0, 0.1)],
        };
        assert!(dens.residual_mass(temper_weight) > 0.0);
    }
}
