use std::collections::HashMap;

use crate::maths;
use crate::maths::Complex;

pub trait Solver {
    fn solve(&mut self, degrees: &HashMap<u32, f64>);
    fn describe(&self);
}

pub struct ZeroDegreeSolver {
    degree_0: f64,
}

impl ZeroDegreeSolver {
    pub fn new() -> Self {
        ZeroDegreeSolver { degree_0: 0.0 }
    }
}

impl Solver for ZeroDegreeSolver {
    fn solve(&mut self, degrees: &HashMap<u32, f64>) {
        self.degree_0 = *degrees.get(&0).unwrap_or_else(|| &0.0);
    }

    fn describe(&self) {
        println!(
            "Reduced form: {}{} * X ^ 0 = 0",
            if self.degree_0 < 0.0 { "- " } else { "" },
            self.degree_0.abs()
        );
        println!("Polynomial degree: 0");
        if self.degree_0 == 0.0 {
            println!("All real numbers are solution");
        } else {
            println!("No real number is solution");
        }
    }
}

pub struct OneDegreeSolver {
    degree_0: f64,
    degree_1: f64,
    x: f64,
}

impl OneDegreeSolver {
    pub fn new() -> Self {
        OneDegreeSolver {
            degree_0: 0.0,
            degree_1: 0.0,
            x: 0.0,
        }
    }
}

impl Solver for OneDegreeSolver {
    fn solve(&mut self, degrees: &HashMap<u32, f64>) {
        self.degree_0 = *degrees.get(&0).unwrap_or_else(|| &0.0);
        self.degree_1 = *degrees.get(&1).unwrap();
        self.x = -(self.degree_0 / self.degree_1);
    }

    fn describe(&self) {
        println!(
            "Reduced form: {}{} * X^0 {} {} * X^1 = 0",
            if self.degree_0 < 0.0 { "- " } else { "" },
            self.degree_0.abs(),
            if self.degree_1 < 0.0 { "-" } else { "+" },
            self.degree_1.abs()
        );
        println!("Polynomial degree: 1");
        println!("The solution is:\n{}", self.x);
    }
}

pub struct TwoDegreeSolver {
    degree_0: f64,
    degree_1: f64,
    degree_2: f64,
    z_1: Complex,
    z_2: Complex,
    delta: f64,
}

impl TwoDegreeSolver {
    pub fn new() -> Self {
        TwoDegreeSolver {
            degree_0: 0.0,
            degree_1: 0.0,
            degree_2: 0.0,
            z_1: Complex {
                real: 0.0,
                imag: 0.0,
            },
            z_2: Complex {
                real: 0.0,
                imag: 0.0,
            },
            delta: 0.0,
        }
    }
}

impl Solver for TwoDegreeSolver {
    fn solve(&mut self, degrees: &HashMap<u32, f64>) {
        self.degree_0 = *degrees.get(&0).unwrap_or_else(|| &0.0);
        self.degree_1 = *degrees.get(&1).unwrap_or_else(|| &0.0);
        self.degree_2 = *degrees.get(&2).unwrap();
        self.delta = self.degree_1 * self.degree_1 - 4.0 * self.degree_2 * self.degree_0;
        if self.delta == 0.0 {
            self.z_1.real = -self.degree_1 / (2.0 * self.degree_2);
        } else if self.delta > 0.0 {
            let sqrt_delta = maths::sqrt(self.delta);
            self.z_1.real = -(-self.degree_1 - sqrt_delta) / (2.0 * self.degree_2);
            self.z_2.real = -(-self.degree_1 + sqrt_delta) / (2.0 * self.degree_2);
        } else {
            let sqrt_delta = maths::sqrt(-self.delta);
            self.z_1.real = -self.degree_1 / (2.0 * self.degree_2);
            self.z_1.imag = -sqrt_delta / (2.0 * self.degree_2);
            self.z_2.real = -self.degree_1 / (2.0 * self.degree_2);
            self.z_2.imag = sqrt_delta / (2.0 * self.degree_2);
        }
    }

    fn describe(&self) {
        println!(
            "Reduced form: {}{} * X^0 {} {} * X^1 {} {} * X^2 = 0",
            if self.degree_0 < 0.0 { "- " } else { "" },
            self.degree_0.abs(),
            if self.degree_1 < 0.0 { "-" } else { "+" },
            self.degree_1.abs(),
            if self.degree_2 < 0.0 { "-" } else { "+" },
            self.degree_1.abs()
        );
        println!("Polynomial degree: 2");
        if self.delta == 0.0 {
            println!("Discriminant is null, the solution is:");
            println!("{:.2}", self.z_1.real);
        } else if self.delta > 0.0 {
            println!("Discriminant is strictly positive, the two solutions are:");
            println!("{:.2}", self.z_1.real);
            println!("{:.2}", self.z_2.real);
        } else {
            println!("Discriminant is strictly negative, the two complex solutions are:");
            println!(
                "{:.2}{}i{:.2}",
                self.z_1.real,
                if self.z_1.imag >= 0.0 { "+" } else { "-" },
                self.z_1.imag.abs()
            );
            println!(
                "{:.2}{}i{:.2}",
                self.z_2.real,
                if self.z_2.imag >= 0.0 { "+" } else { "-" },
                self.z_2.imag.abs()
            );
        }
    }
}

pub struct MoreDegreeSolver {
    degrees: Vec<(u32, f64)>,
}

impl<'a> MoreDegreeSolver {
    pub fn new() -> Self {
        MoreDegreeSolver {
            degrees: Vec::new(),
        }
    }
}

impl Solver for MoreDegreeSolver {
    fn solve(&mut self, degrees: &HashMap<u32, f64>) {
        self.degrees = degrees
            .iter()
            .map(|(deg, val)| (*deg, *val))
            .collect::<Vec<(u32, f64)>>();
        self.degrees.sort_by_key(|&(deg, _)| deg);
    }

    fn describe(&self) {
        print!("Reduced form: ");
        for (i, (deg, val)) in self.degrees.iter().enumerate() {
            if i == 0 {
                print!(
                    "{}{} * X^{}",
                    if *val < 0.0 { "- " } else { "" },
                    val.abs(),
                    deg
                );
            } else {
                print!(
                    "{} {} * X^{}",
                    if *val < 0.0 { "-" } else { "+" },
                    val.abs(),
                    deg
                );
            }
        }
        print!(" = 0\n");
        println!(
            "Polynomial degree: {}",
            self.degrees[self.degrees.len() - 1].0
        );
        println!("The polynomial degree is stricly greater than 2, I can't solve.");
    }
}

pub fn choose_solver(degrees: &HashMap<u32, f64>) -> Box<dyn Solver> {
    let mut degrees_vec = degrees
        .iter()
        .map(|(deg, val)| (*deg, *val))
        .collect::<Vec<(u32, f64)>>();
    degrees_vec.sort_by_key(|&(deg, _)| deg);
    println!("{}", degrees.len());
    match degrees_vec[degrees.len() - 1].0 {
        0 => Box::new(ZeroDegreeSolver::new()),
        1 => Box::new(OneDegreeSolver::new()),
        2 => Box::new(TwoDegreeSolver::new()),
        _ => Box::new(MoreDegreeSolver::new()),
    }
}
