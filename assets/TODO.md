# TODO - Auth Service Development Roadmap

## 📦 Core Setup
- [x] Initialize Go project (go mod init)
- [x] Set up project folder structure
- [ ] Connect to PostgreSQL database
- [ ] Create User model (with GORM or SQLx)

## 🔒 Authentication and Security
- [ ] Create JWT utilities (token generation, token validation)
- [ ] Implement password hashing utilities (bcrypt)

## 🛠 API Handlers
- [ ] Setup basic Fiber/Gin server
- [ ] Write handlers for JWT middleware (token checking)
- [ ] Implement `POST /api/v1/auth/register` (User Registration)
- [ ] Implement `POST /api/v1/auth/login` (User Login & Token Issuance)
- [ ] Implement `POST /api/v1/auth/refresh` (Access Token Renewal)
- [ ] Implement `POST /api/v1/auth/logout` (Invalidate Refresh Token)
- [ ] Implement `GET /api/v1/auth/me` (Get current user info)

## 🧹 Validation and Error Handling
- [ ] Validate request body inputs (email, password strength, etc.)
- [ ] Standardize API responses (success/error format)
- [ ] Add proper HTTP status codes (201, 400, 401, etc.)

## 🗃️ Database Integration
- [ ] Migrate database schema (auto-migration or SQL scripts)
- [ ] Create RefreshToken table/model (for secure logout/refresh)
- [ ] Write DB query helpers (FindUserByEmail, SaveUser, etc.)

## 🛡️ Middleware
- [ ] Create Auth middleware (JWT parsing, user injection into context)
- [ ] Add CORS middleware (for frontend connection)

## 🧪 Testing
- [ ] Write unit tests for handlers
- [ ] Write integration tests for full auth flow
- [ ] Set up mock DB/testing containers (Testcontainers-Go)

## ⚙️ DevOps
- [ ] Create Dockerfile for the service
- [ ] Create `docker-compose.yml` to run app + Postgres locally
- [ ] Set up `.env` file loading (Viper or custom config loader)

## 🚀 Deployment
- [ ] Set up basic CI/CD pipeline (GitHub Actions)
- [ ] Create staging and production environment config templates
- [ ] Healthcheck endpoint (`GET /api/v1/auth/healthz`)

## 📈 Monitoring (Future)
- [ ] Integrate simple Prometheus metrics (requests, errors)
- [ ] Add logs (Zap or Logrus)

## 📝 Documentation
- [ ] Create API documentation (Swagger / OpenAPI spec)
- [ ] Update README.md as features are completed

