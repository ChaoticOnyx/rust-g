// Simple

/// Returns a random integer in range from `i32::MIN` to `i32::MAX`
#define rustg_rand_i32(...) text2num(RUSTG_CALL(RUST_G, "rand_i32")())

/// Returns a random integer in range from `u32::MIN` to `u32::MAX`
#define rustg_rand_u32(...) text2num(RUSTG_CALL(RUST_G, "rand_u32")())

/// Returns a random integer in range from `f32::MIN` to `f32::MAX`
#define rustg_rand_f32(...) text2num(RUSTG_CALL(RUST_G, "rand_f32")())

/// Returns a random i32 value in range `[low, high)`
#define rustg_rand_range_i32(low, high) text2num(RUSTG_CALL(RUST_G, "rand_range_i32")(istext(low) ? low : num2text(low), istext(high) ? high : num2text(high)))

/// Returns a random u32 value in range `[low, high)`
#define rustg_rand_range_u32(low, high) text2num(RUSTG_CALL(RUST_G, "rand_range_u32")(istext(low) ? low : num2text(low), istext(high) ? high : num2text(high)))

/// Returns a random f32 value in range `[low, high)`
#define rustg_rand_range_f32(low, high) text2num(RUSTG_CALL(RUST_G, "rand_range_f32")(istext(low) ? low : num2text(low), istext(high) ? high : num2text(high)))

/// Returns a bool with a probability p of being true.
#define rustg_rand_bool(p) text2num(RUSTG_CALL(RUST_G, "rand_bool")(istext(p) ? p : num2text(p)))

/// Returns a bool with a probability of numerator/denominator of being true. I.e. gen_ratio(2, 3) has chance of 2 in 3, or about 67%, of returning true.
/// If numerator == denominator, then the returned value is guaranteed to be true. If numerator == 0, then the returned value is guaranteed to be false.
#define rustg_rand_ratio(nominator, denominator) ext2num(RUSTG_CALL(RUST_G, "rand_ratio")(istext(nominator) ? nominator : num2text(nominator), istext(denominator) ? denominator : num2text(denominator)))

// Related to real-valued quantities that grow linearly (e.g. errors, offsets):

// Normal distribution

/// The normal distribution `N(mean, std_dev**2)`. From mean and standard deviation
/// Parameters:
/// mean (μ, unrestricted)
/// standard deviation (σ, must be finite)
#define rustg_rand_normal_sample(mean, std_dev) text2num(RUSTG_CALL(RUST_G, "rand_normal_sample")(istext(mean) ? mean : num2text(mean), istext(std_dev) ? std_dev : num2text(std_dev)))

/// The normal distribution `N(mean, std_dev**2)`. From mean and coefficient of variation
/// Parameters:
/// mean (μ, unrestricted)
/// coefficient of variation (cv = abs(σ / μ))
#define rustg_rand_normal_cv_sample(mean, cv) text2num(RUSTG_CALL(RUST_G, "rand_normal_cv_sample")(istext(mean) ? mean : num2text(mean), istext(cv) ? cv : num2text(cv)))

// Skew

/// The skew normal distribution `SN(location, scale, shape)`.
/// The skew normal distribution is a generalization of the Normal distribution to allow for non-zero skewness.
/// It has the density function, for scale > 0, f(x) = 2 / scale * phi((x - location) / scale) * Phi(alpha * (x - location) / scale) where phi and Phi are the density and distribution of a standard normal variable.
#define rustg_rand_skew_sample(location, scale, shape) text2num(RUSTG_CALL(RUST_G, "rand_skew_sample")(istext(location) ? location : num2text(location), istext(scale) ? scale : num2text(scale), istext(shape) ? shape : num2text(shape))

// Cauchy

/// The Cauchy distribution `Cauchy(median, scale)`.
/// This distribution has a density function: f(x) = 1 / (pi * scale * (1 + ((x - median) / scale)^2))
#define rustg_rand_cauchy_sample(median, scale) text2num(RUSTG_CALL(RUST_G, "rand_cauchy_sample")(istext(median) ? median : num2text(median), istext(scale) ? scale : num2text(scale)))

// Related to Bernoulli trials (yes/no events, with a given probability):

// Binomial

/// The binomial distribution Binomial(n, p).
/// This distribution has density function: f(k) = n!/(k! (n-k)!) p^k (1-p)^(n-k) for k >= 0.
#define rustg_rand_binomial_sample(n, p) text2num(RUSTG_CALL(RUST_G, "rand_binomial_sample")(istext(n) ? n : num2text(n), istext(p) ? p : num2text(p)))

// Geometric

/// The geometric distribution `Geometric(p)` bounded to [0, u64::MAX].
/// This is the probability distribution of the number of failures before the first success in a series of Bernoulli trials.
/// It has the density function `f(k) = (1 - p)^k p` for` k >= 0`, where p is the probability of success on each trial.
/// This is the discrete analogue of the exponential distribution.
#define rustg_rand_geometric_sample(p) text2num(RUSTG_CALL(RUST_G, "rand_geometric_sample")(istext(p) ? p : num2text(p)))

