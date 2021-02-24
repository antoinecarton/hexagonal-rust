use crate::model::comic::{Comic, ComicID};
use anyhow::Error;
use mockall::*;

#[automock]
pub trait ComicRepository {
    fn find(&self, upc: ComicID) -> Result<Option<Comic>, Error>;
}