use rocket::State;
use rocket::http::Status;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::PubNewPaper;
use crate::app::providers::services::fetch::Fetch;

pub async fn execute(fetch: &State<Fetch>, user_id: i32) -> Result<Status, Status> {
    let user_url = ConfigGetter ::get_entity_url("user")
        .unwrap_or("http://localhost:8002/api/v1/user/".to_string())
        + &user_id.to_string()
        + "/project/toggle";

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .get(user_url)
            .bearer_auth(robot_token)
            .header("Accetp", "application/json")
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
