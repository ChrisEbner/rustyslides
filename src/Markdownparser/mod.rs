//! The parser takes a subset of markdown and returns an AST
//! This enables the generic translation from md to html or other output formats

//! The parser is getting implemented to follow the CommonMark reference spec
//! as of version 0.29, available at https://spec.commonmark.org/0.29


/// Line Ending Consts
const NEWLINE: char = '\u{000a}';
const CARRIAGE_RETURN: char = '\u{000d}';

/// Space
const SPACE: char = '\u{0020}';
/// Nonbreaking Space
const NBSP: char = '\u{00a0}';
/// Tab
const TAB: char = '\u{0009}';
/// Line Tabulation
const LINE_TABULATION: char = '\u{000b}';
/// Form Feed
const FORM_FEED: char = '\u{000c}';

/// Ogham Space Mark
const OGHAM_SPACE_MARK: char = '\u{1680}';
/// En Quad
const EN_QUAD: char = '\u{2000}';
/// En Space
const EN_SPACE: char = '\u{2002}';
/// Em Quad
const EM_QUAD: char = '\u{2001}';
/// Em SPACE
const EM_SPACE: char = '\u{2003}';

const REPLACE_ME_CHAR: char = '\u{0000}';
const REPLACEMENT_CHAR: char = '\u{fffd}';

// Todo: implement all Zs https://www.compart.com/en/unicode/category/Zs


// Thematic Breaks consist of lines that only contain 3 of the following:
const HYPHEN_MINUS: char = '\u{002d}';
const ASTERISK: char = '\u{002a}';
const LOW_LINE: char = '\u{005f}';

/// These are line endings
enum LineEndingChars {
    NEWLINE,
    CARRIAGE_RETURN
}

/// These are all recognized Unicode Whitespace Chars
enum UnicodeWhitespaceCharacters {
    NewLine,
    FormFeed,
    CarriageReturn,
    OghamSpaceMark,
    EnQuad,
    EnSpace,
    EmQuad,
    EmSpace
}

impl UnicodeWhitespaceCharacters {
    fn value(&self) -> char {
        match *self {
           UnicodeWhitespaceCharacters::NewLine => NEWLINE,
           UnicodeWhitespaceCharacters::FormFeed => FORM_FEED,
           UnicodeWhitespaceCharacters::CarriageReturn => CARRIAGE_RETURN,
           UnicodeWhitespaceCharacters::OghamSpaceMark => OGHAM_SPACE_MARK,
           UnicodeWhitespaceCharacters::EnQuad => EN_QUAD,
           UnicodeWhitespaceCharacters::EnSpace => EN_SPACE,
           UnicodeWhitespaceCharacters::EmQuad => EM_QUAD,
           UnicodeWhitespaceCharacters::EmSpace => EM_SPACE
        }
    }
}


// A document is a sequence of blocks
enum BlockType {
    Paragraph,
    Blockquotation,
    List,
    Heading,
    Rule,
    Codeblock
}

pub struct Document {
    pub blocks: Vec<Block>
}

pub struct Block {
    block_type: BlockType
}

pub struct AST {
    nodes: Vec<ASTBlock>
}

#[derive(Debug)]
pub enum ASTBlockType {
    Paragraph {
        data: String
    },
    Headline {
        headline_type: u8,
        data: String
    },
    List {
        list_type: ListBlockType,
        data: String 
    }
}

#[derive(Debug)]
enum ListBlockType {
    Unordered,
    Ordered
}

#[derive(Debug)]
pub struct ASTBlock {
    pub block_type: ASTBlockType,
}

pub struct Parser {
    document: Vec<Block>,
    ast: Vec<ASTBlock>
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            document: Vec::new(),
            ast: Vec::new()
        }
    }
    /// this function takes an input string and parses it to blocks and store it in the AST
    pub fn parse_to_blocks(mut self, input_string: &str) -> Vec<ASTBlock> {
        let workable_string = input_string;
        let char_vec: Vec<char> = Vec::new();
        let mut last_newposition = 0;
        let mut done: bool = false;

        while !done {
            println!("{:?}", last_newposition);
            // Break into block
            // take the string, find Lineendingchar, take string, transform into block, push block onto ast
            let mut _workable = workable_string.clone();
            let newline_position = find_newline(_workable[last_newposition.._workable.len()].chars());
            let mut char_vec_clone = char_vec.clone();
            
            match newline_position {
                Some(pos) => {
                  
                    for c in _workable.chars().take(pos) {
                        char_vec_clone.push(c)
                    }

                    // make it into block
                    self.ast.push(self.construct_block_from_char_vec(char_vec_clone));
                    // take rest
                    // move into workable string
                    last_newposition += pos;
                },
                None => {
                    // take everything from workablestring,
                    for c in _workable.clone().chars() {
                        char_vec_clone.push(c)
                    }
                    // make it into block,
                    self.ast.push(self.construct_block_from_char_vec(char_vec_clone));

                    //then set done to true.
                    done = true;
                }
            }
        }
        
        self.ast
    }

    fn construct_block_from_char_vec(&self, _string: Vec<char>) -> ASTBlock {
        // check chars for whitespace and block demarcation chars
        for xc in _string
        {
            println!("Input string {:?}", xc);
        }
        
        ASTBlock {
            ///return the ASTBlock
            block_type: ASTBlockType::Paragraph {
                data: "WHAT".to_string()
            }
        }
    }
}

fn find_newline(mut chariter: std::str::Chars<'_>) -> Option<usize> {
    return chariter.position(|c| {
        c == NEWLINE
    });
}