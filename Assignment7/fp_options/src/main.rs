#[derive(Debug)]
enum SafeFloat {
    Value(f64),
    NaN,
}

impl SafeFloat {
    fn unit(x: f64) -> SafeFloat {
        if x.is_nan() { SafeFloat::NaN } else { SafeFloat::Value(x) }
    }

    fn map<F: FnOnce(f64) -> f64>(self, f: F) -> SafeFloat {
        match self {
            SafeFloat::NaN => SafeFloat::NaN,
            SafeFloat::Value(x) => SafeFloat::unit(f(x)),
        }
    }

    fn and_then<F: FnOnce(f64) -> SafeFloat>(self, f: F) -> SafeFloat {
        match self {
            SafeFloat::NaN => SafeFloat::NaN,
            SafeFloat::Value(x) => f(x),
        }
    }
}

fn division(x: f64, y: f64) -> SafeFloat {
    if y == 0.0 { SafeFloat::NaN } else { SafeFloat::unit(x / y) }
}

fn square_root(x: f64) -> SafeFloat {
    if x < 0.0 { SafeFloat::NaN } else { SafeFloat::unit(x.sqrt()) }
}

fn logrithmic_equation(x: f64) -> SafeFloat {
    if x <= 0.0 { SafeFloat::NaN } else { SafeFloat::unit(x.ln()) }
}

fn main() {
    let result = SafeFloat::unit(16.0)
        .and_then(|x| square_root(x))
        .and_then(|x| division(x, 2.0))
        .map(|x| x + 24.0);
    println!("Non NaN input:{:?}", result);

    let result = SafeFloat::unit(f64::NAN).map(|x| x + 1.0);
    println!("Raw NaN input:{:?}", result);

    let result = SafeFloat::unit(16.0)
        .and_then(|x| square_root(x))
        .and_then(|x| division(x, 1.0))
        .map(|x| x + 1.0);
    println!("Division by 1:{:?}", result);

    let result = SafeFloat::unit(16.0)
        .and_then(|x| square_root(x))
        .and_then(|x| division(x, 0.0))
        .map(|x| x + 1.0);
    println!("Division by zero:{:?}", result);

    let result = SafeFloat::unit(4.0)
        .and_then(|x| square_root(x))
        .and_then(|x| division(x, 2.0))
        .map(|x| x * 100.0);
    println!("Square root of non-negative number:{:?}", result);

    let result = SafeFloat::unit(-4.0)
        .and_then(|x| square_root(x))
        .and_then(|x| division(x, 2.0))
        .map(|x| x * 100.0);
    println!("Square root of negative number:{:?}", result);

    let result = SafeFloat::unit(55.0)
        .and_then(|x| logrithmic_equation(x))
        .and_then(|x| logrithmic_equation(x))
        .map(|x| x + 99.0);
    println!("Log of non-zero number:{:?}", result);

    let result = SafeFloat::unit(1.0)
        .and_then(|x| logrithmic_equation(x))
        .and_then(|x| logrithmic_equation(x))
        .map(|x| x + 99.0);
    println!("Log of zero:{:?}", result);
}
