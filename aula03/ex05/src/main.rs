// programa para somar os elementos de um vetor de inteiros

fn main() {
    let n = 10;
    let mut soma:i32 = 0;
    let mut vet: Vec<i32> = Vec::with_capacity(n);

    for _i in 0..n {
        vet.push(_i as i32);
    }
    println!("Vetor: {:?}", vet);

    for _i in 0..n{
        soma = soma + vet[_i];
    }
    println!("Soma: {}", soma);

    soma = vet.iter().sum();
    println!("Soma: {}", soma);
}
