## 📌 TODOs and Future Enhancements

- OAuth2 (Google, Facebook login)
- Email verification
- Password reset flow
- Admin user management
- Docker-Compose orchestration
- Prometheus + Grafana monitoring


---

## 🔒 Security Features

- Passwords are securely hashed using **bcrypt**.
- JWT tokens are signed using **strong secret keys**.
- Refresh tokens are securely stored and validated.
- Rate limiting (recommended to implement) for login endpoints.

--- 

## :memo: Golang's Interface

In Go (Golang), there are no "classes" like in Python, Java, or C++. Instead, Go uses:
+ structs: These are used to group data together (like fields or attributes).
+ methods: These are functions that are attached to a struct. They can work with the data inside the struct.

Go also has interfaces, which are just a list of method names.
To "implement" an interface, a struct needs to have all the methods listed in that interface.

When you call a method from an interface, you can pass a variable of a struct type — as long as that struct has already implemented the needed methods.