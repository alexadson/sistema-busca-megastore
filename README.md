Sistema de Busca Otimizado para Catálogo de Produtos – MegaStore
📌 Descrição do projeto
Este sistema de busca foi desenvolvido para indexar e consultar produtos de forma eficiente com base em três critérios: nome, marca e categoria. Idealizado com foco acadêmico e didático, ele simula uma funcionalidade real de catálogos de e-commerce. Além de ser leve e intuitivo, o projeto é modular e testável, demonstrando os fundamentos de organização de código em Rust.

🧰 Tecnologias utilizadas
Linguagem: Rust (edition 2021)

Gerenciador de dependências: Cargo

Crates utilizadas: Apenas a biblioteca padrão (std)

Testes automatizados: Sistema nativo de testes do Rust com #[test]

🚀 Instruções de como executar o sistema
Certifique-se de ter o Rust instalado. Se não tiver, instale com:

bash
curl https://sh.rustup.rs -sSf | sh
Clone o repositório:

bash
git clone https://github.com/seu-usuario/sistema-busca-megastore.git
cd sistema-busca-megastore
Compile e execute a aplicação:

bash
cargo run
🧪 Instruções de como executar os testes
Execute os testes unitários e de integração com:

bash
cargo test
Os testes estão localizados em:

tests/busca_testes.rs → validações de busca por nome, marca e categoria.

💡 Exemplos de uso
Exemplo de entrada no terminal:

Digite um termo para buscar (nome, marca ou categoria): notebook
Saída esperada:

Produtos encontrados:
- Notebook Z3 | TechX | Eletrônicos
Outras possibilidades:

nike → retorna produtos da marca Nike

calçados → retorna produtos com categoria Calçados

runner → retorna produtos com esse termo no nome

🧱 Arquitetura do sistema
Estrutura modular com responsabilidades separadas:

src/
├── main.rs       → Interface de terminal
├── lib.rs        → Função central de busca e exportações
├── modelo.rs     → Estrutura do objeto Produto
tests/
└── busca_testes.rs → Testes externos com cenários variados
🧠 Algoritmos e estruturas de dados utilizados
A busca é implementada por meio de:

Iteração linear sobre Vec<Produto>

Comparações com to_lowercase() + contains()

Filtros com iter().filter()

Esse tipo de busca textual direta simula, em pequena escala, o comportamento de tabelas hash ao combinar performance com flexibilidade. A estrutura é eficiente para catálogos de pequeno e médio porte.

📈 Considerações sobre desempenho e escalabilidade
Desempenho: Excelente com catálogos locais contendo até milhares de produtos — execução instantânea.

Escalabilidade futura: Para lidar com milhões de itens:

Adicionar indexação avançada (e.g. índice invertido)

Armazenamento persistente (e.g. SQLite, Redis, ou banco relacional)

Servir via API web (e.g. com actix-web)

Aplicar técnicas como cache, paginador e sharding

🤝 Contribuições
Este projeto é aberto a contribuições! Para participar:

Faça um fork do repositório

Crie uma nova branch com sua melhoria

Envie um pull request explicando suas mudanças

Sugestões, melhorias e comentários são sempre bem-vindos.

⚖️ Licença
Distribuído sob a licença MIT. Consulte o arquivo LICENSE para mais detalhes.