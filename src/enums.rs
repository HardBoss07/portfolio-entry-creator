pub enum Token {
    Symbol(char),                                   // Specific chars (# for example)
    Text(String),                                   // Longer Strings of text
}

pub enum ASTNode {
    Document(Vec<ASTNode>),                         // Whole Tree
    Section(SectionDescriptor, Vec<ASTNode>),       // Section for different parts
    Heading(usize, String),                         // Level + Text
    Paragraph(String),                              // Text
    Code(String, String),                           // Language + Text
    Button(ButtonType, String),                     // Button type + Link
}

pub enum SectionDescriptor {
    HEADER,
    RESULT,
    END,
}

pub enum ButtonType {
    REPOSITORY,
    DOWNLOAD,
}