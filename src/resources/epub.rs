use epub::doc::EpubDoc;
use unicode_segmentation::UnicodeSegmentation;
use crate::models::keywords::KeywordsModel;

use crate::messages::{Book, Chapter, Line, Paragraph};

pub fn read(path: &str) -> Result<Book, Box<dyn std::error::Error>> {
    let mut doc = EpubDoc::new(path)?;

    let title = doc.mdata("title").map(|m| m.value.clone()).unwrap_or_default();
    let author = doc.mdata("creator").map(|m| m.value.clone()).unwrap_or_default();
    let description = doc.mdata("description").map(|m| m.value.clone()).unwrap_or_default();
    let keywords = KeywordsModel::apply(&[&title, &author, &description]);

    let mut chapters = Vec::new();

    loop {
        let mime = doc.get_current_mime().unwrap_or_default();
        if mime.contains("html") {
            if let Some((content, _)) = doc.get_current_str() {
                if let Some(chapter) = parse_html_chapter(&content) {
                    chapters.push(chapter);
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

fn parse_html_chapter(html: &str) -> Option<Chapter> {
    let text = html2text::from_read(html.as_bytes(), usize::MAX);

    let mut blocks = text
        .split("\n\n")
        .map(|b| b.trim())
        .filter(|b| !b.is_empty());

    // Treat the first block as the chapter title if it's a single short line
    let first = blocks.next()?;
    let (chapter_title, rest_start) = if !first.contains('\n') && first.len() <= 120 {
        (first.to_string(), None)
    } else {
        (String::new(), Some(first))
    };

    let paragraphs: Vec<Paragraph> = rest_start
        .into_iter()
        .chain(blocks)
        .map(|block| {
            let lines = block
                .unicode_sentences()
                .map(|s| Line { text: s.to_string() })
                .collect();
            Paragraph { lines }
        })
        .filter(|p| !p.lines.is_empty())
        .collect();

    if paragraphs.is_empty() {
        return None;
    }

    Some(Chapter {
        title: chapter_title,
        paragraphs,
    })
}
