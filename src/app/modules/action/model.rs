use rocket::State;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::paper::{PubNewPaper, PubPaperPush};
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::action::services;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Action {
    pub action: String,
    pub params: Vec<i32>
}

impl Action {
    pub async fn execute_action(&self, fetch: &State<Fetch>, push_paper: &PubPaperPush) -> Result<Status, Status> {
        match self.action.as_str() {
            "add_resource" => {
                for param in self.params.iter() {
                    let new_paper = PubNewPaper {
                        user_id: push_paper.user_id,
                        project_id: push_paper.project_id,
                        resource_id: *param,
                        completed: false,
                    };

                    services::add_resource::execute(fetch, &new_paper).await?; 
                }
            },
            "resource_completed" => {
                for param in self.params.iter() {
                    services::resource_completed::execute(fetch, param).await?;
                }
            }
            _ => {}
        }

        Ok(Status::Ok)
    }
}
