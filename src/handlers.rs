use actix_web::{
    HttpRequest, HttpResponse,
    web::Json
};

use actix_files::NamedFile;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use url_query::UrlQuery;
use super::solar::ground_radiation_for_year;

#[derive(Deserialize)]
pub struct PostRequest {
    lat: f64,
    lng: f64,
    from: String,
    to: String,
}

#[derive(Serialize)]
pub struct YearIrradiance {
    error_code: u8,
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

pub async fn initial_page(_: HttpRequest) -> HttpResponse {
    let contents = std::fs::read_to_string( "./index.html"/* "../gpo-webgui/build/index.html" */)
        .expect("Index failed!");

    println!("Index read");
    return HttpResponse::Ok().body(contents)
}

pub async fn static_files(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();

    println!("path to static file: {:?}", path);
    Ok(NamedFile::open(path)?)
}

// TODO: make it return result
pub async fn do_arloste_post(request: Json<PostRequest>) -> HttpResponse {
    let latitude = request.lat;
    let t_min = 18_f64; // TODO: retrieve temperatures from files
    let t_max = 22_f64;
    let answer = YearIrradiance {
        error_code: 0,
        data: ground_radiation_for_year(latitude, t_min, t_max),
    };

    // let res_json = 
    // match serde_json::ser::to_string(&answer) {
    //     Ok(x) => x,
    //     Err(_) => panic!("\n--MAYDAY! MAYDAY! We couldn't serialize json!\n--Понял, вычеркиваю."),
    // };

    // let final_json = String::from_utf8(res_json.as_bytes()
    //     .iter()
    //     .filter(|&&c| c != b'\\').map(|x| x.clone())
    //     .collect::<Vec<u8>>()).unwrap();

    let final_json = serde_json::json!({
        "error_code": 0,
        "data": answer.data
    });

    println!("Doing arloste post\n{:?}", &final_json);
    
    HttpResponse::Ok().json(final_json)
}

// Original comments are marked with OR.CO.
pub async fn do_arloste_get(request: HttpRequest) -> Result<HttpResponse, String> {
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

    let year_data = YearIrradiance{data:daily_solar_rad, error_code: 0};

    Ok(HttpResponse::Ok()
        .json(format!("{}", match serde_json::to_string(&year_data) {
            Ok(x) => x,
            Err(_) => Err("Couldn't write json!")?
        }))
    )
}