// Hypergeometric

/// The hypergeometric distribution Hypergeometric(N, K, n).
/// This is the distribution of successes in samples of size n drawn without replacement from a population of size N containing K success states.
/// It has the density function: f(k) = binomial(K, k) * binomial(N-K, n-k) / binomial(N, n), where binomial(a, b) = a! / (b! * (a - b)!).
/// The binomial distribution is the analogous distribution for sampling with replacement. It is a good approximation when the population size is much larger than the sample size.
#define rustg_rand_hypergeometric_sample(N, K, n) text2num(RUSTG_CALL(RUST_G, "rand_hypergeometric_sample")(istext(N) ? N : num2text(N), istext(K) ? K : num2text(K), istext(n) ? n : num2text(n)))

// Related to positive real-valued quantities that grow exponentially (e.g. prices, incomes, populations):

// Log

// The log-normal distribution ln N(mean, std_dev**2).
// If X is log-normal distributed, then ln(X) is N(mean, std_dev**2) distributed.

/// Construct, from (log-space) mean and standard deviation
/// Parameters are the “standard” log-space measures (these are the mean and standard deviation of the logarithm of samples):
/// mu (μ, unrestricted) is the mean of the underlying distribution
/// sigma (σ, must be finite) is the standard deviation of the underlying Normal distribution
#define rustg_rand_log_sample(mu, sigma) text2num(RUSTG_CALL(RUST_G, "rand_log_sample")(istext(mu) ? mu : num2text(mu), istext(sigma) ? sigma : num2text(sigma)))

/// Construct, from (linear-space) mean and coefficient of variation
/// Parameters are linear-space measures:
/// mean (μ > 0) is the (real) mean of the distribution
/// coefficient of variation (cv = σ / μ, requiring cv ≥ 0) is a standardized measure of dispersion
/// As a special exception, μ = 0, cv = 0 is allowed (samples are -inf).
#define rustg_rand_log_mean_cv_sample(mean, cv) text2num(RUSTG_CALL(RUST_G, "rand_log_mean_cv_sample")(istext(mean) ? mean : num2text(mean), istext(cv) ? cv : num2text(cv)))

// Related to the occurrence of independent events at a given rate:

// Pareto

/// Samples floating-point numbers according to the Pareto distribution
#define rustg_rand_pareto_sample(scale, shape) text2num(RUSTG_CALL(RUST_G, "rand_pareto_sample")(istext(scale) ? scale : num2text(scale), istext(shape) ? shape : num2text(shape)))

// Poisson

/// The Poisson distribution `Poisson(lambda)`.
/// This distribution has a density function: f(k) = lambda^k * exp(-lambda) / k! for k >= 0.
#define rustg_rand_poisson_sample(lambda) text2num(RUSTG_CALL(RUST_G, "rand_poisson_sample")(istext(lambda) ? lambda : num2text(lambda)))

// Exp

/// The exponential distribution Exp(lambda).
/// This distribution has density function: f(x) = lambda * exp(-lambda * x) for x > 0, when lambda > 0. For lambda = 0, all samples yield infinity.
#define rustg_rand_exp_sample(lambda) text2num(RUSTG_CALL(RUST_G, "rand_exp_sample")(istext(lambda) ? lambda : num2text(lambda)))

// Weibull

/// Samples floating-point numbers according to the Weibull distribution
#define rustg_rand_weibull_sample(scale, shape) text2num(RUSTG_CALL(RUST_G, "rand_weibull_sample")(istext(scale) ? scale : num2text(scale), istext(shape) ? shape : num2text(shape)))

// Gumbel

/// Samples floating-point numbers according to the Gumbel distribution
/// This distribution has density function: f(x) = exp(-(z + exp(-z))) / σ, where z = (x - μ) / σ, μ is the location parameter, and σ the scale parameter.
#define rustg_rand_gumbel_sample(location, scale) text2num(RUSTG_CALL(RUST_G, "rand_gumbel_sample")(istext(location) ? location : num2text(location), istext(scale) ? scale : num2text(scale)))

// Frechet

/// Samples floating-point numbers according to the Fréchet distribution
/// This distribution has density function: f(x) = [(x - μ) / σ]^(-1 - α) exp[-(x - μ) / σ]^(-α) α / σ, where μ is the location parameter, σ the scale parameter, and α the shape parameter.
#define rustg_rand_frechet_sample(location, scale, shape) text2num(RUSTG_CALL(RUST_G, "rand_frechet_sample")(istext(location) ? location : num2text(location), istext(scale) ? scale : num2text(scale), istext(shape) ? shape : num2text(shape)))

// Zeta

/// Samples integers according to the zeta distribution.
/// The zeta distribution is a limit of the Zipf distribution.
/// Sometimes it is called one of the following: discrete Pareto, Riemann-Zeta, Zipf, or Zipf–Estoup distribution.
/// It has the density function f(k) = k^(-a) / C(a) for k >= 1, where a is the parameter and C(a) is the Riemann zeta function.
#define rustg_rand_zeta_sample(a) text2num(RUSTG_CALL(RUST_G, "rand_zeta_sample")(istext(a) ? a : num2text(a)))

