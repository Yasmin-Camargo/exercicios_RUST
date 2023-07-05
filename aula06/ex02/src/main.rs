enum Shape {
    Circulo(f64),
    Quadrado(f64),
    Triangulo(f64, f64),
    Retangulo(f64, f64)
}

fn calcularArea(shape: Shape){
    match shape{
        Shape::Circulo(raio) => {
            println!("Area círculo: {}", f64::powf(raio, 2.0)*3.14);
        }
        Shape::Quadrado(lado) => {
            println!("Area Quadrado: {}", lado * lado);
        }
        Shape::Triangulo(base, altura) => {
            println!("Area Triangulo: {}", (base * altura)/2.0);
        }
        Shape::Retangulo(base, altura) => {
            println!("Area Retangulo: {}", base * altura);
        }
    }
}

fn main() {
    let figura = Shape::Quadrado(10.2);
    let figura2 = Shape::Circulo(4.0);
    let figura3 = Shape::Triangulo(2.0, 5.0);
    let figura4 = Shape::Retangulo(2.0, 2.0);
    calcularArea(figura);
    calcularArea(figura2);
    calcularArea(figura3);
    calcularArea(figura4);
}