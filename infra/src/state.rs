use crate::db::in_memory::InMemoryComicRepository;
use crate::web::routes::WebState;
use domain::usecase::comic_command::ComicCommandUseCase;
use std::sync::Arc;
use domain::usecase::comic_query::ComicQueryUseCase;

pub async fn state_factory() -> std::io::Result<WebState> {
    let comic_repository = InMemoryComicRepository::new();
    let query_comics = ComicQueryUseCase::new(Box::new(comic_repository));
    let command_comics = ComicCommandUseCase::new();
    let state = WebState {
        query_comics: Arc::new(query_comics), 
        command_comics: Arc::new(command_comics)
    };
    Ok(state)
}