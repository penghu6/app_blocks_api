use actix_web::web::Json;
use actix_web::get;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Day {
  name:String,
  date:String,
}

use actix_web::Error;


#[get("/home")]
pub async fn home() -> Result<Json<Day>, Error> {
  let day = Day {
    name: String::from("星期一"),
    date: String::from("2023-05-01"),
  };
  Ok(Json(day))
}