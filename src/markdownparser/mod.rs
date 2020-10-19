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
    NEWLINE,
    FORM_FEED,
    CARRIAGE_RETURN,
    OGHAM_SPACE_MARK,
    EN_QUAD,
    EN_SPACE,
    EM_QUAD,
    EM_SPACE
}


// A document is a sequence of blocks
enum BlockTypes {
    paragraph,
    block_quotation,
    list,
    heading,
    rule,
    code_block
}

pub struct Parser {

}


impl Parser {
}