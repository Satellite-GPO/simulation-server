use std::{
    io,
    io::Write,
    str::FromStr,
};

use actix_web::{
    HttpRequest, HttpResponse,
    Responder,
};

use serde::{Serialize, Deserialize};

use url_query::UrlQuery;
use super::solar;

// TODO: think of adding this functionality to UrlQuery
// TODO: fix up weird behavior when key does not exist
fn get_of_type<'a, T>(query: &UrlQuery, name: &'a str) 
    -> Result<T, &'a str>
        where T: FromStr
{
    match query[name].as_ref() {
        Some(raw) => {
            match raw.parse() {
                Ok(value) => Ok(value),
                Err(_) => Err("Error parsing value"),
            }
        },
        None => {
            Err("No value in query")
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct YearIrradiance {
    data: Vec<f64>,
}

// Original comments are marked with OR.CO.
pub async fn do_arloste(request: HttpRequest) -> impl Responder {
    let mut log_stream = io::stdout();
    let default_response = HttpResponse::Ok().json(format!("¯\\_(ツ)_/¯"));

    let query = request.uri().query();
    let query = match query {
        Some(raw_values) => UrlQuery::from(raw_values),
        None => {
            log_stream.write("Nothing to handle: no request query!\n".as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        }
    };

    let latitude = match get_of_type::<f64>(&query, "lat") {
        Ok(x) if -90.0 <= x && x <= 90.0 => x.to_radians(),
        res => {
            let output = match res {
                Ok(x) => format!("Latitude {} doesn't fitn\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        },
    };

    let longtitude = match get_of_type::<f64>(&query, "long") {
        Ok(x) if -180.0 <= x && x <= 180.0 => x,
        res => {
            let output = match res {
                Ok(x) => format!("Longtitude {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        },
        
    };

    let slope = match get_of_type::<f64>(&query, "slope") {
        Ok(x) if 0.0 <= x && x <= 180.0 => x.to_radians(),
        res => {
            let output = match res {
                Ok(x) => format!("Slope {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        },
    };

    let aspect = match get_of_type::<f64>(&query, "aspect") {
        Ok(x) if 0.0 <= x && x <= 360.0 => x.to_radians(),
        res => {
            let output = match res {
                Ok(x) => format!("Aspect {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        },
    };

    // no viewshed for now

    let t_min: f64 = match get_of_type(&query, "tmin") {
        Ok(x) if -273.15 <= x && x <= 100.0 => x,
        res => {
            let output = match res {
                Ok(x) => format!("Minimal temperature {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
        },
    };

    let t_max: f64 = match get_of_type(&query, "tmax") {
        Ok(x) if t_min < x && x <= 100.0 => x,
        res => {
            let output = match res {
                Ok(x) => format!("Maximal temperature {} doesn't fit (minimal is {})\n", x, t_min),
                Err(msg) => format!("{}\n", msg),
            };
            log_stream.write(output.as_bytes()).unwrap();
            log_stream.flush().unwrap();
            return default_response;
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
                return default_response;
            },
        }
    ))
}