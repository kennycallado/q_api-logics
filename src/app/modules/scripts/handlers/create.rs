use rocket::Data;
use rocket::data::{ToByteUnit, ByteUnit};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::content::RawText;

pub async fn save(name: String, id: i32, script: Data<'_>) -> Result<Status, Status> {
    let mut path = std::path::Path::new("SCRIPTS/").join(&id.to_string());

    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    // If the name is different from push
    // the file should be into the cron folder
    if name != "push" {
        path = path.join("cron");
        if !path.exists() {
            match std::fs::create_dir_all(&path) {
                Ok(_) => (),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
    }

    let name = format!("{}.js", name);
    path = path.join(&name);

    match script.open(1.megabytes()).into_file(path).await {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

