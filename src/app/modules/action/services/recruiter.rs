use rocket::{State, http::Status};

use crate::app::providers::models::paper::PubPaperPush;
use crate::app::providers::services::fetch::Fetch;
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::project::PubProjectWithRecords;
use crate::app::providers::models::record::{PubRecord, PubNewRecord};

pub async fn execute(fetch: &State<Fetch>, project_id: i32) -> Result<Vec<PubPaperPush>, &'static str> {
    // type Error = &'static str;

    // let users_record = get_users_record_via_project(fetch, project_id).await?;
    let users_record = get_users_record_via_user(fetch, project_id).await?;
    let users_paper  = get_users_paper(fetch, project_id).await?;

    // Mix them
    let users_record = users_record.into_iter().map(|record| {
        let user_paper = users_paper.iter().find(|paper| paper.user_id == record.user_id);

        match user_paper {
            Some(paper) => {
                PubPaperPush {
                    id: paper.id,
                    user_id: record.user_id,
                    user_record: record.record.unwrap(), //secure unwrap
                    project_id,
                    resource_id: paper.resource_id,
                    completed: paper.completed,
                    answers: paper.answers.clone(),
                }
            },
            None => {
                PubPaperPush {
                    id: 0,
                    user_id: record.user_id,
                    user_record: record.record.unwrap(),  //secure unwrap
                    project_id,
                    resource_id: 0,
                    completed: false,
                    answers: None,
                }
            },
        }
    }).collect::<Vec<PubPaperPush>>();
    
    Ok(users_record)
}

async fn get_users_paper(fetch: &State<Fetch>, project_id: i32) -> Result<Vec<PubPaperPush>, &'static str> {
    // type Error = &'static str;

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err("Error getting robot token"),
    };

    let paper_url = ConfigGetter::get_entity_url("paper")
        .unwrap_or("http://localhost:8032/api/v1/paper/".to_string())
        + "project/"
        + project_id.to_string().as_str()
        + "/lasts";

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .get(paper_url)
            .bearer_auth(robot_token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;
    }

    match res  {
        Ok(res) => {
            if !res.status().is_success() {
                return Err("Error getting project lasts");
            }

            match res.json::<Vec<PubPaperPush>>().await {
                Ok(papers) => Ok(papers),
                Err(_) => return Err("Error getting project lasts; Response"),
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return Err("Error getting project lasts; Request")
        },
    }
}

async fn get_users_record_via_user(fetch: &State<Fetch>, project_id: i32) -> Result<Vec<PubNewRecord>, &'static str> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err("Error getting robot token"),
    };

    let user_url = ConfigGetter::get_entity_url("user")
        .unwrap_or("http://localhost:8002/api/v1/user/".to_string())
        + "project/"
        + project_id.to_string().as_str()
        + "/record";

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .get(user_url)
            .bearer_auth(robot_token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;
    }

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err("Error getting user last records");
            }
            
            match res.json::<Vec<PubNewRecord>>().await {
                Ok(records) => Ok(records),
                Err(_) => return Err("Error getting user last records; Response"),
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return Err("Error getting user last records; Request");
        },
    }
}
