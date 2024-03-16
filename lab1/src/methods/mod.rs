pub mod bisection;
pub mod chord;
pub mod golden_section;
pub mod newton;

pub trait Function {
    fn f(&self, x: f64) -> f64;
    fn f_deriv(&self, x: f64) -> f64;
    fn f_deriv2(&self, x: f64) -> f64;
}

pub struct Optimizer<'a> {
    a: f64,
    b: f64,
    eps: f64,
    func: &'a dyn Function,
}

impl<'a> Optimizer<'a> {
    pub fn new(a: f64, b: f64, eps: f64, func: &'a dyn Function) -> Self {
        Optimizer { a, b, eps, func }
    }

    pub fn bisection(&self) -> f64 {
        bisection::bisection(self.a, self.b, self.eps, self.func)
    }

    pub fn golden_section(&self) -> f64 {
        golden_section::golden_section(self.a, self.b, self.eps, self.func)
    }

    pub fn chord(&self) -> f64 {
        chord::chord(self.a, self.b, self.eps, self.func)
    }

    pub fn newton(&self) -> f64 {
        newton::newton(self.a, self.b, self.eps, self.func)
    }
}
