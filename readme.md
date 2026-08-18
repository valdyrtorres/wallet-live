# Wallet Live

## Carteira de Investimentos Fullstack com Rust

## Resumo executivo

Este projeto é uma aplicação fullstack em Rust para gerenciamento de carteira de investimentos. A solução permite que o usuário faça login, visualize seus ativos, registre compras e acompanhe o histórico de movimentações financeiras de forma simples, segura e organizada.

A aplicação combina:

- backend em Rust com Axum
- banco de dados PostgreSQL com SQLx
- autenticação via cookies e JWT
- renderização do frontend em servidor com Askama
- migrações SQL para versionamento do schema

---

## Funcionalidades principais

- autenticação de usuários
- login e logout com sessão persistida em cookie
- listagem de ativos disponíveis
- registro de compras em carteira
- cálculo de quantidade possuída e variação de valor
- histórico de compras por ativo
- persistência em banco PostgreSQL
- API REST para gestão de ativos
- interface web para uso direto no navegador

---

## Fluxo principal da aplicação

### 1. Inicialização
O processo começa em `src/main.rs`, que executa `App::start()`.

Na inicialização:

- o sistema configura logging com `tracing`
- lê a variável `DATABASE_URL`
- conecta ao PostgreSQL
- executa as migrações em `migrations/`
- inicia o servidor na porta `3000`

### 2. Rotas do frontend
O frontend fica em `src/routes/frontend.rs`.

Principais rotas:

- `/` → redireciona para `/login` ou `/assets`
- `/login` → renderiza a tela de login e autentica o usuário
- `/assets` → exibe a carteira do usuário autenticado
- `/logout` → encerra a sessão

### 3. Autenticação
A autenticação é feita em `src/auth/user.rs`.

O fluxo é:

1. o usuário envia username e senha
2. o sistema busca o usuário no banco
3. verifica a senha com `password-auth`
4. gera um token JWT
5. guarda o token em cookie
6. redireciona para a página principal da carteira

### 4. Gerenciamento de ativos
A camada de dados fica em `src/repository.rs`.

Ela cuida de:

- listar ativos
- criar e atualizar ativos
- listar ativos do usuário
- inserir compras na carteira
- consultar usuário por nome

### 5. API REST
A API em `src/routes/api.rs` expõe operações para ativos em JSON.

Ela é útil para integração e testes estruturados do backend.

---

## Arquitetura da aplicação

A solução foi organizada em camadas para facilitar manutenção, teste e entendimento do sistema.

```text
src/
├── main.rs                 # ponto de entrada da aplicação
├── app.rs                  # inicialização do servidor e banco
├── models.rs               # entidades do domínio
├── repository.rs           # acesso e consultas ao banco
├── error.rs                # tratamento de erros
├── auth/
│   ├── user.rs             # autenticação do usuário
│   ├── admin.rs            # autenticação administrativa
│   └── mod.rs
├── routes/
│   ├── frontend.rs         # rotas web e SSR
│   ├── api.rs              # API REST
│   └── mod.rs
├── templates/              # páginas renderizadas pelo Askama
├── migrations/             # versionamento do schema SQL
├── .env                    # variáveis de ambiente
└── compose.yml             # banco em Docker
```

## Tecnologias utilizadas

- Rust
- Axum
- SQLx
- PostgreSQL
- Askama
- JWT
- Cookies
- Docker Compose
- Tracing

---

## Como executar a aplicação

### 1. Subir o banco

```bash
docker compose up -d
```

### 2. Configurar o ambiente
O arquivo `.env` deve conter a URL do banco, por exemplo:

```env
DATABASE_URL=postgres://wallet_user:wallet_password@localhost:5432/wallet_db
```

### 3. Rodar as migrações

```bash
cargo sqlx migrate run
```

### 4. Iniciar a aplicação

```bash
cargo run
```

### 5. Acessar a aplicação

- login: `http://localhost:3000/login`
- carteira: `http://localhost:3000/assets`

---

## Exemplos de uso da API

A API fica no prefixo `/api`.

### Listar ativos

```bash
curl http://localhost:3000/api/assets
```

Resposta esperada:

```json
[
  {
    "id": 1,
    "name": "Bitcoin",
    "unit_value": 150000.0
  }
]
```

### Criar um ativo

```bash
curl -X POST http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -H "Authorization: im-the-admin" \
  -d '{
    "name": "Ethereum",
    "unit_value": 7500.0
  }'
```

### Atualizar um ativo

