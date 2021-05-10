use crate::constants::*;

use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};

// r#type for escaping the rust-type command to normal type string

#[derive(Deserialize, Serialize)]
pub struct Point {
    pub latitude: Angle,
    pub longitude: Angle,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    pub cost: Option<String>,
    pub alpha: Option<Vec<f64>>,
}

// request are two points
#[derive(Deserialize, Serialize, Debug)]
pub struct GeometryRequest {
    pub r#type: String,
    pub coordinates: Vec<Angle>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeatureRequest {
    pub r#type: String,
    pub properties: Option<Property>,
    pub geometry: GeometryRequest,
}

#[derive(Deserialize, Serialize)]
pub struct GeoJsonRequest {
    pub r#type: String,
    pub features: Vec<FeatureRequest>,
}

// response is array of tuples
#[derive(Deserialize, Serialize, Debug)]
pub struct GeometryResponse {
    pub r#type: String,
    pub coordinates: Vec<(Angle, Angle)>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeatureResponse {
    pub r#type: String,
    pub properties: Option<Property>,
    pub geometry: GeometryResponse,
}

#[derive(Deserialize, Serialize)]
pub struct GeoJsonResponse {
    pub r#type: String,
    pub features: Vec<FeatureResponse>,
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub msg: String,
    pub status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}
