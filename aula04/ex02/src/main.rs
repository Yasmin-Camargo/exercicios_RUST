// 2) Implemente uma função compare-and-swap. Esta função recebe três parâmetros inteiros, antigo, novo e condicao. Se o valor antigo e o valor condição forem iguais, não faz nada. Se forem diferentes, o valor antigo deve ser alterado para o valor novo. Algo do tipo:
use std::io;

fn main() {
    println!("Digite um número: ");
    let mut valor1 = input_int();

    println!("Digite um novo número: ");
    let valor2 = input_int();

    println!("Digite uma condição: ");
    let valor3 = input_int();

    println!("\nValores digitados: {}, {}, {}", valor1, valor2, valor3);

    if valor1 != valor3{
        valor1 = valor2;
    }

    println!("Compare-and-swap:  {}, {}, {}", valor1, valor2, valor3);
}

fn input_int() -> i32{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    let valor_int: i32 = input.trim().parse().expect("Valor inválido");
    
    valor_int
}
