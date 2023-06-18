// Construa um programa que imprima a mensagem Hello <meu nome>! na tela, onde o nome a ser apresentado é o seu, passado como parâmetro ao lançamento do programa
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
      println!("Hello, {}!",args[1]);
    }
}