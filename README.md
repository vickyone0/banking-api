# Banking API

A simple banking API built with **Rust**, **Actix Web**, and **SQLx** for PostgreSQL.  
It supports user registration, authentication (JWT), balance management, and transactions.

---

## Features

- User registration and login (with password hashing)
- JWT-based authentication
- Account balance tracking
- Credit and debit transactions
- View transaction history
- RESTful API structure
- Logging middleware

---

## Getting Started

### Prerequisites

- Rust (https://rustup.rs)
- PostgreSQL (local or Docker)
- [sqlx-cli](https://crates.io/crates/sqlx-cli) (for running migrations)

### Setup

1. **Clone the repository:**
   ```sh
   git clone https://github.com/yourusername/banking-api.git
   cd banking-api
   ```

2. **Set up environment variables:**

   Create a `.env` file in the project root:
   ```
   DATABASE_URL=postgres://postgres:password@localhost/banking
   JWT_SECRET=your_jwt_secret
   ```

3. **Run database migrations:**
   ```sh
   sqlx migrate run
   ```

4. **Build and run the server:**
   ```sh
   cargo run
   ```

   The API will be available at [http://localhost:8080](http://localhost:8080).

---

## API Endpoints

### Public

- `POST /api/register` — Register a new user
- `POST /api/login` — Login and receive a JWT

### Protected (require `Authorization: Bearer <token>`)

- `GET /api/profile` — Get user profile
- `PUT /api/profile` — Update user profile
- `POST /api/transactions` — Create a transaction (credit/debit)
- `GET /api/transactions` — List user transactions
- `GET /api/balance` — Get account balance

---

## Testing

Run unit tests with:

```sh
cargo test
```

---

## Example Requests

**Register:**
```sh
curl -X POST http://localhost:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password123"}'
```

**Login:**
```sh
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'
```

**Create Transaction (requires JWT):**
```sh
curl -X POST http://localhost:8080/api/transactions \
  -H "Authorization: Bearer <JWT_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"amount":1000,"transaction_type":"credit","description":"Deposit"}'
```

---

## Project Structure

```
src/
  ├── auth/           # JWT and middleware
  ├── models/         # Database models
  ├── routes/         # API route handlers
  ├── main.rs         # Application entry point
  └── ...
migrations/           # SQLx migration scripts
```

---

## License

MIT

---

**Contributions welcome!**
