use super::Function;

pub fn newton(a: f64, b: f64, eps: f64, func: &dyn Function) -> f64 {
    let mut x = (a + b) / 2.0;
    while func.f_deriv(x).abs() > eps {
        x = x - func.f_deriv(x) / func.f_deriv2(x);
    }
    x
}
