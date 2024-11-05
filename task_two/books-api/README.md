# Task two

## WebSocket Books API

Description: Develop a small WebSocket API in Rust using a lightweight web framework like Actix-web or Rocket. The API should provide basic CRUD (Create, Read, Update, Delete) operations for managing a list of books, where each book has a title, author, and year of publication.

**To build and run:**
  
  ```bash
  cargo build
  cargo run
  ```

**To test:**

  ```bash
  cargo test
  ```

**Linter:**

  ```bash
  cargo clippy
  ```


**Legacy integration tests using Python:**

Now you can directly run `cargo test`, but initially I had some problems with `setup_test_server` in Rust and was limited by time, so I wrote some integration tests in Python.

  ```bash
  # assumes you have python3 and pip3 installed
  cd integration_tests; chmod +x setup.sh; ./test.sh
  ```