pub mod primitive;

#[derive(Debug)]
pub struct Character {
    value: char,
}

#[derive(Debug)]
pub enum CategoryCode {
    Escape,           // 0
    BeginningOfGroup, // 1
    EndOfGroup,       // 2
    MathShift,        // 3
    AlignmentTab,     // 4
    EndOfLine,        // 5
    Parameter,        // 6
    Superscript,      // 7
    Subscript,        // 8
    Ignored,          // 9
    Space,            // 10
    Letter,           // 11
    Other,            // 12
    Active,           // 13
    Comment,          // 14
    Invalid,          // 15
}

#[derive(Debug)]
pub struct Program<'a> {
    renderer: &'a Renderer,
    logger: &'a Logger,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn advance(&mut self, c: Character) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Renderer {}

#[derive(Debug)]
pub struct Logger {}
