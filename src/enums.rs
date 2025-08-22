#[derive(Debug)]
pub enum Token {
    Symbol(char),                                   // Specific chars (# for example)
    Text(String),                                   // Longer Strings of text
}

#[derive(Debug)]
pub enum ASTNode {
    Document(Vec<ASTNode>),                         // Whole Tree
    Section(SectionDescriptor, Vec<ASTNode>),       // Section for different parts
    Heading(usize, String),                         // Level + Text
    Paragraph(String),                              // Text
    Code(String, String),                           // Language + Text
    Button(ButtonType, String),                     // Button type + Link
}

#[derive(Debug)]
pub enum SectionDescriptor {
    HEADER,
    RESULT,
    END,
}

#[derive(Debug)]
pub enum ButtonType {
    REPOSITORY,
    DOWNLOAD,
}