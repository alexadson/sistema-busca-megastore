pub fn tokenizar(texto: &str) -> Vec<String> {
    texto
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizar() {
        let entrada = "Camiseta azul tamanho M!";
        let saida = tokenizar(entrada);
        assert!(saida.contains(&"camiseta".to_string()));
        assert!(saida.contains(&"azul".to_string()));
    }
}
