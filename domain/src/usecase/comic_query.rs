use anyhow::Error;

use crate::model::comic::{Comic, ComicID};
use crate::port::primary::query::ComicQueryPort;
use crate::port::secondary::comics::ComicRepository;

pub struct ComicQueryUseCase {
    pub comic_repository: Box<dyn ComicRepository>,
}

impl ComicQueryPort for ComicQueryUseCase {
    // Business logic goes here

    fn find(&self, id: ComicID) -> Result<Option<Comic>, Error> {
        self.comic_repository.find(id)
    }
}

impl ComicQueryUseCase {
    pub fn new(comic_repository: Box<dyn ComicRepository>) -> ComicQueryUseCase {
        ComicQueryUseCase { comic_repository }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::secondary::comics::MockComicRepository;
    use mockall::predicate::*;

    #[test]
    fn find_must_call_repository_port() {
        let mut comic_query_port = MockComicRepository::new();

        comic_query_port
            .expect_find()
            .with(eq("1234".to_string()))
            .times(1)
            .returning(|_u| {
                Ok(Some(Comic {
                    upc: "1234".to_string(),
                    title: "title".to_string(),
                    description: "description".to_string(),
                }))
            });

        let comic_query_usecase = ComicQueryUseCase::new(Box::new(comic_query_port));

        let result = comic_query_usecase.find("1234".to_string());

        assert_eq!(
            result.unwrap(),
            Some(Comic {
                upc: "1234".to_string(),
                title: "title".to_string(),
                description: "description".to_string(),
            })
        );
    }
}
