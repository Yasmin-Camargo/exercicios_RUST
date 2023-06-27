// 4) Implemente a função bubble_sort para completar o programa.
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
                let temp  = num[i];
                num[i] = num[j];
                num[j] = temp;
                //num.swap(i,j);
            }

        }
    }
}

fn main() {
    let size = 10;
    let mut vet = gera_vet(size);
    println!("Vetor original: {:?}", vet);
    bubble_sort(&mut vet); // Opa... isso não existe (ainda)
    println!("Vetor ordenado: {:?}", vet);
}
