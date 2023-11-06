use rocket::Data;
use rocket::data::{ToByteUnit, ByteUnit};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::content::RawText;

use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::scripts::handlers::{create, show};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        upload_file,
        upload_file_none,
        show_file,
        show_file_none,
    ]
}

#[options("/")]
pub fn options_index() -> Status {
    Status::Ok
}

#[options("/<_name>")]
pub fn options_show(_name: &str) -> Status {
    Status::Ok
}

#[post("/<name>/project/<id>", data = "<script>", rank = 101)]
pub async fn upload_file(claims: AccessClaims, name: &str, id: i32, script: Data<'_>) -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => create::save(name, id, script).await,
        _ => {
            println!("Error: upload_file; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[post("/<_name>/project/<_id>", data = "<_script>", rank = 102)]
pub async fn upload_file_none(_name: &str, _id: i32, _script: Data<'_>) -> Result<Status, Status> {
    Err(Status::Unauthorized)
}

#[get("/<name>/project/<id>", rank = 101)]
pub async fn show_file(claims: AccessClaims, name: &str, id: i32) -> Result<rocket::tokio::fs::File, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => show::read(name, id).await,
        _ => {
            println!("Error: show_file; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_name>/project/<_id>", rank = 102)]
pub async fn show_file_none(_name: &str, _id: i32) -> Result<RawText<String>, Status> {
    Err(Status::Unauthorized)
}
