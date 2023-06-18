// 2) Faça um programa que receba um valor por parâmetro e imprima um triangulo na tela com o caracter

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        let num1: i32 = match args[1].parse(){
            Ok(num) => num,
            Err(_) => {
                println!("argumento 1 é inválido");
                return;
            }
        };

        for _i in 0..num1 {
            for _j in 0..(_i + 1){
                print!("*");
            }
            println!("");
        }

    }
    else{
        println!("Número de argumentos é inválido");
    }
}