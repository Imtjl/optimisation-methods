mod methods;

use methods::{Optimizer, Function};

struct Function1;
impl Function for Function1 {
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

struct Function2;
impl Function for Function2 {
    fn f(&self, x: f64) -> f64 {
        x.powi(3) + x.powi(2) - 7.0 * x
    }

    fn f_deriv(&self, x: f64) -> f64 {
        3.0*x.powi(2) + 2.0*x - 7.0
    }

    fn f_deriv2(&self, x: f64) -> f64 {
        6.0*x + 2.0
    }
}

fn main() {
    let func = Function1;
    let test1 = Optimizer::new(0.0, 1.0, 0.0001, &func);

    let func2 = Function2;
    let test2 = Optimizer::new(0.0, 2.0, 0.1, &func2);

    let opt = test2;
    println!("Метод половинного деления: {}", opt.bisection());
    println!("Метод золотого сечения: {}", opt.golden_section());
    println!("Метод хорд: {}", opt.chord());
    println!("Метод Ньютона: {}", opt.newton());
    println!("Метод квадратичной аппроксимации: {}", opt.quadratic_approx());
}
