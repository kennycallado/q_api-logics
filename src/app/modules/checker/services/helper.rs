use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::modules::checker::model::PaperPushWithAction;

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::app::providers::interfaces::paper::PubPaperPush;

pub async fn send_to_checker(fetch: &State<Fetch>, paper: PubPaperPush) -> Result<Json<PubPaperPush>, Status> {
    let checker_url = ConfigGetter::get_entity_url("checker")
        .unwrap_or("http://localhost:3000/".to_string())
        + "project/"
        + paper.project_id.to_string().as_str()
        + "/push"; 

    let fetch = fetch.client.lock().await;
    let res = fetch
        .post(checker_url)
        .header("Content-Type", "application/json")
        .json(&paper)
        .send().await;

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            } 

            let paper_with_actions = res.json::<PaperPushWithAction>().await.unwrap();

            if paper_with_actions.actions.is_some() {
                // TODO: Execute the actions
            }

            // Return the paper
            Ok(Json(paper_with_actions.into()))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
