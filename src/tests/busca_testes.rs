use sistema_busca_megastore::{buscar_produtos, modelo::Produto};

#[test]
fn busca_por_marca_funciona() {
    let produtos = vec![
        Produto {
            nome: "Produto X".to_string(),
            marca: "MarcaA".to_string(),
            categoria: "Categoria1".to_string(),
        },
        Produto {
            nome: "Produto Y".to_string(),
            marca: "MarcaB".to_string(),
            categoria: "Categoria2".to_string(),
        },
    ];

    let resultado = buscar_produtos(&produtos, "marcaa");
    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado[0].marca, "MarcaA");
}

#[test]
fn busca_sem_resultado() {
    let produtos = vec![Produto {
        nome: "Produto X".to_string(),
        marca: "MarcaZ".to_string(),
        categoria: "CategoriaY".to_string(),
    }];

    let resultado = buscar_produtos(&produtos, "banana");
    assert!(resultado.is_empty());
}
