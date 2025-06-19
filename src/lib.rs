pub mod modelo;

use modelo::Produto;

/// Realiza uma busca simples no vetor de produtos.
/// Retorna uma lista de produtos cujo campo nome, marca ou categoria contém o termo informado.
pub fn buscar_produtos(catalogo: &[Produto], termo: &str) -> Vec<Produto> {
    let termo = termo.to_lowercase();
    catalogo
        .iter()
        .filter(|p| {
            p.nome.to_lowercase().contains(&termo)
                || p.marca.to_lowercase().contains(&termo)
                || p.categoria.to_lowercase().contains(&termo)
        })
        .cloned()
        .collect()
}

