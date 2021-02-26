use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Rational {
    nominator: BigInt,
    denominator: BigInt,
}

impl Rational {
    fn new(n: &BigInt, d: &BigInt) -> Rational {
        assert!(!d.is_zero(), "denominator cannot be 0");
        // simplify if possible
        let c = n.gcd(d);
        Rational {
            nominator: n / &c,
            denominator: d / &c,
        }
    }

    fn is_proper(&self) -> bool {
        self.nominator < self.denominator
    }
    fn to_egyptian(&self) -> VecRational {
        let mut frac: VecRational = VecRational(Vec::new());

        let mut current: Rational;
        if !self.is_proper() {
            // input is grater than 1
            // store the integer part
            frac.0.push(Rational::new(
                &self.nominator.div_floor(&self.denominator),
                &One::one(),
            ));

            // calculate the remainder
            current = Rational::new(
                &self.nominator.mod_floor(&self.denominator),
                &self.denominator,
            );
        } else {
            current = self.clone();
        }

        while !current.nominator.is_one() {
            let div = current.denominator.div_ceil(&current.nominator);

            // store the term
            frac.0.push(Rational::new(&One::one(), &div));

            current = Rational::new(
                &(-&current.denominator).mod_floor(&current.nominator),
                match current.denominator.checked_mul(&div).as_ref() {
                    Some(r) => r,
                    _ => break,
                },
            );
        }

        frac.0.push(current);
        frac
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator.is_one() {
            // for integers only display the integer part
            write!(f, "{}", self.nominator)
        } else {
            write!(f, "{}/{}", self.nominator, self.denominator)
        }
    }
}

#[derive(Debug)]
struct VecRational(Vec<Rational>);

impl fmt::Display for VecRational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in self.0.iter().enumerate() {
            // For every element except the first, add a +.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn run_max_searches(x: usize) {
    // generate all proper fractions with 2 digits
    let pairs = (1..x).flat_map(move |i| (i + 1..x).map(move |j| (i, j)));

    let mut max_length = (0, Rational::new(&BigInt::from(1), &BigInt::from(1)));
    let mut max_denom = (
        Zero::zero(),
        Rational::new(&BigInt::from(1), &BigInt::from(1)),
    );

    for (i, j) in pairs {
        let e = Rational::new(&BigInt::from(i), &BigInt::from(j)).to_egyptian();
        if e.0.len() > max_length.0 {
            max_length = (e.0.len(), Rational::new(&BigInt::from(i), &BigInt::from(j)));
        }

        if e.0.last().unwrap().denominator > max_denom.0 {
            max_denom = (
                e.0.last().unwrap().denominator.clone(),
                Rational::new(&BigInt::from(i), &BigInt::from(j)),
            );
        }
    }

    println!(
        "Maximum length of terms is for {} with {} terms",
        max_length.1, max_length.0
    );
    println!("{}", max_length.1.to_egyptian());

    println!(
        "Maximum denominator is for {} with {} terms",
        max_denom.1, max_denom.0
    );
    println!("{}", max_denom.1.to_egyptian());
}
fn main() {
    let tests = [
        Rational::new(&BigInt::from(43), &BigInt::from(48)),
        Rational::new(&BigInt::from(5), &BigInt::from(121)),
        Rational::new(&BigInt::from(2014), &BigInt::from(59)),
    ];

    for test in tests.iter() {
        println!("{} -> {}", test, test.to_egyptian());
    }

    run_max_searches(100);
    run_max_searches(1000);
}

#[cfg(test)]
mod tests {
    use super::Rational;
    use num_bigint::BigInt;

    #[test]
    fn test_egyptian() {
        // case: 1/2 -> 1.2
        let e = Rational::new(&BigInt::from(1), &BigInt::from(2));
        println!("{}", e);
        assert_eq!(
            e.to_egyptian().0,
            vec![Rational::new(&BigInt::from(1), &BigInt::from(2)),]
        );

        // case: 5/6 -> 1/2 + 1/3
        let e = Rational::new(&BigInt::from(5), &BigInt::from(6));
        println!("{}", e);
        assert_eq!(
            e.to_egyptian().0,
            vec![
                Rational::new(&BigInt::from(1), &BigInt::from(2)),
                Rational::new(&BigInt::from(1), &BigInt::from(3)),
            ]
        );

        // case 3/2 (improper fraction) -> 1 + 1/2
        let e = Rational::new(&BigInt::from(3), &BigInt::from(2));
        println!("{}", e);
        assert_eq!(
            e.to_egyptian().0,
            vec![
                Rational::new(&BigInt::from(1), &BigInt::from(1)),
                Rational::new(&BigInt::from(1), &BigInt::from(2)),
            ]
        );

        // case 43/48 -> 1/2 + 1/3 + /16
        let e = Rational::new(&BigInt::from(43), &BigInt::from(48));
        println!("{}", e);
        assert_eq!(
            e.to_egyptian().0,
            vec![
                Rational::new(&BigInt::from(1), &BigInt::from(2)),
                Rational::new(&BigInt::from(1), &BigInt::from(3)),
                Rational::new(&BigInt::from(1), &BigInt::from(16)),
            ]
        );
    }

    #[test]
    #[should_panic]
    fn zero_denominator() {
        let e = Rational::new(&BigInt::from(1), &BigInt::from(0));
        println!("{}", e);
    }
}
