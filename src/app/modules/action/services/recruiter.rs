use rocket::{State, http::Status};

use crate::app::providers::models::paper::PubPaperPush;
use crate::app::providers::services::fetch::Fetch;
use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::project::PubProjectWithRecords;
use crate::app::providers::models::record::PubRecord;

pub async fn execute(fetch: &State<Fetch>, project_id: i32) -> Result<Vec<PubPaperPush>, &'static str> {
    // type Error = &'static str;

    let users_record = get_users_record(fetch, project_id).await?;
    let users_paper  = get_users_paper(fetch, project_id).await?;

    // Mix them
    let users_paper = users_paper.into_iter().map(|paper| {
        let user_record = users_record.iter().find(|record| record.user_id == paper.user_id);

        match user_record {
            Some(record) => {
                let mut paper = paper.clone();
                paper.user_record = record.record.clone();
                paper
            },
            None => paper,
        }
    }).collect::<Vec<PubPaperPush>>();

    Ok(users_paper)
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
        Err(_) => return Err("Error getting project lasts; Request"),
    }
}

async fn get_users_record(fetch: &State<Fetch>, project_id: i32) -> Result<Vec<PubRecord>, &'static str> {
    // type Error = &'static str;

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err("Error getting robot token"),
    };

    let project_url = ConfigGetter::get_entity_url("project")
        .unwrap_or("http://localhost:8051/api/v1/project/".to_string())
        + project_id.to_string().as_str()
        + "/record/lasts";

    let res;
    {
        let client = fetch.client.lock().await;
        res = client
            .get(project_url)
            .bearer_auth(&robot_token)
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

            match res.json::<PubProjectWithRecords>().await {
                Ok(project_records) => Ok(project_records.records.unwrap_or(vec![])),
                Err(_) => return Err("Error getting project lasts; Response"),
            }
        },
        Err(_) => return Err("Error getting project lasts; Request"),
    }
}
