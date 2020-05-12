use std::{
    io,
    io::Write,
};

use actix_web::{
    HttpRequest, HttpResponse,
    Responder,
};

use serde::{Serialize, Deserialize};

use url_query::UrlQuery;
use super::solar;

#[derive(Serialize, Deserialize)]
pub struct YearIrradiance {
    data: Vec<f64>,
}

// Original comments are marked with OR.CO.
pub async fn do_arloste(request: HttpRequest) -> impl Responder {
    let mut log_stream = io::stdout();
    let fallback_response = HttpResponse::Ok().json(format!("¯\\_(ツ)_/¯")); // TODO: think of another appropriate default fallback response

    let query = request.uri().query();
    let query = match query {
        Some(raw_values) => UrlQuery::from(raw_values),
        None => {
            log_stream.write("Nothing to handle: no request query!\n".as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return fallback_response;
        }
    };

    let latitude = match query.get_of_type::<f64>("lat") {
        Ok(x) if -90.0 <= x && x <= 90.0 => x.to_radians(),
        res => {
            let output = match res {
                Ok(x) => format!("Latitude {} doesn't fitn\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return fallback_response;
        },
    };

    let t_min: f64 = match query.get_of_type::<f64>("tmin") {
        Ok(x) if -273.15 <= x && x <= 100.0 => x,
        res => {
            let output = match res {
                Ok(x) => format!("Minimal temperature {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return fallback_response;
        },
    };

    let t_max: f64 = match query.get_of_type::<f64>("tmax") {
        Ok(x) if t_min < x && x <= 100.0 => x,
        res => {
            let output = match res {
                Ok(x) => format!("Maximal temperature {} doesn't fit (minimal is {})\n", x, t_min),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return fallback_response;
        },
    };
    
    // TODO: think on leap year detection
    let daily_solar_rad = solar::ground_radiation_for_year(latitude, t_min, t_max);

    let year_data = YearIrradiance{data:daily_solar_rad};

    HttpResponse::Ok().json(format!("{}",
        match serde_json::to_string(&year_data) {
            Ok(x) => x,
            Err(_) => {
                log_stream.write("Couldn't write json!".as_bytes()).unwrap();
                return fallback_response;
            },
        }
    ))
}