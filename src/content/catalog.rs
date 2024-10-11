use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        return Catalog {
            items: vec![]
        };
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get(&self, idx: usize) -> Option<&Media> {
        if idx < self.items.len() {
            return Some(&self.items[idx]);
        } else {
            return None;
        }
    }
}
