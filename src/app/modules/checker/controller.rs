use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::providers::guards::claims::AccessClaims;

use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::app::providers::interfaces::paper::PubPaperPush;

use crate::app::modules::checker::services::helper;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        post_project_checker,
        post_project_checker_none
    ]
}

#[options("/")]
pub fn options_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub fn options_show(_id: i32) -> Status {
    Status::Ok
}

#[post("/", data = "<paper>", rank = 1)]
pub async fn post_project_checker(fetch: &State<Fetch>, claims: AccessClaims, paper: Json<PubPaperPush>) -> Result<Json<PubPaperPush>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => helper::send_to_checker(fetch, paper.into_inner()).await,
        "robot" => helper::send_to_checker(fetch, paper.into_inner()).await,
        _ => {
            println!("Error: post_project_checker; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_paper>", rank = 2)]
pub async fn post_project_checker_none(_paper: Json<PubPaperPush>) -> Status {
    Status::Unauthorized
}
