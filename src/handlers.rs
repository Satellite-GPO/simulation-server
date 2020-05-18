use actix_web::{
    HttpRequest, HttpResponse
};

use serde::{Serialize, Deserialize};

use url_query::UrlQuery;
use super::solar::ground_radiation_for_year;

#[derive(Serialize, Deserialize)]
pub struct YearIrradiance {
    data: Vec<f64>,
}

fn check_result<T, Err, Pred, Handler>(pattern: Result<T, Err>, predicate: Pred, otherwise: Handler)
    -> Result<T, String>
where Pred: Fn(&T) -> bool,
    Handler: Fn(Result<T, Err>) -> String 
{
    match pattern {
        Ok(x) if predicate(&x) => Ok(x),
        other=> Err(otherwise(other)),
    }
}

// Original comments are marked with OR.CO.
pub async fn do_arloste(request: HttpRequest) -> Result<HttpResponse, String> {
    let query = UrlQuery::from( check_result (
            request.uri().query().ok_or("Nothing to handle: no query!"),
            |_| true, |other| String::from(other.unwrap_err()) // since predicate is always true we can be sure 'other' is Err
        )?
    );

    let latitude = check_result(query.get_of_type::<f64>("lat"), 
        |&x| -90.0 <= x && x <= 90.0,
        |other| {
            match other {
                Ok(x) => format!("Latitude {} doesn't fit\n", x),
                Err(msg) => format!("{}\n", msg),
            }
        })?;

    let t_min = check_result(query.get_of_type::<f64>("tmin"), 
        |&x| -273.15 <= x && x <= 100.0,
        |other| match other {
            Ok(x) => format!("Minimal temperature {} doesn't fit\n", x),
            Err(msg) => format!("{}\n", msg),
        })?;

    let t_max = check_result(query.get_of_type::<f64>("tmax"),
        |&x| t_min < x && x <= 100.0,
        |other| match other {
            Ok(x) => format!("Maximal temperature {} doesn't fit\n", x),
            Err(msg) => format!("{}\n", msg),
        })?;
    
    // TODO: think on leap year detection
    let daily_solar_rad = ground_radiation_for_year(latitude, t_min, t_max);

    let year_data = YearIrradiance{data:daily_solar_rad};

    Ok(HttpResponse::Ok()
        .json(format!("{}", match serde_json::to_string(&year_data) {
            Ok(x) => x,
            Err(_) => Err("Couldn't write json!")?
        }))
    )
}
