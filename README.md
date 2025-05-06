# 🛒 Sistema de Busca para Catálogo de Produtos - MegaStore

## 📌 Descrição do Projeto
Este projeto implementa um sistema de busca eficiente para o catálogo de produtos da **MegaStore**, 
utilizando **Rust** e **tabelas hash** para indexação. Com técnicas de **pré-processamento de texto** e um **sistema de cache**, 
o sistema melhora a relevância dos resultados e reduz o tempo de resposta em consultas frequentes.

## 🚀 Tecnologias Utilizadas
- **Rust** → Linguagem segura e de alto desempenho.
- **Cargo** → Gerenciador de pacotes e sistema de build do Rust.
- **Crates utilizadas**:
  - [`serde`](https://crates.io/crates/serde) → Serialização de dados.
  - [`tokio`](https://crates.io/crates/tokio) → Manipulação assíncrona.
  - [`hashbrown`](https://crates.io/crates/hashbrown) → Tabelas hash eficientes.

## 🛠️ Como Executar o Sistema de Busca
1. **Clone o repositório**:
   ```sh
   git clone https://github.com/seu-usuario/sistema-busca-megastore.git
   cd sistema-busca-megastore


🏗️ Arquitetura do Sistema
O sistema segue um modelo modular para facilitar manutenção e escalabilidade:

indexador.rs: Armazena o catálogo de produtos em tabelas hash.

buscador.rs: Responsável por processar consultas de busca.

cache.rs: Implementa cache para consultas frequentes.

pre_processamento.rs: Normaliza os termos da consulta.

📊 Algoritmos e Estruturas de Dados Utilizados
Tabelas hash para indexação eficiente dos produtos.

Normalização de texto (stemming e remoção de stop words).

Sistema de cache para melhorar desempenho.

⚡ Considerações sobre Desempenho e Escalabilidade
Busca rápida utilizando tabelas hash.

Melhoria na relevância com técnicas de pré-processamento de texto.

Cache para otimizar buscas repetidas e reduzir consumo de recursos.

💡 Contribuições
Contribuições são bem-vindas! Para enviar melhorias:

Faça um fork do projeto.

Crie uma branch para sua funcionalidade.

Envie um pull request com suas alterações.

🔖 Licença
Este projeto está licenciado sob a MIT License.




