#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        return Catalog {
            items: vec![]
        };
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get(&self, idx: usize) -> Option<&Media> {
        if idx < self.items.len() {
            return Some(&self.items[idx]);
        } else {
            return None;
        }
    }
}

fn main() {
    let movie = Media::Movie { title: String::from("Movie"), director: String::from("Director") };
    let mut catalog = Catalog::new();
    catalog.add(movie);

    match catalog.get(0) {
        Some(media) => println!("C: {:#?}", media),
        None => println!("Not found")
    }
}
