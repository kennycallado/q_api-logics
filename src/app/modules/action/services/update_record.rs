use rocket::State;
use rocket::http::Status;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::PubNewPaper;
use crate::app::providers::models::project::PubProject;
use crate::app::providers::models::record::{PubNewRecord, PubRecord};
use crate::app::providers::services::fetch::Fetch;

pub async fn execute(fetch: &State<Fetch>, project_id: i32, new_record: PubNewRecord)
-> Result<PubRecord, Status> {
    // should send ref new_record
    let project_record = match PubProject::store_record(fetch, project_id, new_record).await {
        Ok(record) => record,
        Err(status) => return Err(status),
    };

    match user_store_record(fetch, &project_record).await {
        Ok(_) => (),
        Err(status) => return Err(status),
    }

    Ok(project_record)
}

async fn user_store_record(fetch: &State<Fetch>, new_record: &PubRecord) -> Result<Status, Status> {
    let user_url = ConfigGetter::get_entity_url("user")
        .unwrap_or("http://localhost:8002/api/v1/user/".to_string())
        + "record";

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let res;
    {
        let client = fetch.client.lock().await;
        res = client.patch(&user_url)
            .bearer_auth(robot_token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&new_record)
            .send()
            .await;
    }

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            } 

            Ok(Status::Ok)
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
