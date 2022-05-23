use actix_web::{
    get, 
    post, 
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[get("/route/{routeId}")]
pub async fn getRoute(routeId: Path<u32>) -> Json<String> {
    let result = format!("Route id: {}", routeId);
    return Json(result);
}