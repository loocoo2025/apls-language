//! Deterministic APLS 0.1 lexer.

#![allow(dead_code)]

use crate::limits::MAX_TOKENS;

macro_rules! define_keywords {
    ($($variant:ident => $text:literal),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) enum Keyword {
            $($variant),+
        }

        impl Keyword {
            fn from_identifier(text: &str) -> Option<Self> {
                match text {
                    $($text => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }

        #[cfg(test)]
        const ALL_KEYWORDS: &[(&str, Keyword)] = &[
            $(($text, Keyword::$variant),)+
        ];
    };
}

define_keywords! {
    Acceptance => "acceptance",
    Action => "action",
    Actor => "actor",
    And => "and",
    Apls => "apls",
    Approved => "approved",
    Architecture => "architecture",
    As => "as",
    Assert => "assert",
    AtLeastOnce => "at_least_once",
    AtMostOnce => "at_most_once",
    BareMetal => "bare_metal",
    BaseUnit => "base_unit",
    Bool => "bool",
    Bus => "bus",
    Call => "call",
    Capacity => "capacity",
    Channel => "channel",
    Class => "class",
    Component => "component",
    Constraint => "constraint",
    Cooperative => "cooperative",
    Coroutine => "coroutine",
    Custom => "custom",
    Decimal => "decimal",
    Decision => "decision",
    Delivery => "delivery",
    Dimension => "dimension",
    Domain => "domain",
    Emit => "emit",
    Enum => "enum",
    Event => "event",
    EventDriven => "event_driven",
    EventLoop => "event_loop",
    ExactlyOnce => "exactly_once",
    Execution => "execution",
    Expect => "expect",
    False => "false",
    Fifo => "fifo",
    From => "from",
    Frozen => "frozen",
    Given => "given",
    Guard => "guard",
    HoldFor => "hold_for",
    Import => "import",
    InProcess => "in_process",
    Initial => "initial",
    Int => "int",
    Intent => "intent",
    Interrupt => "interrupt",
    Invoke => "invoke",
    Ipc => "ipc",
    Kind => "kind",
    List => "list",
    MainLoop => "main_loop",
    ManagedRuntime => "managed_runtime",
    Message => "message",
    Mode => "mode",
    Model => "model",
    Network => "network",
    Not => "not",
    Offset => "offset",
    On => "on",
    Open => "open",
    Operation => "operation",
    Optional => "optional",
    Or => "or",
    Ordering => "ordering",
    Os => "os",
    Owner => "owner",
    Payload => "payload",
    Performance => "performance",
    Periodic => "periodic",
    Preemptive => "preemptive",
    Priority => "priority",
    Process => "process",
    Prohibit => "prohibit",
    Pure => "pure",
    Quantity => "quantity",
    Question => "question",
    Queue => "queue",
    Rationale => "rationale",
    Record => "record",
    Resource => "resource",
    Responsibility => "responsibility",
    Returns => "returns",
    Rpc => "rpc",
    Rtos => "rtos",
    Rule => "rule",
    Safety => "safety",
    Scale => "scale",
    Schedule => "schedule",
    Set => "set",
    Source => "source",
    Spec => "spec",
    State => "state",
    StateChange => "state_change",
    StateMachine => "state_machine",
    Statement => "statement",
    Status => "status",
    Stream => "stream",
    String => "string",
    Supports => "supports",
    Symbol => "symbol",
    Task => "task",
    Terminal => "terminal",
    Then => "then",
    Thread => "thread",
    Timeout => "timeout",
    Timer => "timer",
    To => "to",
    Transition => "transition",
    Transport => "transport",
    Trigger => "trigger",
    True => "true",
    Type => "type",
    Unit => "unit",
    Unknown => "unknown",
    Unordered => "unordered",
    Version => "version",
    When => "when",
    Within => "within",
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TokenKind {
    Keyword(Keyword),
    Identifier,
    Integer,
    Decimal,
    String,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    At,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    EqualEqual,
    BangEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ByteSpan {
    pub start: usize,
    pub end: usize,
}

impl ByteSpan {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LexErrorCode {
    InvalidCharacter,
    InvalidStringOrComment,
    InvalidNumber,
    TokenLimitExceeded,
}

impl LexErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidCharacter => "APLS-E1001",
            Self::InvalidStringOrComment => "APLS-E1002",
            Self::InvalidNumber => "APLS-E1003",
            Self::TokenLimitExceeded => "APLS-T0007",
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::InvalidCharacter => "invalid source byte or character",
            Self::InvalidStringOrComment => "invalid or unterminated string or block comment",
            Self::InvalidNumber => "invalid numeric literal",
            Self::TokenLimitExceeded => "compiler token limit exceeded",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LexFinding {
    pub code: LexErrorCode,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceIndex {
    line_starts: Vec<usize>,
}

impl SourceIndex {
    pub(crate) fn new(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut line_starts = vec![0];
        let mut offset = 0;

        while offset < bytes.len() {
            match bytes[offset] {
                b'\r' if bytes.get(offset + 1) == Some(&b'\n') => {
                    offset += 2;
                    line_starts.push(offset);
                }
                b'\r' | b'\n' => {
                    offset += 1;
                    line_starts.push(offset);
                }
                byte if byte.is_ascii() => offset += 1,
                _ => {
                    let width = source[offset..]
                        .chars()
                        .next()
                        .expect("offset is inside validated UTF-8")
                        .len_utf8();
                    offset += width;
                }
            }
        }

        Self { line_starts }
    }

    pub(crate) fn location(&self, source: &str, byte_offset: usize) -> Option<SourceLocation> {
        if byte_offset > source.len() || !source.is_char_boundary(byte_offset) {
            return None;
        }

        let next_line = self
            .line_starts
            .partition_point(|start| *start <= byte_offset);
        let line_index = next_line.saturating_sub(1);
        let line_start = self.line_starts[line_index];
        let column = source[line_start..byte_offset].chars().count() + 1;

        Some(SourceLocation {
            line: line_index + 1,
            column,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LexedSource<'source> {
    source: &'source str,
    pub tokens: Vec<Token>,
    index: SourceIndex,
}

impl<'source> LexedSource<'source> {
    pub fn source(&self) -> &'source str {
        self.source
    }

    pub fn lexeme(&self, token: Token) -> &'source str {
        &self.source[token.span.start..token.span.end]
    }

    pub fn location(&self, byte_offset: usize) -> Option<SourceLocation> {
        self.index.location(self.source, byte_offset)
    }
}

pub(crate) fn lex(source: &[u8]) -> Result<LexedSource<'_>, LexFinding> {
    lex_with_token_limit(source, MAX_TOKENS)
}

pub(crate) fn lex_with_token_limit(
    source: &[u8],
    token_limit: usize,
) -> Result<LexedSource<'_>, LexFinding> {
    if source.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(finding(LexErrorCode::InvalidCharacter, 0, 3));
    }

    let text = match std::str::from_utf8(source) {
        Ok(text) => text,
        Err(error) => {
            let start = error.valid_up_to();
            let end = error
                .error_len()
                .map_or(source.len(), |length| start + length);
            return Err(finding(LexErrorCode::InvalidCharacter, start, end));
        }
    };

    let mut scanner = Scanner {
        source: text,
        offset: 0,
        token_limit,
        tokens: Vec::new(),
    };
    scanner.scan()?;

    Ok(LexedSource {
        source: text,
        tokens: scanner.tokens,
        index: SourceIndex::new(text),
    })
}

struct Scanner<'source> {
    source: &'source str,
    offset: usize,
    token_limit: usize,
    tokens: Vec<Token>,
}

impl Scanner<'_> {
    fn scan(&mut self) -> Result<(), LexFinding> {
        while self.offset < self.source.len() {
            let byte = self.bytes()[self.offset];

            if matches!(byte, b'\t' | b'\n' | b'\r' | b' ') {
                self.offset += 1;
                continue;
            }

            if byte == b'/' && self.bytes().get(self.offset + 1) == Some(&b'/') {
                self.skip_line_comment();
                continue;
            }

            if byte == b'/' && self.bytes().get(self.offset + 1) == Some(&b'*') {
                self.skip_block_comment()?;
                continue;
            }

            if is_identifier_start(byte) {
                self.scan_identifier()?;
                continue;
            }

            if byte.is_ascii_digit() {
                self.scan_number()?;
                continue;
            }

            if byte == b'"' {
                self.scan_string()?;
                continue;
            }

            if !byte.is_ascii() {
                let end = self.offset
                    + self.source[self.offset..]
                        .chars()
                        .next()
                        .expect("offset is inside validated UTF-8")
                        .len_utf8();
                return Err(finding(LexErrorCode::InvalidCharacter, self.offset, end));
            }

            self.scan_punctuation()?;
        }

        Ok(())
    }

    fn bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn skip_line_comment(&mut self) {
        self.offset += 2;
        while let Some(byte) = self.bytes().get(self.offset) {
            if matches!(byte, b'\r' | b'\n') {
                break;
            }
            self.offset += 1;
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        self.offset += 2;

        while self.offset + 1 < self.source.len() {
            if self.bytes()[self.offset] == b'*' && self.bytes()[self.offset + 1] == b'/' {
                self.offset += 2;
                return Ok(());
            }
            self.offset += 1;
        }

        Err(finding(
            LexErrorCode::InvalidStringOrComment,
            start,
            self.source.len(),
        ))
    }

    fn scan_identifier(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        self.offset += 1;
        while self
            .bytes()
            .get(self.offset)
            .is_some_and(|byte| is_identifier_continue(*byte))
        {
            self.offset += 1;
        }

        let text = &self.source[start..self.offset];
        let kind = Keyword::from_identifier(text)
            .map(TokenKind::Keyword)
            .unwrap_or(TokenKind::Identifier);
        self.push_token(kind, start, self.offset)
    }

    fn scan_number(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        self.offset += 1;
        while self
            .bytes()
            .get(self.offset)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.'))
        {
            self.offset += 1;
        }

        let text = &self.source[start..self.offset];
        let kind = if valid_integer(text) {
            TokenKind::Integer
        } else if valid_decimal(text) {
            TokenKind::Decimal
        } else {
            return Err(finding(LexErrorCode::InvalidNumber, start, self.offset));
        };

        self.push_token(kind, start, self.offset)
    }

    fn scan_string(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        self.offset += 1;

        while self.offset < self.source.len() {
            match self.bytes()[self.offset] {
                b'"' => {
                    self.offset += 1;
                    return self.push_token(TokenKind::String, start, self.offset);
                }
                b'\\' => self.scan_escape()?,
                0x00..=0x1F => {
                    return Err(finding(
                        LexErrorCode::InvalidStringOrComment,
                        self.offset,
                        self.offset + 1,
                    ));
                }
                byte if byte.is_ascii() => self.offset += 1,
                _ => {
                    let width = self.source[self.offset..]
                        .chars()
                        .next()
                        .expect("offset is inside validated UTF-8")
                        .len_utf8();
                    self.offset += width;
                }
            }
        }

        Err(finding(
            LexErrorCode::InvalidStringOrComment,
            start,
            self.source.len(),
        ))
    }

    fn scan_escape(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        let Some(&escaped) = self.bytes().get(start + 1) else {
            return Err(finding(
                LexErrorCode::InvalidStringOrComment,
                start,
                self.source.len(),
            ));
        };

        match escaped {
            b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {
                self.offset += 2;
                Ok(())
            }
            b'u' => {
                let (first, after_first) = parse_hex_escape(self.source, start)?;
                if (0xD800..=0xDBFF).contains(&first) {
                    if self.bytes().get(after_first..after_first + 2) != Some(b"\\u") {
                        return Err(finding(
                            LexErrorCode::InvalidStringOrComment,
                            start,
                            after_first,
                        ));
                    }
                    let (second, after_second) = parse_hex_escape(self.source, after_first)?;
                    if !(0xDC00..=0xDFFF).contains(&second) {
                        return Err(finding(
                            LexErrorCode::InvalidStringOrComment,
                            start,
                            after_second,
                        ));
                    }
                    self.offset = after_second;
                    Ok(())
                } else if (0xDC00..=0xDFFF).contains(&first) {
                    Err(finding(
                        LexErrorCode::InvalidStringOrComment,
                        start,
                        after_first,
                    ))
                } else {
                    self.offset = after_first;
                    Ok(())
                }
            }
            _ => {
                let end = start
                    + 1
                    + self.source[start + 1..]
                        .chars()
                        .next()
                        .expect("escape has a validated UTF-8 scalar")
                        .len_utf8();
                Err(finding(LexErrorCode::InvalidStringOrComment, start, end))
            }
        }
    }

    fn scan_punctuation(&mut self) -> Result<(), LexFinding> {
        let start = self.offset;
        let first = self.bytes()[start];
        let second = self.bytes().get(start + 1).copied();
        let (kind, width) = match (first, second) {
            (b'=', Some(b'=')) => (TokenKind::EqualEqual, 2),
            (b'!', Some(b'=')) => (TokenKind::BangEqual, 2),
            (b'<', Some(b'=')) => (TokenKind::LessEqual, 2),
            (b'>', Some(b'=')) => (TokenKind::GreaterEqual, 2),
            (b'{', _) => (TokenKind::LeftBrace, 1),
            (b'}', _) => (TokenKind::RightBrace, 1),
            (b'(', _) => (TokenKind::LeftParen, 1),
            (b')', _) => (TokenKind::RightParen, 1),
            (b'[', _) => (TokenKind::LeftBracket, 1),
            (b']', _) => (TokenKind::RightBracket, 1),
            (b';', _) => (TokenKind::Semicolon, 1),
            (b':', _) => (TokenKind::Colon, 1),
            (b',', _) => (TokenKind::Comma, 1),
            (b'.', _) => (TokenKind::Dot, 1),
            (b'@', _) => (TokenKind::At, 1),
            (b'<', _) => (TokenKind::Less, 1),
            (b'>', _) => (TokenKind::Greater, 1),
            (b'=', _) => (TokenKind::Equal, 1),
            (b'+', _) => (TokenKind::Plus, 1),
            (b'-', _) => (TokenKind::Minus, 1),
            (b'*', _) => (TokenKind::Star, 1),
            (b'/', _) => (TokenKind::Slash, 1),
            (b'%', _) => (TokenKind::Percent, 1),
            _ => {
                return Err(finding(LexErrorCode::InvalidCharacter, start, start + 1));
            }
        };

        self.offset += width;
        self.push_token(kind, start, self.offset)
    }

    fn push_token(&mut self, kind: TokenKind, start: usize, end: usize) -> Result<(), LexFinding> {
        if self.tokens.len() >= self.token_limit {
            return Err(finding(LexErrorCode::TokenLimitExceeded, start, end));
        }
        self.tokens.push(Token {
            kind,
            span: ByteSpan::new(start, end),
        });
        Ok(())
    }
}

fn parse_hex_escape(source: &str, escape_start: usize) -> Result<(u16, usize), LexFinding> {
    let digits_start = escape_start + 2;
    let required_end = digits_start + 4;
    if required_end > source.len() {
        return Err(finding(
            LexErrorCode::InvalidStringOrComment,
            escape_start,
            source.len(),
        ));
    }

    let mut value = 0_u16;
    for offset in digits_start..required_end {
        let byte = source.as_bytes()[offset];
        let Some(digit) = hex_value(byte) else {
            let width = if byte.is_ascii() {
                1
            } else {
                source[offset..]
                    .chars()
                    .next()
                    .expect("offset is inside validated UTF-8")
                    .len_utf8()
            };
            return Err(finding(
                LexErrorCode::InvalidStringOrComment,
                escape_start,
                offset + width,
            ));
        };
        value = (value << 4) | u16::from(digit);
    }

    Ok((value, required_end))
}

fn finding(code: LexErrorCode, start: usize, end: usize) -> LexFinding {
    LexFinding {
        code,
        span: ByteSpan::new(start, end),
    }
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    is_identifier_start(byte) || byte.is_ascii_digit()
}

fn valid_integer(text: &str) -> bool {
    let bytes = text.as_bytes();
    match bytes {
        [b'0'] => true,
        [first, rest @ ..] if matches!(first, b'1'..=b'9') => rest.iter().all(u8::is_ascii_digit),
        _ => false,
    }
}

fn valid_decimal(text: &str) -> bool {
    let Some((integer, fraction)) = text.split_once('.') else {
        return false;
    };
    !fraction.is_empty()
        && !fraction.contains('.')
        && valid_integer(integer)
        && fraction.as_bytes().iter().all(u8::is_ascii_digit)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ALL_KEYWORDS, ByteSpan, Keyword, LexErrorCode, SourceLocation, TokenKind, lex,
        lex_with_token_limit,
    };

    #[test]
    fn grammar_keyword_inventory_is_exact() {
        assert_eq!(ALL_KEYWORDS.len(), 122);
        for &(text, keyword) in ALL_KEYWORDS {
            let result = lex(text.as_bytes()).unwrap();
            assert_eq!(result.tokens.len(), 1, "{text}");
            assert_eq!(result.tokens[0].kind, TokenKind::Keyword(keyword), "{text}");
        }

        let result = lex(b"specification").unwrap();
        assert_eq!(result.tokens[0].kind, TokenKind::Identifier);
        assert_ne!(result.tokens[0].kind, TokenKind::Keyword(Keyword::Spec));
    }

    #[test]
    fn lexes_literals_comments_and_longest_match_operators() {
        let source = br#"apls "0.1"; // comment
spec Demo { version "\uD83D\uDE80"; scale 1.25; offset -0; }
/* outer marker /* is text */ == = != <= < >= >"#;
        let result = lex(source).unwrap();
        let kinds: Vec<_> = result.tokens.iter().map(|token| token.kind).collect();

        assert!(kinds.contains(&TokenKind::Keyword(Keyword::Apls)));
        assert!(kinds.contains(&TokenKind::Identifier));
        assert!(kinds.contains(&TokenKind::String));
        assert!(kinds.contains(&TokenKind::Decimal));
        assert!(kinds.contains(&TokenKind::Integer));
        assert!(kinds.ends_with(&[
            TokenKind::EqualEqual,
            TokenKind::Equal,
            TokenKind::BangEqual,
            TokenKind::LessEqual,
            TokenKind::Less,
            TokenKind::GreaterEqual,
            TokenKind::Greater,
        ]));
    }

    #[test]
    fn byte_spans_and_scalar_locations_follow_the_profile() {
        let result = lex("\"树\"\r\n\tname".as_bytes()).unwrap();
        assert_eq!(result.tokens[0].span, ByteSpan::new(0, 5));
        assert_eq!(result.lexeme(result.tokens[0]), "\"树\"");

        let name = result.tokens[1];
        assert_eq!(name.span, ByteSpan::new(8, 12));
        assert_eq!(
            result.location(name.span.start),
            Some(SourceLocation { line: 2, column: 2 })
        );
    }

    #[test]
    fn rejects_bom_invalid_utf8_and_non_ascii_source_characters() {
        let cases: &[(&[u8], ByteSpan)] = &[
            (b"\xEF\xBB\xBFapls", ByteSpan::new(0, 3)),
            (b"\xFF", ByteSpan::new(0, 1)),
            ("树".as_bytes(), ByteSpan::new(0, 3)),
        ];

        for &(source, span) in cases {
            let finding = lex(source).unwrap_err();
            assert_eq!(finding.code, LexErrorCode::InvalidCharacter);
            assert_eq!(finding.code.as_str(), "APLS-E1001");
            assert_eq!(finding.span, span);
        }
    }

    #[test]
    fn rejects_the_entire_invalid_numeric_candidate() {
        for source in ["01", "1e3", "1.", "1abc", "1.2.3"] {
            let finding = lex(source.as_bytes()).unwrap_err();
            assert_eq!(finding.code, LexErrorCode::InvalidNumber, "{source}");
            assert_eq!(finding.code.as_str(), "APLS-E1003");
            assert_eq!(finding.span, ByteSpan::new(0, source.len()), "{source}");
        }
    }

    #[test]
    fn enforces_json_scalar_escape_rules() {
        assert_eq!(
            lex(br#""\uD83D\uDE80\n\/""#).unwrap().tokens[0].kind,
            TokenKind::String
        );

        for source in [
            br#""\uD83D""#.as_slice(),
            br#""\uDE80""#.as_slice(),
            br#""\q""#.as_slice(),
            b"\"\n\"".as_slice(),
        ] {
            let finding = lex(source).unwrap_err();
            assert_eq!(finding.code, LexErrorCode::InvalidStringOrComment);
            assert_eq!(finding.code.as_str(), "APLS-E1002");
        }
    }

    #[test]
    fn rejects_unterminated_string_and_block_comment() {
        for source in [b"\"open".as_slice(), b"/* open".as_slice()] {
            let finding = lex(source).unwrap_err();
            assert_eq!(finding.code, LexErrorCode::InvalidStringOrComment);
            assert_eq!(finding.span, ByteSpan::new(0, source.len()));
        }
    }

    #[test]
    fn token_limit_fails_at_the_first_excess_token() {
        let finding = lex_with_token_limit(b"apls spec", 1).unwrap_err();
        assert_eq!(finding.code, LexErrorCode::TokenLimitExceeded);
        assert_eq!(finding.code.as_str(), "APLS-T0007");
        assert_eq!(finding.span, ByteSpan::new(5, 9));
    }
}
