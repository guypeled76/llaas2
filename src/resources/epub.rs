use crate::models::keywords::KeywordsModel;
use epub::doc::EpubDoc;
use unicode_segmentation::UnicodeSegmentation;

/// A module to handle reading and parsing EPUB files, extracting metadata and content into structured formats.
use crate::messages::{Book, Chapter, Line, Paragraph};

/// Reads an EPUB file from the given path and extracts its metadata and content into a Book struct. It uses the EpubDoc library to read the EPUB file and the KeywordsModel to extract keywords from the metadata.
/// # Arguments
/// * `path` - A string slice that holds the path to the EPUB file.
/// # Returns
/// * `Result<Book, Box<dyn std::error::Error>>` - A result containing the Book struct if successful, or an error if the file cannot be read or parsed.
/// # Errors
/// This function will return an error if the EPUB file cannot be read, if the metadata cannot be extracted, or if the content cannot be parsed. The error will be returned as a boxed dynamic
/// error type, allowing for flexibility in the types of errors that can be returned.
pub fn read(path: &str) -> Result<Book, Box<dyn std::error::Error>> {
    let mut doc = EpubDoc::new(path)?;

    let title = doc
        .mdata("title")
        .map(|m| m.value.clone())
        .unwrap_or_default();
    let author = doc
        .mdata("creator")
        .map(|m| m.value.clone())
        .unwrap_or_default();
    let description = doc
        .mdata("description")
        .map(|m| m.value.clone())
        .unwrap_or_default();

    print!("Extracting keywords from metadata... ");
    let keywords = KeywordsModel::apply(&[&title, &author, &description]);
    print!("Done.\n");

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

/// Parses an HTML string to extract the chapter title and paragraphs. It uses the html2text library to convert the HTML content into plain text, and then processes the text to identify the chapter title and paragraphs.
/// # Arguments
/// * `html` - A string slice that holds the HTML content of a chapter.
/// # Returns
/// * `Option<Chapter>` - An option containing the Chapter struct if parsing is successful, or None if the content cannot be parsed into a chapter. The Chapter struct contains the chapter title and
/// a vector of Paragraph structs, each containing a vector of Line structs representing the lines of text in the chapter.
/// # Errors
/// This function will return None if the HTML content cannot be parsed into a chapter, which can occur if the content is empty or does not contain valid text. The function does not return an error
/// but instead uses the Option type to indicate the success or failure of the parsing operation.
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
                .map(|s| Line {
                    text: s.to_string(),
                })
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
