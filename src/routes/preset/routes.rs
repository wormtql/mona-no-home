use mona::character::CharacterName;
use rocket::serde::json::Json;
use serde::{Deserialize};
use diesel::prelude::*;
use crate::common::response_wrapper::ResponseWrapper;
use crate::db_pool::DBConn;
use crate::guards::admin_user::AdminUserGuard;
use crate::models::preset::{NewPreset, Preset};

#[derive(Deserialize)]
pub struct DebugCreatePresetRequest {
    name: String,
    config_json: String,
    note: Option<String>,
    dsl: bool,
    genre: CharacterName,
    image: Option<String>
}

#[post("/preset_debug/create", data = "<data>")]
pub async fn debug_route_create_preset(db: DBConn, data: Json<DebugCreatePresetRequest>) -> ResponseWrapper<Preset> {
    let result = db.run(move |c| {
        let genre = data.genre.to_string();
        let new_preset = NewPreset {
            name: data.name.as_str(),
            config_json: data.config_json.as_str(),
            note: data.note.as_ref().map(|x| x.as_str()),
            is_dsl: data.dsl,
            genre: &genre,
            image: data.image.as_ref().map(|x| x.as_str())
        };

        use crate::schema::preset;

        diesel::insert_into(preset::table).values(&new_preset)
            .get_result(c)
    }).await;

    match result {
        Err(e) => ResponseWrapper::from_error(&e),
        Ok(v) => ResponseWrapper::ok(v)
    }
}

// #[post("/preset/create", data = "<data>")]
// pub async fn route_create_preset(db: DBConn, admin: AdminUserGuard<'_>, data: Json<DebugCreatePresetRequest>) -> ResponseWrapper<Preset> {
//
// }

#[get("/preset/all")]
pub async fn route_get_all_presets(db: DBConn) -> ResponseWrapper<Vec<Preset>> {
    let result: Vec<Preset> = match db.run(|c| {
        use crate::schema::preset::dsl as p;

        p::preset.load::<Preset>(c)
    }).await {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    ResponseWrapper::ok(result)
}