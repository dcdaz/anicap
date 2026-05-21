use crate::controllers::QueryParam;
use crate::models::{NewSerie, Serie};
use crate::schema::serie;
use crate::schema::serie::dsl::*;
use crate::utils::{DBType, ServerError, SqlConnection};
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods, delete, insert_into, update
};

type BoxablePredicate = Box<dyn BoxableExpression<serie::table, DBType, SqlType = Bool>>;

pub fn add_new_serie(
    connection: &mut SqlConnection,
    new_serie: NewSerie,
) -> Result<usize, ServerError> {
    insert_into(serie)
        .values(&new_serie)
        .execute(connection)
        .map_err(|error| ServerError::InsertFailure(error.to_string()))
}

fn build_filter(
    logged_user_id: i32,
    query_param: QueryParam,
) -> BoxablePredicate {
    let mut filter: BoxablePredicate = Box::new(user_id.eq(logged_user_id));

    if query_param.name.is_some() {
        filter = Box::new(filter.and(name.like(format!("%{}%", query_param.name.unwrap()))));
    }

    if query_param.season.is_some() {
        filter = Box::new(filter.and(season.eq(query_param.season.unwrap())))
    }

    if query_param.chapter.is_some() {
        filter = Box::new(filter.and(chapter.eq(query_param.chapter.unwrap())))
    }

    if query_param.score.is_some() {
        filter = Box::new(filter.and(score.eq(query_param.score.unwrap())))
    }

    if query_param.favorite.is_some() {
        filter = Box::new(filter.and(favorite.eq(query_param.favorite.unwrap())))
    }

    if query_param.wish_to_see.is_some() {
        filter = Box::new(filter.and(wish_to_see.eq(query_param.wish_to_see.unwrap())))
    }

    if query_param.watch_status.is_some() {
        filter = Box::new(filter.and(watch_status.eq(query_param.watch_status.unwrap())))
    }

    filter
}

pub fn search_series(
    connection: &mut SqlConnection,
    logged_user_id: i32,
    query_param: QueryParam,
) -> Result<Vec<Serie>, ServerError> {
    serie
        .filter(build_filter(logged_user_id, query_param))
        .select(Serie::as_select())
        .load::<Serie>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn get_serie_by_id(
    connection: &mut SqlConnection,
    logged_user_id: i32,
    serie_id: i32,
) -> Result<Serie, ServerError> {
    serie
        .filter(user_id.eq(logged_user_id).and(id.eq(serie_id)))
        .select(Serie::as_select())
        .first::<Serie>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn update_serie(
    connection: &mut SqlConnection,
    serie_id: i32,
    updated_serie: NewSerie,
) -> Result<usize, ServerError> {
    update(serie.filter(id.eq(serie_id)))
        .set(updated_serie)
        .execute(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn delete_serie(
    connection: &mut SqlConnection,
    serie_id: i32,
) -> Result<usize, ServerError> {
    delete(serie.filter(id.eq(serie_id)))
        .execute(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}
