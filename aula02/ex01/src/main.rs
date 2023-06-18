// 1) Faça um programa que receba dois valores inteiros como parâmetro e imprima o maior dos dois valores recebidos

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
      println!("Erro! Número de argumentos é inválido");
      return;
    }
    else {
        let num1: i32 = match args[1].parse(){
            Ok(num) => num,
            Err(_) => {
                println!("argumento 1 é inválido");
                return;
            }
        };
        let num2: i32 = match args[2].parse(){
            Ok(num) => num,
            Err(_) => {
                println!("argumento 1 é inválido");
                return;
            }
        };

        if num1 > num2{
            println!("Número {} é maior que {}", num1, num2);
        }
        else {
            println!("Número {} é maior que {}", num2, num1);
        }
    }
}
