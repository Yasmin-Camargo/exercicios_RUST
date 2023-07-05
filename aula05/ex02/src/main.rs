fn gera_retangulos(size: usize) -> Vec<Retangulo> {
    let mut vet = Vec::with_capacity(size);
    for _ in 0..size {
        let mut novo_retangulo = Retangulo::cria(rand::thread_rng().gen_range(2, 20), rand::thread_rng().gen_range(2, 20), 0);
        novo_retangulo.calcula_area();
        vet.push(novo_retangulo);
    }
    vet
}

fn bubble_sort(vetor: & mut Vec<Retangulo>){
    for i in 0..(vetor.len()){
        for j in (i)..(vetor.len()){
            if vetor[j].area < vetor[i].area{
                vetor.swap(i, j);
            }
        }
    }
}

fn insere_ordenado(vet: & mut Vec<Retangulo>, retangulo: Retangulo) {
    for i in 0..(vet.len()){
        if retangulo.area < vet[i].area {
            if i == 0{
                vet.insert(0, retangulo);
            } else {
                vet.insert(i, retangulo);
            }
            return;
        }
    }
}

fn main() {
    let quantidade_vetores_aleatorio = 5;
    let mut novo_retangulo = Retangulo::cria(10, 20, 0);
    println!("Meu retangulo: largura = {}, altura = {}", novo_retangulo.largura, novo_retangulo.altura);
    novo_retangulo.calcula_area();
    println!("Area do retangulo: {}", novo_retangulo.area);

    let mut vetor = gera_retangulos(quantidade_vetores_aleatorio);

    println!("\n\n{} Vetores foram gerados", quantidade_vetores_aleatorio);
    for _i in 0..(vetor.len()){
        println!("Vetor {}, area = {}", _i, vetor[_i].area);
    }

    bubble_sort(&mut vetor);
    println!("\n\n ordenando ....");
    for _i in 0..(vetor.len()){
        println!("Vetor {}, area = {}", _i, vetor[_i].area);
    }

    println!("\nInserindo mais um, mas já na posição correta...");
    insere_ordenado(&mut vetor, novo_retangulo);
    for _i in 0..(vetor.len()){
        println!("Vetor {}, area = {}", _i, vetor[_i].area);
    }
}