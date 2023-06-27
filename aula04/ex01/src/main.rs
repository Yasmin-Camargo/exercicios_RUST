// 1) Implemente uma função "swap", que receba dois valores inteiros e os troque
use std::io;

fn main() {
    println!("Digite o valor 1:");
    let mut valor1 = input_int();

    println!("Digite o valor 2:");
    let mut valor2 = input_int();

    println!("\n{}, {}", valor1, valor2);
    swap(&mut valor1, &mut valor2);
    println!("... swap ...");
    println!("\n{}, {}", valor1, valor2);
}

fn swap(valor1: &mut i32, valor2: &mut i32){
    let temp = *valor1;
    *valor1 = *valor2;
    *valor2 = temp;
}

fn input_int() -> i32{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    let valor_int: i32 = input.trim().parse().expect("Valor inválido");
    
    valor_int
}
