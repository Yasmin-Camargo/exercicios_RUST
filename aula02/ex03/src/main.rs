// 3) Faça um programa que receba dois valores por parâmetro e imprima um retângulo na tela com o caracter * de forma que a base seja mais "comprida" que a altura
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
            imprime_retangulo(num2, num1)
        } 
        else {
            imprime_retangulo(num1, num2)
        }
    }
}

fn imprime_retangulo(num1: i32, num2: i32){
    for _i in 0..num1 {
        for _j in 0..num2 {
            print!("*");
        }
        println!("");
    }
}