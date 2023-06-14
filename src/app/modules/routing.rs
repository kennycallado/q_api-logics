use crate::app::modules::checker::controller as checker_controller;
use crate::app::modules::scripts::controller as scripts_controller;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/logic/checker", checker_controller::routes())
            .mount("/api/v1/logic/script", scripts_controller::routes())
    })
}
