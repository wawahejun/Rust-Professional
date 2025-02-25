use std::cmp::max;

type Number = u128;

struct PrimeFactor {
    bases: [Number; 7],        // Miller-Rabin test bases
    pollard_threshold: Number, // Threshold for switching to Pollard's Rho
}

impl PrimeFactor {
    fn new() -> Self {
        Self {
            bases: [2, 3, 5, 7, 11, 13, 17],
            pollard_threshold: 1_000_000,
        }
    }

    /// Modular multiplication to avoid overflow
    #[inline]
    fn mul_mod(&self, a: Number, b: Number, m: Number) -> Number {
        let mut res = 0;
        let mut base = b % m;
        let mut a = a;
        while a > 0 {
            if a & 1 == 1 {
                res = (res + base) % m;
            }
            base = (base << 1) % m;
            a >>= 1;
        }
        res
    }

    /// Modular exponentiation
    fn mod_pow(&self, base: Number, exp: Number, modulus: Number) -> Number {
        if modulus == 1 {
            return 0;
        }

        let mut result = 1;
        let mut base = base % modulus;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                result = self.mul_mod(result, base, modulus);
            }
            base = self.mul_mod(base, base, modulus);
            exp >>= 1;
        }
        result
    }

    /// Miller-Rabin primality test
    fn is_prime(&self, n: Number) -> bool {
        if n <= 1 || n == 4 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let mut d = n - 1;
        let mut r = 0;
        while d % 2 == 0 {
            d /= 2;
            r += 1;
        }

        for &a in self.bases.iter() {
            if a >= n {
                break;
            }
            let mut x = self.mod_pow(a, d, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            let mut is_composite = true;
            for _ in 0..r - 1 {
                x = self.mul_mod(x, x, n);
                if x == n - 1 {
                    is_composite = false;
                    break;
                }
            }
            if is_composite {
                return false;
            }
        }
        true
    }

    /// Pollard's Rho algorithm
    fn pollard_rho(&self, n: Number) -> Option<Number> {
        if n % 2 == 0 {
            return Some(2);
        }
        if self.is_prime(n) {
            return Some(n);
        }

        let f = |x: Number, c: Number, n: Number| -> Number {
            (self.mul_mod(x, x, n) + c) % n
        };

        for c in 1..=10 {
            let (mut x, mut y, mut d) = (2, 2, 1);
            while d == 1 {
                x = f(x, c, n);
                let t = f(y, c, n);
                y = f(t, c, n);
                d = gcd(x.abs_diff(y), n);
            }
            if d != 1 && d != n {
                return Some(d);
            }
        }
        None
    }

    /// Trial division
    fn trial_division(&self, n: Number) -> Option<Number> {
        let sqrt = (n as f64).sqrt() as Number;
        let mut i = 5;

        while i <= sqrt {
            if n % i == 0 {
                return Some(i);
            }
            if n % (i + 2) == 0 {
                return Some(i + 2);
            }
            i += 6;
        }
        None
    }
}

/// Greatest Common Divisor (GCD)
fn gcd(a: Number, b: Number) -> Number {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Find the maximum prime factor
pub fn find_max_prime_factor(number: Number) -> Number {
    let prime = PrimeFactor::new();
    let mut n = number;
    let mut max_factor = 1;

    // Handle small prime factors
    for &small_prime in &[2, 3] {
        while n % small_prime == 0 {
            max_factor = max(max_factor, small_prime);
            n /= small_prime;
        }
    }

    while n > 1 {
        let factor = if n < prime.pollard_threshold {
            prime.trial_division(n)
        } else {
            prime.pollard_rho(n).or_else(|| prime.trial_division(n))
        };

        match factor {
            Some(f) => {
                max_factor = max(max_factor, f);
                n /= f;
                if prime.is_prime(n) {
                    max_factor = max(max_factor, n);
                    break;
                }
            }
            None => {
                max_factor = max(max_factor, n);
                break;
            }
        }
    }

    max_factor
}