//! monolith file for now

use actix_web::{
    web, App, 
    HttpServer,
    HttpResponse, HttpRequest, 
    Responder
};
use url_query::URLquery;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct RadiationResponse {
    radiation: f64,
}

async fn radiation_response(request: HttpRequest) -> impl Responder {
    // TODO: rewrite and handle errors
    let query = request.uri().query();
    let result = match query {
        Some(args) => {
            let query = URLquery::from(args);
            let r_type = query["type"].as_ref()
                                    .expect("No type provided in query!");
            let latitude:f64 = query["lat"].as_ref()
                                    .expect("No latitude provided in query!").parse().expect("No f64 latitude provided in query!");
            let longtitude:f64 = query["long"].as_ref()
                                    .expect("No longtitude provided in query!")
                                    .parse()
                                    .expect("No f64 longtude provided in query!");
            latitude + longtitude as f64 // TODO: rewrite. It's only for demo
        },
        None => -1f64,
    };
    let r = RadiationResponse{radiation: result};

    HttpResponse::Ok().json(format!("{}", serde_json::to_string(&r).unwrap())) // unwrap due to handeled errors
}

struct AppConfig {
    bind_to: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sim", web::get().to(radiation_response))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
