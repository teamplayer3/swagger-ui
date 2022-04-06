use hyper::Server;
use routerify::{Router, RouterService};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = Server::bind(&"127.0.0.1:88".parse().unwrap()).serve(
        RouterService::new(
            Router::builder()
                .scope(
                    "/api/v1/swagger",
                    hyper_routerify_swagger_ui::swagger(
                        swagger_ui::swagger_spec_file!("../../swagger-ui/examples/openapi.json"),
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
