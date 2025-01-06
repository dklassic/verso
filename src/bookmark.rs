/// Tab state
pub struct Bookmark {
    /// Bookmark id
    id: BookmarkId,
    /// Name of bookmark
    name: String,
    /// Url of bookmark
    url: ServoUrl,
    /// Prompt
    prompt: Option<PromptDialog>,
}

impl Bookmark {
    
}

/// Bookmark manager to handle the all the bookmarks.
pub struct BookmarkManager {
    /// Bookmarks
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    /// Create a new tab manager.
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }
    pub fn count(&self) -> usize {
        self.bookmarks.len()
    }
    pub fn add_bookmark(&mut self, name: &str, url: ServoUrl) -> BookmarkId {
        let id = BookmarkId::new();
        self.bookmarks.push(Bookmark {
            id,
            name: name.to_string(),
            url,
            prompt: None,
        });
        id
    }
    pub fn remove_bookmark(&mut self, id: BookmarkId) -> Result<(), BookmarkManagerErr> {
        let index = self.bookmarks.iter().position(|b| b.id == id);
        match index {
            Some(i) => {
                self.bookmarks.remove(i);
                Ok(())
            }
            None => Err(BookmarkManagerErr::BookmarkNotFound),
        }
    }
    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}


/// Tab manager errors.
pub enum BookmarkManagerErr {
}