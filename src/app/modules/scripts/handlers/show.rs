use rocket::Data;
use rocket::data::{ToByteUnit, ByteUnit};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::content::RawText;

pub async fn read(name: &str, id: i32) -> Result<rocket::tokio::fs::File, Status> {
    let mut path = std::path::Path::new("SCRIPTS/").join(&id.to_string());

    if name != "push" {
        path = path.join("cron");
    }

    let name = format!("{}.js", name);
    path = path.join(&name);

    match rocket::tokio::fs::File::open(path).await {
        Ok(file) => Ok(file),
        Err(_) => Err(Status::NotFound),
    }
}