// Zipf

/// Samples integers according to the Zipf distribution.
/// The samples follow Zipf’s law: The frequency of each sample from a finite set of size n is inversely proportional to a power of its frequency rank (with exponent s).
/// For large n, this converges to the Zeta distribution.
/// For s = 0, this becomes a uniform distribution.
#define rustg_rand_zipf_sample(n, s) text2num(RUSTG_CALL(RUST_G, "rand_zipf_sample")(istext(n) ? n : num2text(n), istext(s) ? s : num2text(s)))

// Gamma and derived distributions:

// Gamma

/// The Gamma distribution Gamma(shape, scale) distribution.
/// The density function of this distribution is
/// `f(x) =  x^(k - 1) * exp(-x / θ) / (Γ(k) * θ^k)`
/// where Γ is the Gamma function, k is the shape and θ is the scale and both k and θ are strictly positive.
/// The algorithm used is that described by Marsaglia & Tsang 20001, falling back to directly sampling from an Exponential for shape == 1, and using the boosting technique described in that paper for shape < 1.
#define rustg_rand_gamma_sample(shape, scale) text2num(RUSTG_CALL(RUST_G, "rand_gamma_sample")(istext(shape) ? shape : num2text(shape), istext(scale) ? scale : num2text(scale)))

// ChiSquared

/// The chi-squared distribution χ²(k), where k is the degrees of freedom.
/// For k > 0 integral, this distribution is the sum of the squares of k independent standard normal random variables.
/// For other k, this uses the equivalent characterisation χ²(k) = Gamma(k/2, 2).
#define rustg_rand_chisquared_sample(k) text2num(RUSTG_CALL(RUST_G, "rand_chisquared_sample")(istext(k) ? k : num2text(k)))

// StudentT

/// The Student t distribution, t(nu), where nu is the degrees of freedom.
#define rustg_rand_studentt_sample(n) text2num(RUSTG_CALL(RUST_G, "rand_studentt_sample")(istext(n) ? n : num2text(n)))

// FisherF

/// The Fisher F distribution F(m, n).
/// This distribution is equivalent to the ratio of two normalised chi-squared distributions, that is, F(m,n) = (χ²(m)/m) / (χ²(n)/n).
#define rustg_rand_fisherf_sample(m, n) text2num(RUSTG_CALL(RUST_G, "rand_fisherf_sample")(istext(m) ? m : num2text(m), istext(n) ? n : num2text(n)))

// Triangular distribution:

// Beta

/// The Beta distribution with shape parameters alpha and beta.
#define rustg_rand_beta_sample(alpha, beta) text2num(RUSTG_CALL(RUST_G, "rand_beta_sample")(istext(alpha) ? alpha : num2text(alpha), istext(beta) ? beta : num2text(beta)))

// Triangular

/// The triangular distribution.
/// A continuous probability distribution parameterised by a range, and a mode (most likely value) within that range.
/// The probability density function is triangular. For a similar distribution with a smooth PDF, see the Pert distribution.
#define rustg_rand_triangular_sample(min, max, mode) text2num(RUSTG_CALL(RUST_G, "rand_triangular_sample")(istext(min) ? min : num2text(min), istext(max) ? max : num2text(max), istext(mode) ? mode : num2text(mode)))

// Pert

// The PERT distribution.
// Similar to the Triangular distribution, the PERT distribution is parameterised by a range and a mode within that range.
// Unlike the Triangular distribution, the probability density function of the PERT distribution is smooth, with a configurable weighting around the mode.

/// Set up the PERT distribution with defined min, max and mode.
/// This is equivalent to calling Pert::new_shape with shape == 4.0.
#define rustg_rand_pert_sample(min, max, mode) text2num(RUSTG_CALL(RUST_G, "rand_pert_sample")(istext(min) ? min : num2text(min), istext(max) ? max : num2text(max), istext(mode) ? mode : num2text(mode)))

/// Set up the PERT distribution with defined min, max, mode and shape.
#define rustg_rand_pert_shape_sample(min, max, mode, shape) text2num(RUSTG_CALL(RUST_G, "rand_pert_shape_sample")(istext(min) ? min : num2text(min), istext(max) ? max : num2text(max), istext(mode) ? mode : num2text(mode), istext(shape) ? shape : num2text(shape)))

// Misc. distributions

// Inverse Gaussian

/// https://en.wikipedia.org/wiki/Inverse_Gaussian_distribution
#define rustg_rand_inverse_gaussian_sample(mean, shape) text2num(RUSTG_CALL(RUST_G, "rand_inverse_gaussian_sample")(istext(mean) ? mean : num2text(mean), istext(shape) ? shape : num2text(shape)))

// Normal Inverse Gaussian

/// https://en.wikipedia.org/wiki/Normal-inverse_Gaussian_distribution
#define rustg_rand_normal_inverse_gaussian(alpha, beta) text2num(RUSTG_CALL(RUST_G, "rand_normal_inverse_gaussian")(istext(alpha) ? alpha : num2text(alpha), istext(beta) ? beta : num2text(beta)))
