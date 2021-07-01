use std::collections::HashMap;

pub trait Solver {
	fn solve(&mut self, degrees: &HashMap<u32, f64>);
	fn describe(&self);
}

pub struct ZeroDegreeSolver {
	degree_0: f64
}

impl ZeroDegreeSolver {

	pub fn new() -> Self {
		ZeroDegreeSolver {
			degree_0: 0.0
		}
	}
}

impl Solver for ZeroDegreeSolver {

	fn solve(&mut self, degrees: &HashMap<u32, f64>) {
		self.degree_0 = *degrees.get(&0).unwrap_or_else(|| &0.0);
	}

	fn describe(&self) {
		println!("Reduced form: {} {} * X ^ 0 = 0", if self.degree_0 < 0.0 {"-"} else {""}, self.degree_0.abs());
		println!("Polynomial degree: 0");
		if self.degree_0 == 0.0 {
			println!("All real numbers are solution");
		}
		else {
			println!("No real number is solution");
		}
	}
}

pub struct OneDegreeSolver {
	degree_0: f64,
	degree_1: f64,
	x: f64
}

impl OneDegreeSolver {

	pub fn new() -> Self {
		OneDegreeSolver {
			degree_0: 0.0,
			degree_1: 0.0,
			x: 0.0
		}
	}
}

impl Solver for OneDegreeSolver {

	fn solve(&mut self, degrees: &HashMap<u32, f64>) {
		self.degree_0 = *degrees.get(&0).unwrap_or_else(|| &0.0);
		self.degree_1 = *degrees.get(&1).unwrap();
		self.x = - (self.degree_0 / self.degree_1); 
	}

	fn describe(&self) {
		println!("Reduced form: {} {} * X ^ 0 {} {} * X ^ 1 = 0",
			if self.degree_0 < 0.0 {"-"} else {""}, self.degree_0.abs(),
			if self.degree_1 < 0.0 {"-"} else {"+"}, self.degree_1.abs());
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
	delta: f64
}

impl TwoDegreeSolver {

	pub fn new() -> Self {
		TwoDegreeSolver {
			degree_0: 0.0,
			degree_1: 0.0,
			degree_2: 0.0,
			z_1: Complex {real: 0.0, imag: 0.0},
			z_2: Complex {real: 0.0, imag: 0.0},
			delta: 0.0
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
			self.z_1.real = - self.degree_1 / (2.0 * self.degree_2);
		}
		else if self.delta > 0.0 {
			let sqrt_delta = sqrt(self.delta);
			self.z_1.real = - (- self.degree_1 - sqrt_delta) / (2.0 * self.degree_2);
			self.z_2.real = - (- self.degree_1 + sqrt_delta) / (2.0 * self.degree_2);
		}
		else {
			let sqrt_delta = sqrt(- self.delta);
			self.z_1.real = - self.degree_1 / (2.0 * self.degree_2);
			self.z_1.imag = - sqrt_delta / (2.0 * self.degree_2);
			self.z_2.real = - self.degree_1 / (2.0 * self.degree_2);
			self.z_2.imag = sqrt_delta / (2.0 * self.degree_2);
		}
	}

	fn describe(&self) {
		println!("Reduced form: {} {} * X ^ 0 {} {} * X ^ 1 {} {} * X ^ 2 = 0",
		if self.degree_0 < 0.0 {"-"} else {""}, self.degree_0.abs(),
		if self.degree_1 < 0.0 {"-"} else {"+"}, self.degree_1.abs(),
		if self.degree_2 < 0.0 {"-"} else {"+"}, self.degree_1.abs());
		println!("Polynomial degree: 2");
		if self.delta == 0.0 {
			println!("Discriminant is null, the solution is:");
			println!("{}", self.z_1.real);
		}
		else if self.delta > 0.0 {
			println!("Discriminant is strictly positive, the two solutions are:");
			println!("{}", self.z_1.real);
			println!("{}", self.z_2.real);
		}
		else {
			println!("Discriminant is strictly negative, the two complex solutions are:");
			println!("{}{}i{}", self.z_1.real, if self.z_1.imag >= 0.0 {"+"} else {"-"}, self.z_1.imag.abs());
			println!("{}{}i{}", self.z_2.real, if self.z_2.imag >= 0.0 {"+"} else {"-"}, self.z_2.imag.abs());
		}
	}
}

pub struct Complex {
	real: f64,
	imag: f64
}

pub fn sqrt<T>(number: T) -> T {
	unimplemented!()
}
