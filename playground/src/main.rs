// 検証用の書き捨て

fn main() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);

    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);

    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(std::f64::consts::FRAC_PI_2) - 1.).abs() < 0.005);
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema(slice: &[i32]) -> Extrema<'_> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

/// N-1 次の多項式を表す。
struct Polynomial<const N: usize> {
    /// 多項式の係数。
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Self { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}
