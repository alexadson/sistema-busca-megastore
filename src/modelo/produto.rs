use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub descricao: String,
    pub categoria: String,
}
