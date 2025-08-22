## How to build & run (local development)

1. Prepare Postgres and create database. Set `DATABASE_URL` env var (e.g. `postgres://user:pass@localhost/tododb`)
2. `cd backend` and run `cargo build` (you can `cargo run` after copying static files)
3. `cd frontend` and run `npm install` then `npm run build`.
4. Copy `frontend/dist` (Vite) or `frontend/build` into `backend/static`.
5. Set `JWT_SECRET` env var for production.
6. Run backend: `cd backend && cargo run` and open `http://127.0.0.1:8080/`.

---

## Security & production notes
- Hash passwords using Argon2 when creating users (I left user-creation out of the sample but you must implement signup or seed users in DB). Do NOT store plain passwords.
- Use HTTPS + secure cookies in production.
- Use database migrations (e.g. refinery or sqlx-cli) instead of manual SQL for production.
- Harden JWT secret and token expiry, consider refresh tokens.