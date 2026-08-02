Rust - 25/06/2026

Versão atual 2024
versão 1.75

https://releases.rs/

https://rust-br.github.io/rust-book-pt-br/title-page.html

https://github.com/digitalinnovationone/treinamento_rust

https://doc.rust-lang.org/book

🧩 Tecnologias que utilizaremos
🖥️ Servidor: Axum
O framework backend mais suportado pelo ecossistema

É a base do loco.rs

Design similar a FastAPI, Flask e Express.js

Async first

🗄️ Queries na DB: SQLx
Permite a escrita de type-safe SQL

Sem abstrações complexas

Mais facilidade na escrita de queries complexas

🎨 Frontend: Askama
Template engine type-safe

Sintaxe similar ao Jinja2 e ao Django Template

Frontend estático tradicional

wsl

Projeto Final

Desenvolvendo sua Carteira de Investimentos Inteligente com Rust
https://web.dio.me/project/desenvolvendo-sua-carteira-de-investimentos-inteligente-com-rust/learning/f9e5990c-2776-465d-8964-b5b115a8bc5b?back=/track/santander-2026-rust-fullstack&tab=undefined&moduleId=undefined

video 1:
https://www.youtube.com/watch?v=Fyv0zgZgYg4

🧠 Desenvolvimento do projeto: ordem das aulas
1️⃣ Primeiros passos com o Axum
API REST para cadastro, listagem e atualização de ativos

Autenticação básica baseada em Secret Key

Salvamento dos ativos em memória

Gerenciamento de erros

2️⃣ Introdução ao SQLx
Gerenciamento de migrações em SQL

Substituição do salvamento em memória por um banco de dados Postgres

Adição de testes unitários

3️⃣ Primeira tela no Frontend: usando Askama para criar um Frontend SSR
Criação do modelo de usuário

Criação da tela de login/cadastro de usuários

Criação de uma simples página de índice

4️⃣ Implementando autenticação stateless com JWTs
Implementação de persistência de sessões via Cookies no Frontend

5️⃣ Criação e gerenciamento dos ativos financeiros do usuário

⚙️ Dependências do Projeto

[dependencies]
askama = { version = "0.15.4", features = ["derive"] }
axum = { version = "0.8.8", features = ["macros"] }
axum-extra = { version = "0.12.5", features = ["cookie-signed"] }
color-eyre = "0.6.5"
dotenvy = "0.15.7"
jwt-simple = "0.12.14"
password-auth = "1.0.0"
serde = { version = "1.0.228", features = ["derive"] }
serde-json = "1.0.149"
sqlx = { version = "0.8.6", features = ["macros", "runtime-tokio", "postgres"] }
thiserror = "2.0.18"
time = { version = "0.3.47", features = ["serde"] }
tokio = { version = "1.50.0", features = ["rt-multi-thread", "macros"] }
tracing = "0.1.44"
tracing-subscriber = "0.3.22"
