use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::PubPaperPush;
use crate::app::providers::models::record::PubNewRecord;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::action::model::Action;
use crate::app::modules::action::services::update_record;
use crate::app::modules::checker::model::PaperPushWithAction;

pub async fn send_to_checker(fetch: &State<Fetch>, name: &str, paper: PubPaperPush) -> Result<Json<PubPaperPush>, Status> {
    let checker_url = ConfigGetter::get_entity_url("checker")
        .unwrap_or("http://localhost:3000/checker/".to_string())
        + name 
        + "/project/"
        + paper.project_id.to_string().as_str();

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .post(checker_url)
            .header("Content-Type", "application/json")
            .json(&paper)
            .send().await;
    }

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            } 

            let paper_with_actions = res.json::<PaperPushWithAction>().await.unwrap();

            let paper_w_clone = paper_with_actions.clone();
            match paper_w_clone.actions.clone() {
                Some(actions) => {
                    println!("Actions: {:?}", actions);
                    let paper_with = paper_w_clone.clone().into();

                    for action in actions {
                        match action.execute_action(fetch, &paper_with).await {
                            Ok(_) => (),
                            Err(s) => {
                                println!("Error executing action: {}", action.action);
                                println!("Error: {}", s);
                            },
                        }
                    }

                    // paper_with.into()
                },
                // None => blah.into(),
                None => {}
            }

            let new_record = PubNewRecord {
                user_id: paper_with_actions.user_id,
                record: paper_with_actions.user_record.clone(),
            };

            match update_record::execute(fetch, paper_with_actions.project_id, new_record).await {
                Ok(_) => (),
                Err(status) => return Err(status),
            }

            // Ok(Json(paper_push))
            Ok(Json(paper_with_actions.into()))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
