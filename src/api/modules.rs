use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    Json,
    Path,
    Responder,
    error::ErrorInternalServerError,
    http::Method,
};
use serde::de::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{
    events::{self, EventManagerAddrExt},
    models::{
        File,
        User,
        draft::PublicData as DraftData,
        module::{Module, PublicData as ModuleData},
    },
};
use super::{
    State,
    session::{Session, ElevatedSession},
    users::UserId,
};

/// Configure routes.
pub fn routes(app: App<State>) -> App<State> {
    app
        .resource("/modules", |r| {
            r.get().with(list_modules);
            r.post().with(create_module);
        })
        .route("/modules/assigned/to/{user}", Method::GET, list_assigned)
        .resource("/modules/{id}", |r| {
            r.get().with(get_module);
            r.post().with(crete_draft);
            r.put().with(update_module);
            r.delete().f(delete_module);
        })
        .resource("/modules/{id}/comments", |r| {
            r.get().f(list_comments);
            r.post().f(add_comment);
        })
        .route("/modules/{id}/files", Method::GET, list_files)
        .route("/modules/{id}/files/{name}", Method::GET, get_file)
}

type Result<T> = std::result::Result<T, actix_web::error::Error>;

#[derive(Debug, Deserialize)]
pub struct NewModule {
    title: String,
}

/// Get list of all modules.
///
/// ## Method
///
/// ```
/// Get /modules
/// ```
pub fn list_modules((
    state,
    _session,
): (
    actix_web::State<State>,
    Session,
)) -> Result<Json<Vec<ModuleData>>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let modules = Module::all(&*db)
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(Json(modules.into_iter()
        .map(|module| module.get_public())
        .collect()))
}

/// Get list of all modules assigned to a particular user.
///
/// ## Method
///
/// ```
/// GET /modules/assigned/to/:user
/// ```
pub fn list_assigned((
    state,
    session,
    user,
): (
    actix_web::State<State>,
    Session,
    Path<UserId>,
)) -> Result<Json<Vec<ModuleData>>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let user = user.get_user(&*state, &session)?;
    let modules = Module::assigned_to(&*db, user.id)
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(Json(modules.into_iter()
        .map(|module| module.get_public())
        .collect()))
}

/// Create a new empty module.
///
/// ## Method
///
/// ```
/// POST /modules
/// ```
pub fn create_module((
    state,
    _session,
    data,
): (
    actix_web::State<State>,
    ElevatedSession,
    Json<NewModule>,
)) -> Result<Json<ModuleData>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let content = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
        <document xmlns="http://cnx.rice.edu/cnxml" cnxml-version="0.7" id="new" module-id="new">
            <title>{}</title>
            <content>
                <para/>
            </content>
        </document>
        "#,
        tera::escape_html(&data.title),
    );

    let index = File::from_data(&*db, &state.config, &content)
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let module = Module::create::<&str, _>(&*db, &data.title, index, &[])
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    Ok(Json(module.get_public()))
}

/// Get a module by ID.
///
/// ## Method
///
/// ```
/// GET /modules/:id
/// ```
pub fn get_module((
    state,
    _session,
    id,
): (
    actix_web::State<State>,
    Session,
    Path<Uuid>,
)) -> Result<Json<ModuleData>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let module = Module::by_id(&*db, id.into_inner())?;

    Ok(Json(module.get_public()))
}

/// Create a new draft of a module.
///
/// ## Method
///
/// ```
/// POST /modules/:id
/// ```
pub fn crete_draft((
    state,
    session,
    id,
): (
    actix_web::State<State>,
    Session,
    Path<Uuid>,
)) -> Result<Json<DraftData>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let module = Module::by_id(&*db, id.into_inner())?;
    let draft = module.create_draft(&*db, session.user)?;

    Ok(Json(draft.get_public()))
}

#[derive(Debug, Deserialize)]
pub struct ModuleUpdate {
    #[serde(default, deserialize_with = "de_optional_null")]
    pub assignee: Option<Option<i32>>,
}

/// Update module
///
/// ## Method
///
/// ```
/// PUT /modules/:id
/// ```
pub fn update_module((
    state,
    session,
    id,
    update,
): (
    actix_web::State<State>,
    ElevatedSession,
    Path<Uuid>,
    Json<ModuleUpdate>,
)) -> Result<HttpResponse> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let module = Module::by_id(&*db, id.into_inner())?;

    use diesel::Connection;
    let dbconn = &*db;
    dbconn.transaction::<_, failure::Error, _>(|| {
        if let Some(user) = update.assignee {
            module.set_assignee(dbconn, user)?;

            if let Some(id) = user {
                let user = User::by_id(dbconn, id)?;
                state.events.notify(user, events::Assigned {
                    who: session.user,
                    module: module.id(),
                });
            }
        }

        Ok(())
    }).map_err(|e| ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a module
///
/// ## Method
///
/// ```
/// DELTE /modules/:id
/// ```
pub fn delete_module(_req: &HttpRequest<State>) -> HttpResponse {
    unimplemented!()
}

/// Get comments on a module.
///
/// ## Method
///
/// ```
/// GET /modules/:id/comments
/// ```
pub fn list_comments(_req: &HttpRequest<State>) -> HttpResponse {
    unimplemented!()
}

/// Add a comment to a module
///
/// ## Method
///
/// ```
/// POST /modules/:id/comments
/// ```
pub fn add_comment(_req: &HttpRequest<State>) -> HttpResponse {
    unimplemented!()
}

/// List files in a module.
///
/// ## Method
///
/// ```
/// GET /modules/:id/files
/// ```
pub fn list_files((
    state,
    _session,
    id,
): (
    actix_web::State<State>,
    Session,
    Path<Uuid>,
)) -> Result<Json<Vec<String>>> {
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let module = Module::by_id(&*db, *id)?;

    module.get_files(&*db)
        .map_err(|e| ErrorInternalServerError(e.to_string()))
        .map(Json)
}

/// Get a file from a module.
///
/// ## Method
///
/// ```
/// GET /modules/:id/files/:name
/// ```
pub fn get_file((
    state,
    _session,
    path,
): (
    actix_web::State<State>,
    Session,
    Path<(Uuid, String)>,
)) -> Result<impl Responder> {
    let (id, name) = path.into_inner();
    let db = state.db.get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let module = Module::by_id(&*db, id)?;

    Ok(module.get_file(&*db, &name)?
        .stream(&state.config))
}

fn de_optional_null<'de, T, D>(de: D) -> std::result::Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    T::deserialize(de).map(Some)
}
