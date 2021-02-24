use anyhow::Error;

use crate::model::comic::{Comic, ComicID};

pub trait ComicQueryPort {
    fn find(&self, id: ComicID) -> Result<Option<Comic>, Error>;
}
