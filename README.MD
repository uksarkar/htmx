# HTMX Single Page Application with Rust's Axum

## Summary

This project demonstrates how to build a single page application (SPA) using [HTMX](https://htmx.org/) on top of Rust's [Axum](https://github.com/tokio-rs/axum) framework. It covers Axum's routing and middleware, and utilizes Handlebars for templating in Rust.

## Features

- **Axum Framework**: Utilizes Axum for routing and middleware.
- **HTMX Integration**: Leverages HTMX for building dynamic and interactive web pages without JavaScript.
- **Handlebars Templating**: Implements Handlebars for server-side templating.

## Prerequisites

- Rust (installed via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

## Running the Application

To run the application normally:

```sh
cargo run
```

To run the application in watch mode (automatically restarts the server when code changes are detected):

```sh
cargo watch -c -x run
```

## Getting Started

- Clone the repository:

```sh
git clone https://github.com/uksarkar/htmx.git
cd htmx
```

- Install dependencies:
  The necessary dependencies will be installed automatically when you run the application using `cargo run`.
- Run the application:
  `cargo run` / `cargo watch -c -x run`
- Access the application:
  Open your web browser and navigate to http://localhost:3000 to see the application in action.

## Project Structure

  -`src/main.rs`: Entry point of the application.
  -`src/routes.rs`: Defines the application routes.
  -`src/middleware.rs`: Contains middleware logic.
  -`templates/`: Directory for Handlebars templates.

## Tree view

```sh
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.MD
├── src
│   ├── db.rs
│   ├── main.rs
│   ├── pagination.rs
│   ├── response.rs
│   └── routes.rs
└── templates
    ├── 404.hbs
    ├── about.hbs
    ├── index.hbs
    └── user
        ├── list.hbs
        └── view.hbs
```
## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

> This `README.md` provides an overview of the project, instructions for running the application, and information on the project structure and contributing.
