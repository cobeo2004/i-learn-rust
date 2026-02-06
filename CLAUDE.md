# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **Rust REST API** built with Axum web framework and Tokio async runtime. It uses PostgreSQL for persistence via SeaORM ORM, with a workspace structure that includes a dedicated migration project for database schema management.

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.8
- **Database**: PostgreSQL 15 with SeaORM 2.0.0-rc
- **Configuration**: Environment variables via dotenv

## Architecture

The project follows a **layered architecture**:

```
main.rs (server bootstrap)
  └→ routes/server.rs (router configuration)
      └→ routes/root/root_handler.rs (endpoint handlers)
          └→ infra/db.rs (database infrastructure)
```

**Key layers:**

- **routes/**: HTTP route definitions and handlers. Entry point is `routes/server.rs` which registers routes and returns the Axum router.
- **infra/**: Infrastructure concerns, specifically database connection management in `infra/db.rs`. Provides `get_db()` function to access the SeaORM connection.
- **utils/**: Helper functions like environment variable management in `utils/env.rs`.
- **migration/**: Separate workspace with CLI for generating, applying, and rolling back database migrations.

**Module structure**: Each directory uses `mod.rs` for module declaration and exports. The `lib.rs` exports public modules for use in `main.rs`.

## Common Commands

### Development

```bash
# Start local PostgreSQL database
docker-compose up -d

# Run database migrations (from project root)
cd migration && cargo run -- fresh

# Start the API server
cargo run

# Test root endpoint (default: http://localhost:5173/)
curl http://localhost:5173/
```

### Database Migrations

Run migration commands from the `migration/` directory:

```bash
# Generate new migration file
cargo run -- generate create_users_table

# Apply all pending migrations
cargo run -- up

# Rollback last migration
cargo run -- down

# Rollback and reapply all migrations (useful for testing)
cargo run -- refresh

# Check migration status
cargo run -- status
```

### Building and Testing

```bash
# Build project
cargo build

# Build optimized release binary
cargo build --release

# Run tests
cargo test

# Quick syntax check without compilation
cargo check
```

## Configuration

Configuration is managed via environment variables in `.env`:

```
SERVER_PATH=0.0.0.0
SERVER_PORT=5173
DATABASE_URL=postgres://postgres:password@localhost:5432/rust_api
```

The server binds to `SERVER_PATH:SERVER_PORT`. Use `0.0.0.0` for Docker/production, `127.0.0.1` for local development.

## Database Schema

Currently defines a `users` table:
- `id` (integer, auto-increment, primary key)
- `name` (string, required)
- `email` (string, unique, required)
- `password` (string, required)
- `uuid` (UUID, unique, required)
- `created_at` (datetime, required)

New tables are added via migrations in the `migration/src/` directory.

## Key Dependencies

- **axum**: Web framework - route handling, middleware, response types
- **tokio**: Async runtime with full feature set (multi-threading, networking, timers)
- **sea-orm**: Type-safe ORM - query building, migrations, entity models
- **serde/serde_json**: Serialization for JSON request/response bodies
- **dotenv**: Environment variable loading from `.env` file
- **tracing-subscriber**: Structured logging output

## Development Notes

- **Workspace structure**: The root `Cargo.toml` includes both the main app and the `migration` workspace member. They share dependencies but are separate binaries.
- **SeaORM integration**: Not yet integrated with route handlers. The `get_db()` function exists in `infra/db.rs` but handlers don't currently use it.
- **Error handling**: Currently uses `.unwrap()` in main.rs for configuration loading and server startup. Add proper error handling for production use.
- **Logging**: Initialized in main.rs via `tracing_subscriber::fmt::init()`. Use `tracing` macros (info!, warn!, error!) for structured logs.
- **Current state**: Early development (v0.1.0). Root endpoint returns 202 ACCEPTED. Ready for expanding with CRUD operations and additional routes.
