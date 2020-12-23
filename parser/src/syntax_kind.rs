#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    EQ,
    PLUS,
    // Number (any number acceptable by vim script)
    NUMBER,
    // Identifier, e.g. `l:a`
    IDENT,

    LET_KW,

    // The whole let statement.
    LET_STMT,

    // Variable to assign to (on the left side of the operator).
    LET_VAR,

    IDENT_EXPR,

    // Space or tab
    WHITESPACE,
    // We use this because in vimscript new lines are important (end of statement).
    NEW_LINE,
    EOF,
    ERROR,

    ROOT,
    __LAST,
}

impl From<u16> for SyntaxKind {
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}
