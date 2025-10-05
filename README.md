# Axum + SeaORM + PostgreSQL Tutorial

A working tutorial for building a **REST API** with:

* **Axum** (web framework)
* **SeaORM** (type-safe ORM)
* **PostgreSQL** (database)
* **Docker & Compose** (local dev + deployment)

> A complete, working example that you can run and modify.

For a comprehensive guide with detailed explanations, code walkthroughs, and best practices, check out the full [Medium article](https://medium.com/@scraiber/building-lightning-fast-apis-with-rust-a-complete-axum-seaorm-postgresql-tutorial-5a196dab86a1).

---

## Table of Contents

1. [What You'll Build](#what-youll-build)
2. [Prerequisites](#prerequisites)
3. [Quick Start](#quick-start)
4. [Project Structure](#project-structure)
5. [How It Works](#how-it-works)
6. [Development](#development)
7. [Production Deployment](#production-deployment)
8. [API Usage](#api-usage)

---

## What You'll Build

A **CRUD API for Users** with:

* Axum routes and handlers
* SeaORM entities & queries
* PostgreSQL database
* Docker setup with hot reload

---

## Prerequisites

* **Docker** and **Docker Compose**
* **Git** (to clone the repository)

---

## Quick Start

### 1) Clone and run

```bash
git clone https://github.com/scraiber/axum-seaorm-tutorial.git
cd axum-seaorm-tutorial
docker-compose up --build
```

This will:
* Start PostgreSQL database
* Run the initial migration (creates the `users` table)
* Build and start the Axum API
* Expose the API at **http://localhost:3000** (this may take a few minutes due to compilation, you can check via `docker-compose logs app`)

### 2) Test the API

```bash
curl http://localhost:3000/
# {"status":"ok"}
```

The API is ready! 🎉

---

## Project Structure

```
axum-seaorm-tutorial/
├── src/
│   ├── entities/          # SeaORM entity definitions
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── handlers.rs        # HTTP handlers (business logic)
│   └── main.rs            # App entry: router, DB, tracing, server
├── migrations/
│   └── init.sql           # Initial schema (users table, indexes)
├── Dockerfile             # Production image
├── Dockerfile.dev         # Dev image with hot-reload
├── docker-compose.yml     # App + DB orchestration
├── Cargo.toml             # Rust dependencies + metadata
├── manual.txt             # Manual testing commands
└── README.md              # This tutorial
```

**Key files:**

* **`src/main.rs`** - App setup: database connection, routes, middleware
* **`src/handlers.rs`** - HTTP handlers for CRUD operations
* **`src/entities/user.rs`** - SeaORM model for the `users` table
* **`migrations/init.sql`** - Database schema
* **`docker-compose.yml`** - Development environment setup

---

## How It Works

1. **Request** hits Axum route (e.g., `POST /users`)
2. Axum **extractors** parse JSON and path parameters
3. **Handler** calls SeaORM **Entity/ActiveModel** functions
4. SeaORM generates SQL and talks to **PostgreSQL** via the **DB pool**
5. Result is serialized to JSON and returned with proper **status codes**
6. **tracing** logs request/response metadata

---

## Development

### Hot Reload

The development setup includes **hot reload** - when you edit files in the `src/` directory, the application automatically recompiles and restarts. This is powered by `cargo-watch` in the `Dockerfile.dev`.

**How it works:**
- Edit any file in `src/` (like `handlers.rs` or `main.rs`)
- Save the file
- The container detects the change and recompiles
- The API restarts with your changes
- No need to manually restart Docker

**Example - Test hot reload:**
1. Edit `src/handlers.rs` and change line 53 from:
   ```rust
   status: "ok".to_string(),
   ```
   to:
   ```rust
   status: "ok after hot reload".to_string(),
   ```
2. Save the file
3. Wait a few seconds for recompilation
4. Test: `curl http://localhost:3000/`
5. You should see: `{"status":"ok after hot reload"}`

### Start/stop

```bash
# Start with hot reload
docker-compose up --build

# Stop everything
docker-compose down

# Clean volumes (start fresh DB)
docker-compose down -v
```

### View logs

```bash
# App logs (live)
docker-compose logs -f app

# DB logs
docker-compose logs -f db
```

### Database shell

```bash
# Connect to the database
docker-compose exec db psql -U postgres -d axum_seaorm
```

> **Pro tip:** Keep the app logs open in a terminal while coding. You'll see recompilation messages when you save changes, and any compilation errors will be displayed immediately.

---

## Production Deployment

### 🚀 Ultra-Minimal Production Container (10MB!)

This project includes an optimized production Dockerfile that creates an **extremely small and secure** container:

**Key Features:**
- **~10MB total size** (vs hundreds of MB for typical containers!)
- **Scratch-based**: No OS, no shell, no package manager
- **Static linking**: Self-contained binary with no runtime dependencies
- **Maximum security**: Minimal attack surface
- **Fast startup**: Minimal overhead

### Build Production Image

```bash
# Build the production image
docker build -t axum-seaorm:prod -f Dockerfile .

# Check the image size (should be ~10MB!)
docker images axum-seaorm:prod
```

### Run Production Container

```bash
# Start your database first
docker-compose up -d db

# Run the production container
docker run -d \
  --name axum-seaorm-prod \
  --network axum-seaorm-tutorial_default \
  -p 3000:3000 \
  -e DATABASE_URL="postgres://postgres:postgres@axum-seaorm-db:5432/axum_seaorm" \
  -e RUST_LOG="axum_seaorm=info,tower_http=info" \
  axum-seaorm:prod
```

The API usage is the same as the development environment, see [API Usage](#api-usage).

### Production Container Management

```bash
# View logs
docker logs axum-seaorm-prod

# Stop container
docker stop axum-seaorm-prod

# Remove container
docker rm axum-seaorm-prod
```

---

## API Usage

**Base URL:** `http://localhost:3000`

| Method | Endpoint      | Description    |
| -----: | ------------- | -------------- |
|    GET | `/`           | Health check   |
|   POST | `/users`      | Create user    |
|    GET | `/users`      | List users     |
|    GET | `/users/{id}` | Get user by ID |
|    PUT | `/users/{id}` | Update user    |
| DELETE | `/users/{id}` | Delete user    |

### Health Check

```bash
curl http://localhost:3000/
# {"status":"ok"}
```

### Create User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'
```

### List Users

```bash
curl http://localhost:3000/users
```

### Get User by ID

```bash
curl http://localhost:3000/users/1
```

### Update User

```bash
curl -X PUT http://localhost:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe"}'
```

### Delete User

```bash
curl -X DELETE http://localhost:3000/users/1 -i
# HTTP/1.1 204 No Content
```
