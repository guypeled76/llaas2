pub struct Book {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub chapters: Vec<Chapter>,
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub paragraphs: Vec<Paragraph>,
}

pub struct Chapter {
    pub title: String,
    pub paragraphs: Vec<Paragraph>,
}

pub struct Paragraph {
    pub lines: Vec<Line>,
}

pub struct Line {
    pub text: String,
}
