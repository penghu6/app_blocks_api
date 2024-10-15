use actix_web::web::Json;
use actix_web::{get, post};
use actix_web::Error;
use crate::application::services::user_component_service::UserComponentService;
use crate::config::inject_config::get_it;
use crate::domain::model::user_component_model::UserComponentModel;

#[post("/user-component/create")]
pub async fn create() -> Result<Json<UserComponentModel>, Error> {
  let service = get_it::<UserComponentService>();
  let model = UserComponentModel {
    id: 0,
    react_code: "".to_string(),
    created_at: Default::default(),
    updated_at: Default::default(),
    version: 0,
    image_url: Some("".to_string()),
  };
  let result = service.create_user_component(model).await.expect("TODO: panic message");

  Ok(Json(result))
}