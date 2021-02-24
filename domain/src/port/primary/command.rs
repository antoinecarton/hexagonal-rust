use crate::model::comic::ComicID;
use anyhow::Error;

pub trait ComicCommandPort {
    fn delete(&self, id: ComicID) -> Result<(), Error>;
}
