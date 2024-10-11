mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let movie = Media::Movie { title: String::from("Movie"), director: String::from("Director") };
    let mut catalog = Catalog::new();
    catalog.add(movie);

    match catalog.get(0) {
        Some(media) => println!("C: {:#?}", media),
        None => println!("Not found")
    }
}
