use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::{PubPaperPush, PubPaper};
use crate::app::providers::models::record::PubNewRecord;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::action::model::Action;
use crate::app::modules::action::services::{update_record, recruiter};
use crate::app::modules::checker::model::PaperPushWithAction;

pub async fn prepare_and_send(fetch: &State<Fetch>, _user: UserInClaims, name: &str, id: i32)
-> Result<Json<Vec<PubPaper>>, Status> {
    if name == "push" {
        return Err(Status::BadRequest);
    }

    let paper_push = match recruiter::execute(fetch, id).await {
        Ok(paper_push) => paper_push,
        Err(status) => {
            println!("Error: prepare_and_send; {}", status);
            return Err(Status::InternalServerError);
        }, 
    };

    // TODO: no longer true!
    // Afeter send it, I have to update the papers
    // because on push it's done auto but
    // from cron it isn't

    match send_to_checker_cron(fetch, name, id, paper_push).await {
        Ok(papers) => Ok(Json(papers)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn send_to_checker_cron(fetch: &State<Fetch>, name: &str, id: i32, papers: Vec<PubPaperPush>)
-> Result<Vec<PubPaper>, &'static str> {
    // type Error = &'static str;

    let checker_url = ConfigGetter::get_entity_url("checker")
        .unwrap_or("http://localhost:3000/api/v1/checker/".to_string())
        + name
        + "/project/"
        + id.to_string().as_str();

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .post(checker_url)
            .header("Content-Type", "application/json")
            .json(&papers)
            .send().await;
    }

    match res  {
        Ok(res) => {
            if !res.status().is_success() {
                return Err("Error getting project lasts");
            }

            // If just to return a status
            // all the code below can be removed
            // but actions
            match res.json::<Vec<PaperPushWithAction>>().await {
                Ok(papers) => {
                    // Execute actions
                    let mut response_papers: Vec<PubPaper> = Vec::new();
                    for paper in papers.iter() {
                        let actions = paper.actions.clone();
                        match actions {
                            Some(actions) => {
                                let paper_with = paper.clone().into();

                                for action in actions {
                                    match action.execute_action(fetch, &paper_with).await {
                                        Ok(_) => (),
                                        Err(s) => {
                                            println!("Error executing action: {}", action.action);
                                            println!("Error: {}", s);
                                        },
                                    }
                                }
                            },
                            None => {},
                        }

                        // response_papers.push(PubPaperPush {
                        //     id: paper.id,
                        //     user_id: paper.user_id,
                        //     user_record: paper.user_record.clone(),
                        //     project_id: paper.project_id,
                        //     resource_id: paper.resource_id,
                        //     answers: paper.answers.clone(),
                        //     completed: paper.completed,
                        // })
                        //
                        response_papers.push(PubPaper {
                            id: paper.id,
                            project_id: paper.project_id,
                            resource_id: paper.resource_id,
                            completed: paper.completed,
                        })
                    }

                    Ok(response_papers)
                },
                Err(_) => return Err("Error getting project lasts; Response"),
            }
        },
        Err(_) => return Err("Error getting project lasts; Request"),
    }
}

pub async fn send_to_checker_push(fetch: &State<Fetch>, paper: PubPaperPush) -> Result<Json<PubPaperPush>, Status> {
    let checker_url = ConfigGetter::get_entity_url("checker")
        .unwrap_or("http://localhost:3000/api/v1/checker/".to_string())
        + "push/project/"
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
