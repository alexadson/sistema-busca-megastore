use sistema_busca_megastore::{buscar_produtos, modelo::Produto};
use std::io;

fn main() {
    // Catálogo estático com alguns produtos
    let catalogo = vec![
        Produto {
            nome: "Camiseta Azul".into(),
            marca: "Nike".into(),
            categoria: "Roupas".into(),
        },
        Produto {
            nome: "Tênis Runner".into(),
            marca: "Adidas".into(),
            categoria: "Calçados".into(),
        },
        Produto {
            nome: "Notebook Z3".into(),
            marca: "TechX".into(),
            categoria: "Eletrônicos".into(),
        },
    ];

    // Lê o termo de busca do usuário
    println!("Digite um termo para buscar (nome, marca ou categoria): ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    let termo = entrada.trim();

    // Chama a função de busca
    let resultados = buscar_produtos(&catalogo, termo);

    if resultados.is_empty() {
        println!("Nenhum produto encontrado para '{}'.", termo);
    } else {
        println!("Produtos encontrados:");
        for produto in resultados {
            println!("- {} | {} | {}", produto.nome, produto.marca, produto.categoria);
        }
    }
}
