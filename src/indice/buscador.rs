use std::collections::HashMap;

pub fn buscar(indice: &HashMap<String, Vec<u32>>, termos: &[String]) -> Vec<u32> {
    let mut resultados = Vec::new();

    for termo in termos {
        if let Some(ids) = indice.get(termo) {
            resultados.extend(ids);
        }
    }

    resultados
}
