use crate::modelo::produto::Produto;
use crate::util::texto::tokenizar;
use std::collections::HashMap;

pub fn construir_indice(produtos: &[Produto]) -> HashMap<String, Vec<u32>> {
    let mut indice = HashMap::new();

    for produto in produtos {
        let texto = format!("{} {}", produto.nome, produto.descricao);
        let termos = tokenizar(&texto);

        for termo in termos {
            indice.entry(termo).or_insert(Vec::new()).push(produto.id);
        }
    }

    indice
}
