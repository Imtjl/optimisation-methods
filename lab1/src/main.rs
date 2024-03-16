mod methods;

use methods::{Optimizer, Function};

struct MyFunction;
impl Function for MyFunction {
    fn f(&self, x: f64) -> f64 {
        0.5 * x.powi(2) - x.sin()
    }

    fn f_deriv(&self, x: f64) -> f64 {
        x - x.cos()
    }

    fn f_deriv2(&self, x: f64) -> f64 {
        1.0 + x.sin()
    }
}

fn main() {
    let func = MyFunction;
    let opt = Optimizer::new(0.0, 1.0, 0.03, &func);
    println!("Метод половинного деления: {}", opt.bisection());
    println!("Метод золотого сечения: {}", opt.golden_section());
    println!("Метод хорд: {}", opt.chord());
    println!("Метод Ньютона: {}", opt.newton());
}
