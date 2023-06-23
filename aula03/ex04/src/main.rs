// Construa uma tupla para representar um número complexo, algo como a + bi, sendo que a e b são números f32. Realize operações simples de soma e subtração com esta tupla
type Complex = (f64, f64);

fn main() {
	let mut tupla_x: Complex = (5.5,2.0);
	let tupla_y: Complex = (10.5,10.5);

	println!("A = {} {}i e B = {} {}i", tupla_x.0, tupla_x.1, tupla_y.0, tupla_y.1);

	tupla_x = complex_soma(tupla_x, tupla_y);

	println!("A + B = {} {}i", tupla_x.0, tupla_x.1);
}

fn complex_soma(a: Complex, b: Complex) -> Complex {
	let real = a.0 + b.0;
	let imaginario = a.1 + b.1;
	(real, imaginario)
}
