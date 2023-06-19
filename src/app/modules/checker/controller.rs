use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::models::paper::{PubPaperPush, PubPaper};
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::checker::services::helper;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        get_project_checker,
        get_project_checker_none,
        post_project_checker,
        post_project_checker_none,
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

#[get("/<name>/project/<id>", rank = 101)] // Maybe return just a status ?
pub async fn get_project_checker(fetch: &State<Fetch>, claims: AccessClaims, name: &str, id: i32) -> Result<Json<Vec<PubPaper>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => helper::prepare_and_send(fetch, claims.0.user, name, id).await,
        _ => {
            println!("Error: get_project_checker; Role not handled {}", claims.0.user.role.name);
            return Err(Status::BadRequest);
        }
    }
}

#[get("/<_name>/project/<_id>", rank = 102)]
pub async fn get_project_checker_none(_name: &str, _id: i32) -> Status {
    Status::Unauthorized
}

#[post("/push", data = "<paper>", rank = 101)]
pub async fn post_project_checker(fetch: &State<Fetch>, claims: AccessClaims, paper: Json<PubPaperPush>) -> Result<Json<PubPaperPush>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => helper::send_to_checker_push(fetch, paper.into_inner()).await,
        "robot" => helper::send_to_checker_push(fetch, paper.into_inner()).await,
        _ => {
            println!("Error: post_project_checker; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[post("/<_name>", data = "<_paper>", rank = 102)]
pub async fn post_project_checker_none(_name: &str, _paper: Json<PubPaperPush>) -> Status {
    Status::Unauthorized
}
