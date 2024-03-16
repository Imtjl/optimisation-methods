use super::Function;

pub fn bisection(mut a: f64, mut b: f64, eps: f64, func: &dyn Function) -> f64 {
    while (b - a) > 2.0 * eps {
        let x = (a + b) / 2.0;
        if func.f(x - (eps / 2.0)) > func.f(x + (eps / 2.0)) {
            a = x;
        } else {
            b = x;
        }
    }
    (a + b) / 2.0
}
