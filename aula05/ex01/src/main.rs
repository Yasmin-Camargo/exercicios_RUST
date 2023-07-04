use std::time::Instant;
use rand::Rng;

fn gera_vet(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vet = Vec::with_capacity(size);

    for _ in 0..size {
        vet.push(rng.gen_range(0..100));
    }
    vet
}

fn bubble_sort(num: & mut Vec<i32>){
    for i in 0..(num.len()){
        for j in (i)..(num.len()){
            if num[j] < num[i]{
                num.swap(i, j);
            }
        }
    }
}

fn swap(valor1: &mut i32, valor2: &mut i32){
    let temp = *valor1;
    *valor1 = *valor2;
    *valor2 = temp;
}

fn main() {
    let size = 10;
    let mut vet = gera_vet(size);
    println!("Vetor original: {:?}", vet);
    let inicio = Instant::now();
    bubble_sort(&mut vet); // Opa... isso não existe (ainda)
    let duracao = Instant::now() - inicio;
    println!("Vetor ordenado: {:?}", vet);
    println!("Tempo de execucao(ms): {:?}", duracao);
}
