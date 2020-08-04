// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::CharIndices;

// It is very similar to `Peekable<CharIndicies>`, with few minor differences:
//
// - it iterates over `char`, but stores position in separate field that is accessible as separate
// call.
// - it fetches one item in advance - normal Peekable will not fetch the next element unless next
// or peek is called.
pub struct PeekableCharsWithPosition<'a> {
    chars: CharIndices<'a>,
    current_ch: Option<char>,
    current_pos: usize,
    size: usize,
}

impl<'a> PeekableCharsWithPosition<'a> {
    pub fn new(input: &'a str) -> PeekableCharsWithPosition<'a> {
        let mut chars = input.char_indices();
        let ch = chars.next().map(|(_, c)| c);
        return PeekableCharsWithPosition {
            current_ch: ch,
            current_pos: 0,
            chars: chars,
            size: input.len(),
        };
    }

    // Returns the next element without advancing the position of the iterator.
    pub fn peek(&self) -> Option<char> {
        return self.current_ch;
    }

    // Returns byte position of current element (the one that will be returned with next/peek).
    pub fn pos(&self) -> usize {
        return self.current_pos;
    }
}

impl<'a> Iterator for PeekableCharsWithPosition<'a> {
    type Item = char;

    // Returns the current element and advances the iterator.
    fn next(&mut self) -> Option<char> {
        let ret = self.current_ch;
        match self.chars.next() {
            None => {
                self.current_ch = None;
                self.current_pos = self.size;
            }
            Some((pos, c)) => {
                self.current_ch = Some(c);
                self.current_pos = pos;
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_returns_none_on_empty_input() {
        let peekable = PeekableCharsWithPosition::new("");
        assert_eq!(None, peekable.peek());
        assert_eq!(None, peekable.peek());
    }

    #[test]
    fn peek_returns_first_element_multiple_time() {
        let peekable = PeekableCharsWithPosition::new("a");
        assert_eq!(Some('a'), peekable.peek());
        assert_eq!(Some('a'), peekable.peek());
        assert_eq!(Some('a'), peekable.peek());
    }

    #[test]
    fn pos_returns_zero_on_empty_input() {
        let peekable = PeekableCharsWithPosition::new("");
        assert_eq!(0, peekable.pos());
        assert_eq!(0, peekable.pos());
    }

    #[test]
    fn next_returns_none_on_empty_input() {
        let mut peekable = PeekableCharsWithPosition::new("");
        assert_eq!(None, peekable.next());
        assert_eq!(None, peekable.next());
    }

    #[test]
    fn next_advances_iterator() {
        let mut peekable = PeekableCharsWithPosition::new("a");
        assert_eq!(Some('a'), peekable.next());

        assert_eq!(1, peekable.pos());
        assert_eq!(None, peekable.peek());
        assert_eq!(None, peekable.next());
    }
}
