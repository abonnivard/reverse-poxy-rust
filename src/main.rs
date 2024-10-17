use hyper::{Body, Client, Request, Response, Server, service::{make_service_fn, service_fn}};
use hyper::client::HttpConnector;
use std::convert::Infallible;

async fn forward_request(req: Request<Body>, client: Client<HttpConnector>) -> Result<Response<Body>, hyper::Error> {
    // Cloner les éléments nécessaires avant d'utiliser `req.into_body()`
    let method = req.method().clone();
    let uri_string = format!("http://127.0.0.1:8000{}", req.uri().path());
    let uri = uri_string.parse().expect("URI invalide");

    // Créer une nouvelle requête vers le serveur backend
    let mut new_req = Request::new(req.into_body());
    *new_req.method_mut() = method;
    *new_req.uri_mut() = uri;

    client.request(new_req).await
}

#[tokio::main]
async fn main() {
    // Initialiser le client HTTP
    let client = Client::new();

    // Créer le serveur proxy
    let make_svc = make_service_fn(move |_| {
        let client = client.clone();  // Cloner ici pour éviter les références empruntées
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                forward_request(req, client.clone())  // Cloner à nouveau pour chaque requête
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 80).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Reverse proxy démarré sur http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Erreur du serveur : {}", e);
    }
}
