use std::path::PathBuf;

use http::{Request, Response};
use hyper::Body;
use mime_guess::MimeGuess;
use routerify::{ext::RequestExt, Router, RouterBuilder};
use swagger_ui::{Assets, Config, Spec};

const CONFIG_FILE_PATH: &str = "/swagger-ui-config.json";

struct SwaggerData {
    spec: Spec,
    config: Config,
}

pub fn swagger(spec: Spec, config: Config) -> Router<Body, hyper::Error> {
    let spec_name = spec.name.to_owned();
    let data = SwaggerData { spec: spec, config };
    let mut router = RouterBuilder::new()
        .data(data)
        .get("/swagger-ui-config.json", config_route)
        .get(spec_name, spec_route)
        .get("", index_route);

    for file in Assets::iter() {
        let filename = file.as_ref();
        router = router.get(format!("/{}", filename), assets);
    }

    router.build().unwrap()
}

async fn config_route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let swagger_data = req.data::<SwaggerData>().unwrap();

    let host = req.uri().path().replace(CONFIG_FILE_PATH, "");
    let mut config = swagger_data.config.to_owned();
    config.url = format!("{}/{}", host, &swagger_data.spec.name);
    let data = serde_json::to_string(&config).unwrap();

    Ok(Response::new(data.into()))
}

async fn spec_route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let swagger_data = req.data::<SwaggerData>().unwrap();

    let content_type = MimeGuess::from_path(&swagger_data.spec.name)
        .first()
        .unwrap();
    let content = Vec::from(swagger_data.spec.content);

    let response =
        Response::builder().header(http::header::CONTENT_ENCODING, content_type.as_ref());
    Ok(response.body(content.into()).unwrap())
}

async fn index_route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    let config_url = format!("{}{}", path, CONFIG_FILE_PATH);
    let index_url = format!("{}/index.html?configUrl={}", path, config_url);

    let response = Response::builder().header(hyper::http::header::LOCATION, index_url);
    Ok(response
        .status(http::StatusCode::FOUND)
        .body(Body::empty())
        .unwrap())
}

async fn assets(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = PathBuf::from(req.uri().path());
    let file = path.file_name().unwrap();

    let content_type = MimeGuess::from_path(file).first().unwrap();
    let content = match Assets::get(file.to_str().unwrap()) {
        Some(content) => content.to_owned(),
        None => {
            return Ok(Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    };

    Ok(Response::builder()
        .header(
            http::header::CONTENT_ENCODING,
            content_type.type_().as_str(),
        )
        .body(content.into())
        .unwrap())
}

// #[cfg(test)]
// mod tests {
//     use std::net::{SocketAddrV4, Ipv4Addr};

//     use hyper::server::conn::Http;
//     use routerify::{Router, RequestServiceBuilder};
//     use tokio::io::DuplexStream;

//     use crate::swagger;

//     fn serve() -> DuplexStream {
//         let (stream_1, stream_2) = tokio::io::duplex(1024);
//         Http::new().serve_connection(stream_2, RequestServiceBuilder::new(
//             Router::builder()
//                 .scope(
//                     "/api/v1/swagger",
//                     swagger(
//                         swagger_ui::swagger_spec_file!(
//                             "../../swagger-ui/examples/openapi.json"
//                         ),
//                         swagger_ui::Config {
//                             ..Default::default()
//                         },
//                     ),
//                 )
//                 .build()
//                 .unwrap()
//             ).unwrap().build(std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80)))
//         );
//         stream_1
//     }

//     #[tokio::test]
//     async fn swagger_ui() {
//         let stream = serve();

//         let connector = Connect

//         let client = Client::;

//         let response = client
//             .get(Uri::from_static("/api/v1/swagger"))
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::SEE_OTHER);

//         let response = client
//             .get(Uri::from_static("/api/v1/swagger/index.html"))
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::OK);

//         let response = client
//             .get(Uri::from_static("/api/v1/swagger/swagger-ui-config.json"))
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::OK);

//         let response = client
//             .get(Uri::from_static("/api/v1/swagger/openapi.json"))
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::OK);

//         let path = env!("CARGO_MANIFEST_DIR").to_string() + "/../swagger-ui/examples/openapi.json";

//         println!("Loading {}", path);

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         assert_eq!(body, std::fs::read(path).unwrap().as_slice());
//     }
// }
