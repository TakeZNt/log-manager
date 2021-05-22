use actix_web::{HttpResponse, Json, Query, State};
use failure::Error;
use log::debug;

use crate::Server;

/// POST /csvのハンドラ
pub fn handle_post_csv(_server: State<Server>) -> Result<HttpResponse, Error> {
    // POSTされたCSVデータは現時点では無視する

    let logs: usize = Default::default();
    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

/// GET /csvのハンドラ
pub fn handle_get_csv(
    _server: State<Server>,
    range: Query<api::logs::get::Query>, // GETのクエリパラメータはこのように取得できる
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    let logs: Vec<api::Log> = Default::default();
    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// POST /logsのハンドラ
pub fn handle_post_logs(
    _server: State<Server>,
    log: Json<api::logs::post::Request>, // POSTのボディのJsonはこのように取得できる
) -> Result<HttpResponse, Error> {
    debug!("{:?}", log);

    Ok(HttpResponse::Accepted().finish())
}

/// GET /logsのハンドラ
pub fn handle_get_logs(
    _server: State<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    // CSVはバイナリで返す
    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
