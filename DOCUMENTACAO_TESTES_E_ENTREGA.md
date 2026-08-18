# Wallet Live

## Portfolio-ready summary

Aplicação fullstack em Rust para gestão de carteira de investimentos, com autenticação, persistência em PostgreSQL e interface web renderizada no servidor.

### O que o projeto entrega

- login e sessão de usuário
- listagem de ativos
- registro de compras em carteira
- acompanhamento de quantidade, valor e histórico
- backend em Rust com API e frontend em uma mesma base

### Stack

- Rust
- Axum
- SQLx
- PostgreSQL
- Askama
- JWT + cookies
- Docker Compose

---

## Arquitetura em uma frase

O projeto combina backend, frontend server-side, autenticação e banco relacional em uma solução coesa para gerenciamento de investimentos pessoais.

---

## Como executar

### Pré-requisitos

- Rust
- Docker + Docker Compose

### Passo a passo

```bash
git clone https://github.com/valdyrtorres/wallet-live.git
cd wallet-live
docker compose up -d
cargo sqlx migrate run
cargo run
```

Acesse:

- http://localhost:3000/login
- http://localhost:3000/assets

---

## Apresentação do projeto

O Wallet Live foi desenvolvido para simular uma carteira de investimentos pessoal com fluxo completo de autenticação e acompanhamento de posições. A solução reforça conceitos de arquitetura web em Rust, autenticação, persistência e renderização server-side, além de demonstrar a integração entre backend e interface em um único projeto.

---

## Estrutura do projeto

```text
.
├── src/
│   ├── app.rs
│   ├── main.rs
│   ├── models.rs
│   ├── repository.rs
│   ├── error.rs
│   ├── auth/
│   └── routes/
├── templates/
│   ├── login.html
│   └── assets.html
├── migrations/
├── compose.yml
├── .env
├── Cargo.toml
├── readme.md
└── DOCUMENTACAO_TESTES_E_ENTREGA.md
```

---

## Como executar em qualquer máquina

### 1. Pré-requisitos

- Rust instalado
- Docker e Docker Compose instalados
- Git

### 2. Clonar o projeto

```bash
git clone https://github.com/valdyrtorres/wallet-live.git
cd wallet-live
```

### 3. Subir o banco PostgreSQL

```bash
docker compose up -d
```

### 4. Verificar a configuração do banco

O arquivo `.env` deve conter:

```env
DATABASE_URL=postgres://wallet_user:wallet_password@localhost:5432/wallet_db
```

### 5. Rodar as migrações

```bash
cargo sqlx migrate run
```

### 6. Compilar e iniciar a aplicação

```bash
cargo run
```

### 7. Acessar a aplicação

- Login: http://localhost:3000/login
- Carteira: http://localhost:3000/assets

---

## Apresentação resumida do projeto

O Wallet Live foi implementado como uma aplicação fullstack simples, segura e funcional, com foco em:

- autenticação de usuários
- persistência de dados em banco relacional
- interface web em renderização server-side
- gestão de carteira de investimentos
- histórico de compra e acompanhamento de patrimônio

Em termos práticos, o projeto entrega uma solução completa para testar conceitos de arquitetura web em Rust, autenticação, modelagem de dados e integração entre frontend e backend.

---

## Validação executada

A aplicação foi validada em ambiente real com os seguintes resultados:

- compilação bem-sucedida com `cargo build`
- servidor subindo em `http://0.0.0.0:3000`
- página de login respondendo com `HTTP 200 OK`
- autenticação funcionando com cookie JWT
- acesso à carteira autenticada com `HTTP 200 OK`
- registro de compra aceito e redirecionando para `/assets`
- renderização da carteira com ativo e quantidade exibidos corretamente

### Evidências reais

```bash
cargo build
cargo run
curl -sS -D - http://localhost:3000/login | head
curl -sS -b /tmp/wallet.jar -D - http://localhost:3000/assets
```

Exemplo de resposta observada:

```text
HTTP/1.1 200 OK
HTTP/1.1 303 See Other
location: /assets
```

---

## Observações finais

- o banco PostgreSQL precisa estar ativo antes do app iniciar
- a aplicação escuta na porta `3000`
- a autenticação usa sessão por cookie com token JWT

---

## Conclusão

O projeto está funcional, documentado e validado em fluxo real de uso. Com backend em Rust, uso de banco relacional, autenticação e desenvolvimento fullstack em uma única stack.
