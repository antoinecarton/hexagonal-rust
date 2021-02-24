use std::sync::Arc;
use domain::port::primary::command::ComicCommandPort;
use domain::port::primary::query::ComicQueryPort;

pub mod comics;
pub mod health_check;

#[derive(Clone)]
pub struct WebState {
    pub query_comics: Arc<dyn ComicQueryPort>,
    pub command_comics: Arc<dyn ComicCommandPort>,
}