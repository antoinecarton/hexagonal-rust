use anyhow::Error;
use domain::model::comic::{Comic, ComicID};
use domain::port::secondary::comics::ComicRepository;

pub struct InMemoryComicRepository {}

impl ComicRepository for InMemoryComicRepository {
    fn find(&self, upc: ComicID) -> Result<Option<Comic>, Error> {
        Ok(self.all_comics()
            .into_iter()
            .find(|comic| comic.upc == upc)
        )
    }
}

impl InMemoryComicRepository {
    pub fn new() -> InMemoryComicRepository {
        InMemoryComicRepository {}
    }

    fn all_comics(&self) -> Vec<Comic> {
        vec![
            Comic {
                upc: "1234".to_string(),
                title: "batman#1".to_string(),
                description: "joker war #1".to_string(),
            },
            Comic {
                upc: "5678".to_string(),
                title: "batman#2".to_string(),
                description: "joker war #2".to_string(),
            },
        ]
    }
}

impl Default for InMemoryComicRepository {
    fn default() -> Self {
        InMemoryComicRepository::new()
    }
}