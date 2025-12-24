use std::sync::{Arc, RwLock};

use axum::{extract::{Path, State}, Json};
use crate::{ApiError, defs::definition_repository::*, defs::definitions::*};


#[derive(serde::Deserialize)]
pub struct CreateArguments {
    term: String,
    meaning: String
}

#[derive(serde::Deserialize)]
pub struct UpdateArguments {
    meaning: String
}

pub async fn list_defs(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>
) -> Json<Vec<Definition>> {
    let defs_lock = def_repo.read().expect("Failed to lock repo for read");

    let def_list = defs_lock.list();

    Json(def_list)
}

pub async fn get_def_by_id(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>,
    Path(id): Path<String>) -> Result<Json<Definition>, ApiError> {
    
    let defs_lock = def_repo.read().expect("Failed to lock repo for read");

    let def = defs_lock.get_by_id(&id);

    match def {
        Some(def) => Ok(Json(def)),
        None => Err(ApiError::NotFound),
    }
}

pub async fn get_def_by_term(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>,
    Path(term): Path<String>
) -> Result<Json<Definition>, ApiError> {

    let defs_lock = def_repo.read().expect("Failed to lock repo for read");

    let def = defs_lock.get_by_term(&term);

    match def {
        Some(def) => Ok(Json(def)),
        None => Err(ApiError::NotFound),
    }
}

pub async fn remove_by_id(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>,
    Path(id): Path<String>) -> Result<Json<Definition>, ApiError> {
    let mut defs_lock = def_repo.write().expect("Failed to lock repo for read");
    
    let def = defs_lock.remove(&id);

    match def {
        Some(def) => Ok(Json(def)),
        None => Err(ApiError::NotFound),
    }
}

pub async fn create_def(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>,
    Json(create_args): Json< CreateArguments>
) -> Json<Definition> {
    let mut defs_lock= def_repo.write().expect("Failed to lock repo for read");

    let term = create_args.term;
    let useful_def = create_args.meaning;

    let def = defs_lock.create(&term, &useful_def);

    Json(def)
}

pub async fn update_def(
    State(def_repo): State<Arc<RwLock<DefinitionManager>>>,
    Path(id): Path<String>, Json(new_def):Json<UpdateArguments>) -> Result<Json<Definition>, ApiError> {
    let mut defs_lock= def_repo.write().expect("Failed to lock repo for read");
    
    let meaning = new_def.meaning;

    let def = defs_lock.update_def(&id, &meaning)
    .map_err(|_error| {ApiError::NotFound})?;

       Ok(Json(def)) 
}

