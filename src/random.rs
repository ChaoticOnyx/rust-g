#![allow(non_snake_case)]

use crate::error::Error;
use paste::paste;
use rand::{
    distributions::{uniform::SampleUniform, Bernoulli, Uniform},
    Rng, SeedableRng,
};
use rand_chacha::ChaCha8Rng;
use rand_distr::{
    Beta, Binomial, Cauchy, ChiSquared, Exp, FisherF, Frechet, Gamma, Geometric, Gumbel,
    Hypergeometric, InverseGaussian, LogNormal, Normal, NormalInverseGaussian, Pareto, Pert,
    Poisson, SkewNormal, StudentT, Triangular, Weibull, Zeta, Zipf,
};
use std::{
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RandomError {
    #[error("Invalid range")]
    InvalidRange,
    #[error(transparent)]
    Bernoulli(#[from] rand::distributions::BernoulliError),
    #[error(transparent)]
    Normal(#[from] rand_distr::NormalError),
    #[error(transparent)]
    Skew(#[from] rand_distr::SkewNormalError),
    #[error(transparent)]
    Cauchy(#[from] rand_distr::CauchyError),
    #[error(transparent)]
    Binomial(#[from] rand_distr::BinomialError),
    #[error(transparent)]
    Geometric(#[from] rand_distr::GeoError),
    #[error(transparent)]
    Hypergeometric(#[from] rand_distr::HyperGeoError),
    #[error(transparent)]
    Pareto(#[from] rand_distr::ParetoError),
    #[error(transparent)]
    Poisson(#[from] rand_distr::PoissonError),
    #[error(transparent)]
    Exp(#[from] rand_distr::ExpError),
    #[error(transparent)]
    Weibull(#[from] rand_distr::WeibullError),
    #[error(transparent)]
    Gumbel(#[from] rand_distr::GumbelError),
    #[error(transparent)]
    Frechet(#[from] rand_distr::FrechetError),
    #[error(transparent)]
    Zeta(#[from] rand_distr::ZetaError),
    #[error(transparent)]
    Zipf(#[from] rand_distr::ZipfError),
    #[error(transparent)]
    Gamma(#[from] rand_distr::GammaError),
    #[error(transparent)]
    ChiSquared(#[from] rand_distr::ChiSquaredError),
    #[error(transparent)]
    FisherF(#[from] rand_distr::FisherFError),
    #[error(transparent)]
    Beta(#[from] rand_distr::BetaError),
    #[error(transparent)]
    Triangular(#[from] rand_distr::TriangularError),
    #[error(transparent)]
    Pert(#[from] rand_distr::PertError),
    #[error(transparent)]
    InverseGaussian(#[from] rand_distr::InverseGaussianError),
    #[error(transparent)]
    NormalInverseGaussian(#[from] rand_distr::NormalInverseGaussianError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
}

#[inline(always)]
fn rng() -> ChaCha8Rng {
    ChaCha8Rng::from_rng(rand::thread_rng()).unwrap()
}

// Simple

macro_rules! rand_funcs {
    ( $($ret:ty),* ) => {
        paste! {
            $(
                byond_fn!(
                    fn [<rand_ $ret>]() {
                        Some(rng().gen::<$ret>().to_string())
                    }
                );
            )*
        }
    };
}

// Create `rand_standard_X` functions.
rand_funcs!(i32, u32, f32);

macro_rules! rand_range_funcs {
    ($($ret:ty),*) => {
        paste! {
            $(
                byond_fn!(fn [<rand_range_ $ret>](low, high) {
                    #[inline(always)]
                    fn uniform_range_impl(low: &str, high: &str) -> Result<$ret, Error> {
                        let d = uniform_ctor(low, high)?;

                        Ok(rng().sample(d))
                    }

                    match uniform_range_impl(low, high) {
                        Ok(v) => Some(v.to_string()),
                        Err(err) => Some(format!("ERROR: {err}"))
                    }
                });
            )*
        }
    };
}

#[inline(always)]
fn uniform_ctor<X: SampleUniform + FromStr + PartialOrd>(
    low: &str,
    high: &str,
) -> Result<Uniform<X>, RandomError>
where
    Error: From<<X as FromStr>::Err>,
    RandomError: From<<X as FromStr>::Err>,
{
    let low = X::from_str(low)?;
    let high = X::from_str(high)?;

    if low >= high {
        return Err(RandomError::InvalidRange);
    }

    Ok(Uniform::new(low, high))
}

// Create `rand_range_X` functions.
rand_range_funcs!(i32, u32, f32);

byond_fn!(fn rand_bool(p) {
    fn rand_bernoulli_bool_impl(p: &str) -> Result<bool, RandomError> {
        let p = f64::from_str(p)?;
        let d = Bernoulli::new(p)?;

        Ok(rng().sample(d))
    }

    match rand_bernoulli_bool_impl(p) {
        Ok(v) => {
            let v = if v { 1 } else { 0 };

            Some(v.to_string())
        },
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

byond_fn!(fn rand_ratio(nominator, denominator) {
    fn rand_bernoulli_ratio_impl(nominator: &str, denominator: &str) -> Result<bool, RandomError> {
        let nominator = u32::from_str(nominator)?;
        let denominator = u32::from_str(denominator)?;
        let d = Bernoulli::from_ratio(nominator, denominator)?;

        Ok(rng().sample(d))
    }

    match rand_bernoulli_ratio_impl(nominator, denominator) {
        Ok(v) => {
            let v = if v { 1 } else { 0 };

            Some(v.to_string())
        },
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Normal

#[inline(always)]
fn normal_ctor(mean: &str, std_dev: &str) -> Result<Normal<f32>, RandomError> {
    let mean = f32::from_str(mean)?;
    let std_dev = f32::from_str(std_dev)?;

    Ok(Normal::new(mean, std_dev)?)
}

#[inline(always)]
fn normal_cv_ctor(mean: &str, cv: &str) -> Result<Normal<f32>, RandomError> {
    let mean = f32::from_str(mean)?;
    let cv = f32::from_str(cv)?;

    Ok(Normal::from_mean_cv(mean, cv)?)
}

byond_fn!(fn rand_normal_sample(mean, std_dev) {
    match normal_ctor(mean, std_dev).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

byond_fn!(fn rand_normal_cv_sample(mean, cv) {
    match normal_cv_ctor(mean, cv).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Skew

byond_fn!(fn rand_skew_sample(location, scale, shape) {
    fn skew_ctor(location: &str, scale: &str, shape: &str) -> Result<SkewNormal<f32>, RandomError> {
        let location = f32::from_str(location)?;
        let scale = f32::from_str(scale)?;
        let shape = f32::from_str(shape)?;

        Ok(SkewNormal::new(location, scale, shape)?)
    }

    match skew_ctor(location, scale, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Cauchy

byond_fn!(fn rand_cauchy_sample(median, scale) {
    fn cauchy_ctor(median: &str, scale: &str) -> Result<Cauchy<f32>, RandomError> {
        let median = f32::from_str(median)?;
        let scale = f32::from_str(scale)?;

        Ok(Cauchy::new(median, scale)?)
    }

    match cauchy_ctor(median, scale).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Binomial

byond_fn!(fn rand_binomial_sample(n, p) {
    fn binominal_ctor(n: &str, p: &str) -> Result<Binomial, RandomError> {
        let n = u64::from_str(n)?;
        let p = f64::from_str(p)?;

        Ok(Binomial::new(n, p)?)
    }

    match binominal_ctor(n, p).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Geometric

byond_fn!(fn rand_geometric_sample(p) {
    fn geometric_ctor(p: &str) -> Result<Geometric, RandomError> {
        let p = f64::from_str(p)?;

        Ok(Geometric::new(p)?)
    }

    match geometric_ctor(p).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Hypergeometric

byond_fn!(fn rand_hypergeometric_sample(N, K, n) {
    fn hypergeometric_ctor(N: &str, K: &str, n: &str) -> Result<Hypergeometric, RandomError> {
        let N = u64::from_str(N)?;
        let K = u64::from_str(K)?;
        let n = u64::from_str(n)?;

        Ok(Hypergeometric::new(N, K, n)?)
    }

    match hypergeometric_ctor(N, K, n).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Log

fn log_ctor(mu: &str, sigma: &str) -> Result<LogNormal<f32>, RandomError> {
    let mu = f32::from_str(mu)?;
    let sigma = f32::from_str(sigma)?;

    Ok(LogNormal::new(mu, sigma)?)
}

fn log_mean_cv_ctor(mean: &str, cv: &str) -> Result<LogNormal<f32>, RandomError> {
    let mean = f32::from_str(mean)?;
    let cv = f32::from_str(cv)?;

    Ok(LogNormal::from_mean_cv(mean, cv)?)
}

byond_fn!(fn rand_log_sample(mu, sigma) {
    match log_ctor(mu, sigma).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

byond_fn!(fn rand_log_mean_cv_sample(mean, cv) {
    match log_mean_cv_ctor(mean, cv).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Pareto

byond_fn!(fn rand_pareto_sample(scale, shape) {
    fn pareto_ctor(scale: &str, shape: &str) -> Result<Pareto<f32>, RandomError> {
        let scale = f32::from_str(scale)?;
        let shape = f32::from_str(shape)?;

        Ok(Pareto::new(scale, shape)?)
    }

    match pareto_ctor(scale, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Poisson

byond_fn!(fn rand_poisson_sample(lambda) {
    fn poisson_ctor(lambda: &str) -> Result<Poisson<f32>, RandomError> {
        let lambda = f32::from_str(lambda)?;

        Ok(Poisson::new(lambda)?)
    }

    match poisson_ctor(lambda).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Exp

byond_fn!(fn rand_exp_sample(lambda) {
    fn exp_ctor(lambda: &str) -> Result<Exp<f32>, RandomError> {
        let lambda = f32::from_str(lambda)?;

        Ok(Exp::new(lambda)?)
    }

    match exp_ctor(lambda).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Weibull

byond_fn!(fn rand_weibull_sample(scale, shape) {
    fn weibull_ctor(scale: &str, shape: &str) -> Result<Weibull<f32>, RandomError> {
        let scale = f32::from_str(scale)?;
        let shape = f32::from_str(shape)?;

        Ok(Weibull::new(scale, shape)?)
    }

    match weibull_ctor(scale, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Gumbel

byond_fn!(fn rand_gumbel_sample(location, scale) {
    fn gumbel_ctor(location: &str, scale: &str) -> Result<Gumbel<f32>, RandomError> {
        let location = f32::from_str(location)?;
        let scale = f32::from_str(scale)?;

        Ok(Gumbel::new(location, scale)?)
    }

    match gumbel_ctor(location, scale).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Frechet

byond_fn!(fn rand_frechet_sample(location, scale, shape) {
    fn frechet_ctor(location: &str, scale: &str, shape: &str) -> Result<Frechet<f32>, RandomError> {
        let location = f32::from_str(location)?;
        let scale = f32::from_str(scale)?;
        let shape = f32::from_str(shape)?;

        Ok(Frechet::new(location, scale, shape)?)
    }

    match frechet_ctor(location, scale, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Zeta

byond_fn!(fn rand_zeta_sample(a) {
    fn zeta_ctor(a: &str) -> Result<Zeta<f32>, RandomError> {
        let a = f32::from_str(a)?;

        Ok(Zeta::new(a)?)
    }

    match zeta_ctor(a).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Zipf

byond_fn!(fn rand_zipf_sample(n, s) {
    fn zipf_ctor(n: &str, s: &str) -> Result<Zipf<f32>, RandomError> {
        let n = u64::from_str(n)?;
        let s = f32::from_str(s)?;

        Ok(Zipf::new(n, s)?)
    }

    match zipf_ctor(n, s).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Gamma

byond_fn!(fn rand_gamma_sample(shape, scale) {
    fn gamma_ctor(shape: &str, scale: &str) -> Result<Gamma<f32>, RandomError> {
        let shape = f32::from_str(shape)?;
        let scale = f32::from_str(scale)?;

        Ok(Gamma::new(shape, scale)?)
    }

    match gamma_ctor(shape, scale).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// ChiSquared

byond_fn!(fn rand_chisquared_sample(k) {
    fn chisquared_ctor(k: &str) -> Result<ChiSquared<f32>, RandomError> {
        let k = f32::from_str(k)?;

        Ok(ChiSquared::new(k)?)
    }

    match chisquared_ctor(k).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// StudentT

byond_fn!(fn rand_studentt_sample(n) {
    fn studentt_ctor(n: &str) -> Result<StudentT<f32>, RandomError> {
        let n = f32::from_str(n)?;

        Ok(StudentT::new(n)?)
    }

    match studentt_ctor(n).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// FisherF

byond_fn!(fn rand_fisherf_sample(m, n) {
    fn fisherf_ctor(m: &str, n: &str) -> Result<FisherF<f32>, RandomError> {
        let m = f32::from_str(m)?;
        let n = f32::from_str(n)?;

        Ok(FisherF::new(m, n)?)
    }

    match fisherf_ctor(m, n).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Beta

byond_fn!(fn rand_beta_sample(alpha, beta) {
    fn beta_ctor(alpha: &str, beta: &str) -> Result<Beta<f32>, RandomError> {
        let alpha = f32::from_str(alpha)?;
        let beta = f32::from_str(beta)?;

        Ok(Beta::new(alpha, beta)?)
    }

    match beta_ctor(alpha, beta).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Triangular

byond_fn!(fn rand_triangular_sample(min, max, mode) {
    fn triangular_ctor(min: &str, max: &str, mode: &str) -> Result<Triangular<f32>, RandomError> {
        let min = f32::from_str(min)?;
        let max = f32::from_str(max)?;
        let mode = f32::from_str(mode)?;

        Ok(Triangular::new(min, max, mode)?)
    }

    match triangular_ctor(min, max, mode).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Pert

byond_fn!(fn rand_pert_sample(min, max, mode) {
    fn pert_ctor(min: &str, max: &str, mode: &str) -> Result<Pert<f32>, RandomError> {
        let min = f32::from_str(min)?;
        let max = f32::from_str(max)?;
        let mode = f32::from_str(mode)?;

        Ok(Pert::new(min, max, mode)?)
    }

    match pert_ctor(min, max, mode).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

byond_fn!(fn rand_pert_shape_sample(min, max, mode, shape) {
    fn pert_shape_ctor(min: &str, max: &str, mode: &str, shape: &str) -> Result<Pert<f32>, RandomError> {
        let min = f32::from_str(min)?;
        let max = f32::from_str(max)?;
        let mode = f32::from_str(mode)?;
        let shape = f32::from_str(shape)?;

        Ok(Pert::new_with_shape(min, max, mode, shape)?)
    }

    match pert_shape_ctor(min, max, mode, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Inverse Gaussian

byond_fn!(fn rand_inverse_gaussian_sample(mean, shape) {
    fn inverse_gaussian_ctor(mean: &str, shape: &str) -> Result<InverseGaussian<f32>, RandomError> {
        let mean = f32::from_str(mean)?;
        let shape = f32::from_str(shape)?;

        Ok(InverseGaussian::new(mean, shape)?)
    }

    match inverse_gaussian_ctor(mean, shape).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});

// Normal Inverse Gaussian

byond_fn!(fn rand_normal_inverse_gaussian(alpha, beta) {
    fn normal_inverse_gaussian_ctor(alpha: &str, beta: &str) -> Result<NormalInverseGaussian<f32>, RandomError> {
        let alpha = f32::from_str(alpha)?;
        let beta = f32::from_str(beta)?;

        Ok(NormalInverseGaussian::new(alpha, beta)?)
    }

    match normal_inverse_gaussian_ctor(alpha, beta).and_then(|d| Ok(rng().sample(d))) {
        Ok(v) => Some(v.to_string()),
        Err(err) => Some(format!("ERROR: {err}"))
    }
});
