use anyhow::Error;

use crate::model::comic::ComicID;
use crate::port::primary::command::ComicCommandPort;

pub struct ComicCommandUseCase {}

impl ComicCommandPort for ComicCommandUseCase {
    // Business logic goes here

    fn delete(&self, _id: ComicID) -> Result<(), Error> {
        Ok(())
    }
}

impl ComicCommandUseCase {
    pub fn new() -> ComicCommandUseCase {
        ComicCommandUseCase {}
    }
}

impl Default for ComicCommandUseCase {
    fn default() -> Self {
        ComicCommandUseCase::new()
    }
}