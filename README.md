## `who`

This project is created to **build a simple authentication service**, including:

- User registration
- User login
- Token generation (JWT)
- Token refreshing
- User logout
- Protected user info retrieval

Designed to be easily scalable and extensible for larger systems.

---

## ⚙️ Tech Stack

- **Golang** (Gin / Fiber web framework)
- **PostgreSQL** (for user storage)
- **JWT** (for authentication)
- **bcrypt** (for secure password hashing)
- **Docker** (for containerization)

---

## 🚀 API Endpoints

| Endpoint         | Method | Purpose                                    |
| :--------------- | :----- | :----------------------------------------- |
| `/auth/register` | POST   | Register a new user                        |
| `/auth/login`    | POST   | Login and receive access/refresh tokens    |
| `/auth/refresh`  | POST   | Refresh access token                       |
| `/auth/logout`   | POST   | Logout and invalidate refresh token        |
| `/auth/me`       | GET    | Get current user profile (protected route) |

---

## 🛠 Setup & Run

### 1. Clone the repository

```bash
git clone https://github.com/8Opt/who.git
cd who
```

### 2. Set up environment variables

Create a `.env` file in the project root:

```bash
DB_URL=postgres://username:password@localhost:5432/authdb?sslmode=disable
JWT_SECRET=your_jwt_secret_key
JWT_EXPIRATION_MINUTES=15
REFRESH_TOKEN_EXPIRATION_HOURS=168
```

### 3. Run PostgreSQL locally (optional via Docker)

```bash
docker run --name auth-postgres -e POSTGRES_PASSWORD=password -e POSTGRES_USER=username -e POSTGRES_DB=authdb -p 5432:5432 -d postgres
```

### 4. Run the application

```bash
go mod tidy
go run main.go
```

The service will be available at:  
```http
http://localhost:8080
```

---

## 🧩 Project Structure

```bash
who/

```


---

## 📄 License

This project is open-source under the [MIT License](LICENSE).
