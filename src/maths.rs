pub struct Complex {
	pub real: f64,
	pub imag: f64
}

pub fn sqrt(number: f64) -> f64
{
	let threshold: f64 = 0.000001;
	let mut ans: f64 = number / 2.0;
	while ((ans * ans) - number).abs() > threshold {
		ans = (ans + (number / ans)) / 2.0;
	}
	ans
}