use rocket::State;
use rocket::http::Status;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::PubNewPaper;
use crate::app::providers::services::fetch::Fetch;

pub async fn execute(fetch: &State<Fetch>, paper_id: &i32) -> Result<Status, Status> {
    let paper_url = ConfigGetter ::get_entity_url("paper")
        .unwrap_or("http://localhost:8032/api/v1/paper/".to_string())
        + paper_id.to_string().as_str()
        + "/completed";

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let res;
    {
        let client = fetch.client.lock().await;
        res = client.patch(&paper_url)
            .bearer_auth(robot_token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
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
