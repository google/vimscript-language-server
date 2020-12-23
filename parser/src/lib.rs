pub mod syntax_kind;

use crate::syntax_kind::SyntaxKind;

pub trait TokenSource {
    fn current(&self) -> SyntaxKind;
    fn bump(&mut self);
}

pub trait TreeSink {
    fn token(&mut self, kind: SyntaxKind);
    fn start_node(&mut self, kind: SyntaxKind);
    fn finish_node(&mut self, kind: SyntaxKind);
}

fn parse(source: &mut impl TokenSource, sink: &mut impl TreeSink) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