```bash
curl -X PATCH http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -H "Authorization: im-the-admin" \
  -d '{
    "id": 1,
    "name": "Bitcoin",
    "unit_value": 160000.0
  }'
```

### Login pela interface web
O fluxo funcional da aplicação web é feito por formulário em `/login`.

Exemplo:

```text
username=alice
password=minhasenha123
```

Após validar, o sistema gera o cookie de sessão e redireciona para `/assets`.

---

## Modelo do banco de dados

A persistência da aplicação foi estruturada em PostgreSQL com quatro entidades principais:

### 1. `users`
Representa cada usuário da carteira.

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
```

Campos principais:

- `id`: identificador único do usuário
- `username`: login do usuário
- `password_hash`: hash da senha para autenticação segura

### 2. `assets`
Representa os ativos disponíveis para compra na plataforma.

```sql
CREATE TABLE assets (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    unit_value DOUBLE PRECISION NOT NULL
);
```

Campos principais:

- `id`: identificador do ativo
- `name`: nome do ativo
- `unit_value`: valor unitário atual do ativo

### 3. `owned_assets`
Relaciona usuário com ativo e registra a quantidade possuída e o preço de aquisição.

```sql
CREATE TABLE owned_assets (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id),
    asset_id BIGINT NOT NULL REFERENCES assets(id),
    bought_for DOUBLE PRECISION NOT NULL,
    quantity_owned DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Esse modelo permite que o sistema saiba:

- quais ativos cada usuário possui
- quanto foi comprado
- qual quantidade está em carteira
- quando a compra foi registrada

### 4. `purchase_history`
Guarda o histórico de cada compra realizada por ativo.

```sql
CREATE TABLE purchase_history (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    asset_id BIGINT NOT NULL REFERENCES assets(id),
    bought_at TIMESTAMPTZ NOT NULL,
    bought_for DOUBLE PRECISION NOT NULL,
    quantity_bought DOUBLE PRECISION NOT NULL,
    value_delta DOUBLE PRECISION NOT NULL
);
```

Esse histórico permite rastrear as movimentações do investimento e exibir a evolução de cada compra individualmente.

### Relacionamentos principais

- um usuário tem muitos ativos possuídos
- um ativo pode estar em vários registros de carteira
- um ativo pode ter vários registros de histórico de compra

Em termos práticos: a carteira é a união entre `users`, `assets` e `owned_assets`; e o histórico de compra fica em `purchase_history` para acompanhar a evolução das aquisições.

---

## Melhorias implementadas durante o desafio

Entre as melhorias implementadas, destacam-se:

- migração do armazenamento em memória para PostgreSQL
- autenticação real com usuário e senha
- uso de JWT e cookie para manter sessão
- frontend SSR com Askama
- separação de frontend e API em rotas distintas
- uso de migrações para versionar o banco
- organização do código em layers claros
- adição de testes para operações principais

---

## Como testar a versão final

### Verificação de build

```bash
cargo build
```

### Execução local

```bash
cargo run
```

### Testes manuais

1. subir o banco
2. iniciar a aplicação
3. abrir `/login`
4. autenticar um usuário
5. acessar `/assets`
6. registrar uma compra
7. validar se o histórico aparece corretamente

---

## O que foi aprendido durante o desafio

Este projeto permitiu aprender, na prática:

- desenvolvimento web em Rust com Axum
- persistência com SQLx e PostgreSQL
- arquitetura de aplicações fullstack em uma única linguagem
- autenticação com cookies e JWT
- renderização de páginas com Askama
- modelagem e versionamento de bancos com migrações
- organização de projeto por módulos e camada de acesso a dados
- fluxo completo de backend + frontend em um mesmo app

---

## Conclusão

Este desafio entregou uma aplicação funcional de carteira de investimentos em Rust, integrando backend, frontend e banco de dados em uma solução coesa. O projeto foi importante para consolidar conceitos de arquitetura web, autenticação, persistência e renderização server-side em uma linguagem de alto desempenho.

Além disso, a estrutura criada oferece uma base sólida para evoluções futuras, como gráficos, filtros, relatórios, dashboards e integrações com dados externos.

---

## Entregável final

Este README foi preparado para servir como documentação de apresentação do projeto, cobrindo:

- contexto e proposta do desafio
- funcionalidades implementadas
- arquitetura e estrutura do sistema
- instruções de execução
- modelo de dados
- exemplos de uso da API
- aprendizado e impactos do desenvolvimento

