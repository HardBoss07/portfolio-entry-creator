pub enum Token {
    Heading(usize, String),             // Level + Text
    Paragraph(String),                  // Text
    Button(bool, String),               // isRepoOrDownload? + Link
    CodeBlock(String, String),          // Language + Text
    SectionMarker(SectionDescriptor),   // Splitter for sections
}

pub enum SectionDescriptor {
    HEADER,
    RESULT,
    END,
}

pub enum ASTNode {
    Document(Vec<ASTNode>),             // Whole Tree
    Section(Vec<ASTNode>),              // Section for different parts
    Heading(usize, String),             // Level + Text
    Paragraph(String),                  // Text
    Code(String, String),               // Language + Text
}