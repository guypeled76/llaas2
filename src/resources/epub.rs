use epub::doc::EpubDoc;
use scraper::{Html, Selector};

use crate::messages::{Book, Chapter, Line, Paragraph};

pub fn read(path: &str) -> Result<Book, Box<dyn std::error::Error>> {
    let mut doc = EpubDoc::new(path)?;

    let title = doc.mdata("title").map(|m| m.value.clone()).unwrap_or_default();
    let author = doc.mdata("creator").map(|m| m.value.clone()).unwrap_or_default();
    let description = doc.mdata("description").map(|m| m.value.clone()).unwrap_or_default();
    let keywords = doc
        .mdata("subject")
        .map(|m| {
            m.value
                .split(',')
                .map(|k| k.trim().to_string())
                .filter(|k| !k.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let p_selector = Selector::parse("p").unwrap();
    let h_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();

    let mut chapters = Vec::new();

    loop {
        let mime = doc.get_current_mime().unwrap_or_default();
        if mime.contains("html") {
            if let Some((content, _)) = doc.get_current_str() {
                let document = Html::parse_document(&content);

                let chapter_title = document
                    .select(&h_selector)
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .unwrap_or_default();

                let paragraphs: Vec<Paragraph> = document
                    .select(&p_selector)
                    .filter_map(|el| {
                        let text = el.text().collect::<String>();
                        let text = text.trim().to_string();
                        if text.is_empty() {
                            return None;
                        }
                        let lines = vec![Line { text: text.clone() }];
                        Some(Paragraph { text, lines })
                    })
                    .collect();

                if !paragraphs.is_empty() {
                    chapters.push(Chapter {
                        title: chapter_title,
                        paragraphs,
                    });
                }
            }
        }

        if !doc.go_next() {
            break;
        }
    }

    Ok(Book {
        title,
        author,
        description,
        keywords,
        chapters,
    })
}
