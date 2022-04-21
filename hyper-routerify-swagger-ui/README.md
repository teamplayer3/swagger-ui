# Hyper-Routerify swagger-ui Implementation

This library provides an implementation for [routerify](https://crates.io/crates/routerify) used with [hyper](https://crates.io/crates/hyper).

# Usage Example

Include the crate in `Cargo.toml`.
```toml
hyper-routerify-swagger-ui = "0.1.0"
```

Serve it with hyper and routerify
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = Server::bind(&"127.0.0.1:88".parse().unwrap()).serve(
        RouterService::new(
            Router::builder()
                .scope(
                    "/api/v1/swagger",
                    hyper_routerify_swagger_ui::swagger(
                        swagger_ui::swagger_spec_file!("../assets/openapi.json"),
                        swagger_ui::Config {
                            ..Default::default()
                        },
                    ),
                )
                .build()
                .unwrap(),
        )
        .unwrap(),
    );

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

Connect by entering the following URL in the browser
```
http://127.0.0.1:88/api/v1/swagger
```