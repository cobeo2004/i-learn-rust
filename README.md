# Rust REST API

A learning project for building REST APIs with Rust, Axum, and PostgreSQL.

## About This Project

This is my journey learning to build production-ready REST APIs using Rust. The project demonstrates:

- Asynchronous web server with Axum framework
- PostgreSQL database integration with SeaORM
- Database migrations management
- Environment-based configuration
- Structured logging

**Status**: 🚧 Early development - currently implementing CRUD operations

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8.8
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **Database**: PostgreSQL 15
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) 2.0.0-rc
- **Logging**: [tracing](https://github.com/tokio-rs/tracing)

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://www.docker.com/get-started) and Docker Compose
- [PostgreSQL](https://www.postgresql.org/) client tools (optional)

## Getting Started

### 1. Clone and Setup

```bash
git clone <your-repo-url>
cd rust-api
```

### 2. Configure Environment

Copy the example environment file and update if needed:

```bash
cp .env.example .env
```

Default configuration:
- Server: `http://localhost:5173`
- Database: `postgres://postgres:password@localhost:5432/rust_api`

### 3. Start PostgreSQL

```bash
docker-compose up -d
```

This starts a PostgreSQL 15 container with the credentials from your `.env` file.

### 4. Run Database Migrations

```bash
cd migration
cargo run -- fresh
cd ..
```

This creates the initial database schema (users table).

### 5. Start the API Server

```bash
cargo run
```

The server will start at `http://localhost:5173`

### 6. Test It Out

```bash
curl http://localhost:5173/
```

Expected response: `202 Accepted`

## Project Structure

```
rust-api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Module exports
│   ├── routes/              # HTTP routes and handlers
│   │   ├── server.rs        # Router configuration
│   │   └── root/            # Root endpoint handlers
│   ├── infra/               # Infrastructure layer
│   │   └── db.rs            # Database connection
│   └── utils/               # Helper functions
│       └── env.rs           # Environment variables
├── migration/               # Database migrations workspace
│   └── src/                 # Migration definitions
├── docker-compose.yml       # PostgreSQL setup
└── .env                     # Environment configuration
```

## Development Commands

### Running the Server

```bash
# Development mode (with recompilation)
cargo run

# Release mode (optimized)
cargo build --release
./target/release/rust-api
```

### Database Management

All migration commands run from the `migration/` directory:

```bash
cd migration

# Create a new migration
cargo run -- generate <migration_name>

# Apply all pending migrations
cargo run -- up

# Rollback the last migration
cargo run -- down

# Reset database (rollback all + reapply all)
cargo run -- refresh

# Check migration status
cargo run -- status
```

### Development Tools

```bash
# Check code without building
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Current Features

- ✅ Basic server setup with Axum
- ✅ PostgreSQL database connection
- ✅ Migration system with SeaORM
- ✅ Environment-based configuration
- ✅ Structured logging
- ✅ Users table schema

## What I'm Learning

- Rust's async/await patterns with Tokio
- Building type-safe APIs with Axum
- Database operations and migrations with SeaORM
- Error handling in Rust web applications
- Project structure and module organization
- Docker containerization for databases

## Next Steps

- [ ] Implement user CRUD endpoints (Create, Read, Update, Delete)
- [ ] Add proper error handling and validation
- [ ] Write integration tests
- [ ] Add authentication/authorization
- [ ] API documentation with OpenAPI/Swagger
- [ ] Containerize the application

## Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SeaORM Tutorial](https://www.sea-ql.org/SeaORM/docs/index/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust Book](https://doc.rust-lang.org/book/)

## License

This is a personal learning project.
