use axum::response::Response;
use tracing::trace;

pub async fn mw_tracing(res: Response) -> Response {
    trace!("");
    res
}
