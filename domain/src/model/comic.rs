//! Domain model

/// Represents the identifier of a comic book
pub type ComicID = String;
/// Represents the title of a comic book
pub type Title = String;
/// Represents the description of a comic book
pub type Description = String;

/// Represents a comic book
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comic {
    pub upc: ComicID,
    pub title: Title,
    pub description: Description,
}
