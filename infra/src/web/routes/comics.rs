use crate::web::routes::WebState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn get_comic(container: web::Data<WebState>, req: HttpRequest) -> HttpResponse {
    req.match_info().get("upc").map_or(
        HttpResponse::BadRequest().body("Missing parameter"),
        |upc| match container.query_comics.find(upc.to_string()) {
            Ok(opt) => match opt {
                Some(comic) => HttpResponse::Ok().json(comic),
                None => HttpResponse::NotFound().finish(),
            },
            Err(_e) => HttpResponse::InternalServerError().finish(),
        },
    )
}
