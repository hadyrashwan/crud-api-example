# Rust CRUD API Example

This is a simple CRUD API example built using Rust. The API interacts with a PostgreSQL database to perform Create, Read, Update, and Delete operations on a `users` table.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- PostgreSQL (https://www.postgresql.org/download/)

## Setup

1. **Clone the repository:**

    ```sh
    git clone https://github.com/your-username/rust_crud.git
    cd rust_crud
    ```

2. **Set the `DATABASE_URL` environment variable:**

    ```sh
    export DATABASE_URL=postgres://postgres:password@localhost:5432/postgres
    ```

3. **Run the Rust API server:**

    ```sh
    cargo run
    ```

## API Endpoints

- **Create a User:**

    ```sh
    POST /users
    ```

    Request Body:

    ```json
    {
        "name": "John Doe",
        "email": "john.doe@example.com"
    }
    ```

- **Get a User by ID:**

    ```sh
    GET /users/{id}
    ```

- **Get All Users:**

    ```sh
    GET /users
    ```

- **Update a User:**

    ```sh
    PUT /users/{id}
    ```

    Request Body:

    ```json
    {
        "name": "Jane Doe",
        "email": "jane.doe@example.com"
    }
    ```

- **Delete a User:**

    ```sh
    DELETE /users/{id}
    ```

## Contributing

Feel free to open issues and pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License.