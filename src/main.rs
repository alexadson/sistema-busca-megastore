mod modelo;
mod indice;
mod util;

use modelo::produto::Produto;
use indice::construtor::construir_indice;
use indice::buscador::buscar;
use util::texto::tokenizar;
use std::fs::File;
use csv::Reader;

fn main() {
    let caminho = "dados/produtos.csv";
    let mut produtos: Vec<Produto> = Vec::new();

    let arquivo = File::open(caminho).expect("Erro ao abrir o arquivo CSV.");
    let mut leitor = Reader::from_reader(arquivo);

    for resultado in leitor.deserialize() {
        let produto: Produto = resultado.expect("Erro ao ler produto.");
        produtos.push(produto);
    }

    let indice = construir_indice(&produtos);

    println!("Digite um termo de busca:");
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).unwrap();
    let termos = tokenizar(&entrada);

    let ids = buscar(&indice, &termos);

    println!("Resultados:");
    for id in ids {
        if let Some(p) = produtos.iter().find(|p| p.id == id) {
            println!("{} - {}", p.id, p.nome);
        }
    }
}
