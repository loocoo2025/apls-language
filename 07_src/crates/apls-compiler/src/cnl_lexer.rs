use std::collections::BTreeSet;

use unicode_normalization::char::canonical_combining_class;
use unicode_normalization::{UNICODE_VERSION, UnicodeNormalization, is_nfc};

use crate::cnl_ast::{ByteSpan, FixedToken as F, SymbolRef, Token};
use crate::diagnostic::Diagnostic;
use crate::limits::MAX_SOURCE_BYTES;
use crate::resource::{Ledger, Resource, source_bytes_limit};

pub const UNICODE_DATA_VERSION: (u8, u8, u8) = (17, 0, 0);

#[derive(Clone, Debug)]
pub struct Sentence<'a> {
    pub index: usize,
    pub span: ByteSpan,
    pub text: &'a str,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Edge {
    pass_rank: u8,
    sentence_index: usize,
    start: usize,
    end: usize,
    kind_rank: u8,
    symbol: String,
    lexeme: String,
    token: Token,
}

pub type TokenStream = Vec<(usize, Token, usize)>;

const FIXED: &[(&str, F)] = &[
    ("简体中文语言版本", F::LanguageVersion),
    ("可观测且可写", F::Both),
    ("验收要求", F::AcceptanceRequirement),
    ("安全要求", F::SafetyRequirement),
    ("初始状态是", F::InitialStateIs),
    ("的状态包括", F::StateIncludes),
    ("状态进入", F::StateEnter),
    ("本规范采用", F::ProfilePrefix),
    ("可观测", F::Observable),
    ("不高于", F::NotAbove),
    ("不低于", F::NotBelow),
    ("必须在", F::MustWithin),
    ("内成立", F::HoldsWithin),
    ("单位为", F::UnitIs),
    ("支持的", F::Supported),
    ("的别名", F::Alias),
    ("类型的", F::TypeSuffix),
    ("摄氏度", F::Celsius),
    ("百分比", F::PercentageType),
    ("执行器", F::EntityActuator),
    ("传感器", F::EntitySensor),
    ("不等于", F::NotEquals),
    ("并且", F::Conjunction),
    ("分钟", F::Minute),
    ("说明", F::Explanation),
    ("必须", F::Must),
    ("禁止", F::Prohibit),
    ("不得", F::MustNot),
    ("处于", F::InState),
    ("收到", F::Received),
    ("不高于", F::NotAbove),
    ("不低于", F::NotBelow),
    ("等于", F::Equals),
    ("低于", F::Below),
    ("高于", F::Above),
    ("状态", F::StateWord),
    ("毫秒", F::Millisecond),
    ("系统", F::System),
    ("可写", F::Writable),
    ("小数", F::DecimalType),
    ("布尔", F::BooleanType),
    ("整数", F::IntegerType),
    ("文本", F::TextType),
    ("时长", F::DurationType),
    ("设备", F::EntityDevice),
    ("组件", F::EntityComponent),
    ("属性", F::Property),
    ("动作", F::Action),
    ("事件", F::Event),
    ("实体", F::Entity),
    ("APLS", F::Apls),
    ("0.1", F::Version01),
    ("如果", F::If),
    ("当", F::When),
    ("时", F::Then),
    ("从", F::From),
    ("和", F::AndList),
    ("是", F::Is),
    ("单位", F::Unit),
    ("真", F::True),
    ("假", F::False),
    ("秒", F::Second),
    ("。", F::FullStop),
    ("，", F::Comma),
    ("：", F::Colon),
    ("、", F::EnumerationComma),
    ("%", F::Percent),
];

pub fn validate_and_split<'a>(
    bytes: &'a [u8],
    path: &str,
) -> Result<(&'a str, Vec<Sentence<'a>>), Diagnostic> {
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(source_bytes_limit(bytes.len()));
    }
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(Diagnostic::source(
            "APLS-E1001",
            "UTF-8 BOM is forbidden",
            path,
            "",
            ByteSpan::new(0, 3),
        ));
    }
    let source = std::str::from_utf8(bytes).map_err(|error| {
        let at = error.valid_up_to();
        Diagnostic::source(
            "APLS-E1001",
            "source is not valid UTF-8",
            path,
            "",
            ByteSpan::new(at, (at + error.error_len().unwrap_or(1)).min(bytes.len())),
        )
    })?;
    if UNICODE_VERSION != UNICODE_DATA_VERSION {
        return Err(Diagnostic::tool(
            "APLS-T0006",
            "Unicode normalization data version mismatch",
        ));
    }
    if !is_nfc(source) {
        let span = first_non_nfc_span(source);
        return Err(Diagnostic::source(
            "APLS-E1004",
            "source must already be Unicode NFC",
            path,
            source,
            span,
        ));
    }
    let mut sentences = Vec::new();
    let mut start = 0;
    let mut quote = None;
    for (index, ch) in source.char_indices() {
        match ch {
            '\r' if !source[index..].starts_with("\r\n") => {
                return Err(Diagnostic::source(
                    "APLS-E1005",
                    "standalone CR is forbidden",
                    path,
                    source,
                    ByteSpan::new(index, index + 1),
                ));
            }
            '\t' => {
                return Err(Diagnostic::source(
                    "APLS-E1005",
                    "tab is forbidden",
                    path,
                    source,
                    ByteSpan::new(index, index + 1),
                ));
            }
            c if c.is_control() && c != '\n' && c != '\r' => {
                return Err(Diagnostic::source(
                    "APLS-E1001",
                    "control character is forbidden",
                    path,
                    source,
                    ByteSpan::new(index, index + c.len_utf8()),
                ));
            }
            '\n' | '\r' if quote.is_some() => {
                return Err(Diagnostic::source(
                    "APLS-E1007",
                    "quoted text cannot cross a line",
                    path,
                    source,
                    ByteSpan::new(index, index + ch.len_utf8()),
                ));
            }
            '“' | '『' if quote.is_some() => {
                return Err(Diagnostic::source(
                    "APLS-E1007",
                    "nested quotes are forbidden",
                    path,
                    source,
                    ByteSpan::new(index, index + ch.len_utf8()),
                ));
            }
            '“' => quote = Some('”'),
            '『' => quote = Some('』'),
            '”' | '』' if quote == Some(ch) => quote = None,
            '”' | '』' => {
                return Err(Diagnostic::source(
                    "APLS-E1007",
                    "unmatched closing quote",
                    path,
                    source,
                    ByteSpan::new(index, index + ch.len_utf8()),
                ));
            }
            '。' if quote.is_none() => {
                let end = index + ch.len_utf8();
                let content_start = skip_spacing(source, start, end);
                if content_start < index {
                    sentences.push(Sentence {
                        index: sentences.len(),
                        span: ByteSpan::new(content_start, end),
                        text: &source[content_start..end],
                    });
                } else {
                    return Err(Diagnostic::source(
                        "APLS-E1006",
                        "empty sentences are forbidden",
                        path,
                        source,
                        ByteSpan::new(index, end),
                    ));
                }
                start = end;
            }
            c if is_forbidden_noncharacter(c) => {
                return Err(Diagnostic::source(
                    "APLS-E1001",
                    "Unicode noncharacter is forbidden",
                    path,
                    source,
                    ByteSpan::new(index, index + c.len_utf8()),
                ));
            }
            c if c.is_whitespace() && c != ' ' && c != '\n' && c != '\r' => {
                return Err(Diagnostic::source(
                    "APLS-E1005",
                    "unsupported Unicode whitespace",
                    path,
                    source,
                    ByteSpan::new(index, index + c.len_utf8()),
                ));
            }
            _ => {}
        }
    }
    if quote.is_some() {
        return Err(Diagnostic::source(
            "APLS-E1007",
            "unclosed quote",
            path,
            source,
            ByteSpan::new(start.min(source.len()), source.len()),
        ));
    }
    if skip_spacing(source, start, source.len()) != source.len() {
        return Err(Diagnostic::source(
            "APLS-E1006",
            "sentence must end with the full-width full stop",
            path,
            source,
            ByteSpan::new(start, source.len()),
        ));
    }
    if sentences.is_empty() {
        return Err(Diagnostic::source(
            "APLS-E1104",
            "the exact language profile declaration is required",
            path,
            source,
            ByteSpan::new(0, source.len()),
        ));
    }
    Ok((source, sentences))
}

fn first_non_nfc_span(source: &str) -> ByteSpan {
    let normalized: String = source.nfc().collect();
    let mut first_difference = 0usize;
    for (original, canonical) in source.chars().zip(normalized.chars()) {
        if original != canonical {
            break;
        }
        first_difference += original.len_utf8();
    }
    first_difference = first_difference.min(source.len());

    let mut start = first_difference;
    if start < source.len()
        && canonical_combining_class(source[start..].chars().next().unwrap()) != 0
    {
        for (index, ch) in source[..start].char_indices().rev() {
            start = index;
            if canonical_combining_class(ch) == 0 {
                break;
            }
        }
    }
    for (relative, ch) in source[start..].char_indices() {
        let end = start + relative + ch.len_utf8();
        if !is_nfc(&source[start..end]) {
            return ByteSpan::new(start, end);
        }
    }
    ByteSpan::new(start, source.len())
}

fn is_forbidden_noncharacter(ch: char) -> bool {
    let value = ch as u32;
    (0xfdd0..=0xfdef).contains(&value) || value & 0xffff >= 0xfffe
}
fn is_spacing(ch: char) -> bool {
    matches!(ch, ' ' | '\n' | '\r')
}
fn skip_spacing(source: &str, mut at: usize, end: usize) -> usize {
    while at < end {
        let ch = source[at..].chars().next().expect("character boundary");
        if !is_spacing(ch) {
            break;
        }
        at += ch.len_utf8();
    }
    at
}

fn build_edges(
    sentence: &Sentence<'_>,
    pass_rank: u8,
    symbols: &[SymbolRef],
    source: &str,
) -> BTreeSet<Edge> {
    let mut edges = BTreeSet::new();
    let mut at = sentence.span.start_byte;
    while at < sentence.span.end_byte {
        at = skip_spacing(source, at, sentence.span.end_byte);
        if at == sentence.span.end_byte {
            break;
        }
        for &(lexeme, fixed) in FIXED {
            if source[at..sentence.span.end_byte].starts_with(lexeme) {
                push_edge(
                    &mut edges,
                    pass_rank,
                    sentence.index,
                    at,
                    at + lexeme.len(),
                    0,
                    "",
                    lexeme,
                    Token::Fixed(fixed),
                );
            }
        }
        if source[at..].starts_with('“') {
            if let Some(relative) = source[at + '“'.len_utf8()..sentence.span.end_byte].find('”')
            {
                let end = at + '“'.len_utf8() + relative + '”'.len_utf8();
                let name_start = at + '“'.len_utf8();
                let name_end = end - '”'.len_utf8();
                let name = &source[name_start..name_end];
                if valid_term(name) {
                    if pass_rank == 1 {
                        push_edge(
                            &mut edges,
                            pass_rank,
                            sentence.index,
                            at,
                            end,
                            1,
                            "",
                            name,
                            Token::DeclaredTerm(name.into()),
                        );
                    }
                    if pass_rank == 2 {
                        add_symbol_edges(
                            &mut edges,
                            pass_rank,
                            sentence.index,
                            at,
                            end,
                            name,
                            true,
                            symbols,
                        );
                    }
                }
            }
        }
        if pass_rank == 2 && source[at..].starts_with('『') {
            if let Some(relative) = source[at + '『'.len_utf8()..sentence.span.end_byte].find('』')
            {
                let end = at + '『'.len_utf8() + relative + '』'.len_utf8();
                let content = &source[at + '『'.len_utf8()..end - '』'.len_utf8()];
                push_edge(
                    &mut edges,
                    pass_rank,
                    sentence.index,
                    at,
                    end,
                    2,
                    "",
                    content,
                    Token::InformativeText(content.into()),
                );
                push_edge(
                    &mut edges,
                    pass_rank,
                    sentence.index,
                    at,
                    end,
                    3,
                    "",
                    content,
                    Token::Text(content.into()),
                );
            }
        }
        if pass_rank == 2 {
            add_bare_symbol_edges(&mut edges, pass_rank, sentence.index, at, source, symbols);
        }
        if pass_rank != 0 {
            add_number_edges(
                &mut edges,
                pass_rank,
                sentence.index,
                at,
                sentence.span.end_byte,
                source,
            );
        }
        let ch = source[at..].chars().next().expect("character boundary");
        at += ch.len_utf8();
    }
    edges
}

pub(crate) struct LexicalCoverage {
    covered: Vec<bool>,
    base: usize,
    pub(crate) first_stall: Option<ByteSpan>,
}

impl LexicalCoverage {
    pub(crate) fn covers(&self, span: ByteSpan) -> bool {
        span.start_byte >= self.base
            && span.end_byte <= self.base + self.covered.len()
            && self.covered[span.start_byte - self.base..span.end_byte - self.base]
                .iter()
                .all(|covered| *covered)
    }
}

/// Lattice 直接证据：Edge 覆盖的 Byte 区间与首个无法继续的位置。
/// 只描述候选词法 Lattice 的机械覆盖事实，不做最长匹配或评分。
pub(crate) fn lexical_coverage(
    sentence: &Sentence<'_>,
    pass_rank: u8,
    symbols: &[SymbolRef],
    source: &str,
) -> LexicalCoverage {
    let edges = build_edges(sentence, pass_rank, symbols, source);
    let base = sentence.span.start_byte;
    let end = sentence.span.end_byte;
    let mut covered = vec![false; end - base];
    let mut by_start = std::collections::BTreeMap::<usize, Vec<&Edge>>::new();
    for edge in &edges {
        for byte in edge.start..edge.end {
            covered[byte - base] = true;
        }
        by_start.entry(edge.start).or_default().push(edge);
    }
    let mut reached = BTreeSet::new();
    let mut queue = std::collections::VecDeque::from([skip_spacing(source, base, end)]);
    let mut stall = None;
    while let Some(at) = queue.pop_front() {
        if at == end || !reached.insert(at) {
            continue;
        }
        match by_start.get(&at) {
            Some(options) => {
                for edge in options {
                    queue.push_back(skip_spacing(source, edge.end, end));
                }
            }
            None => {
                stall = Some(stall.map_or(at, |known: usize| known.min(at)));
            }
        }
    }
    let first_stall = stall.map(|at| {
        let mut stop = edges
            .iter()
            .filter(|edge| edge.start > at)
            .map(|edge| edge.start)
            .min()
            .unwrap_or(end);
        while stop > at {
            let ch = source[..stop]
                .chars()
                .next_back()
                .expect("character boundary");
            if !is_spacing(ch) {
                break;
            }
            stop -= ch.len_utf8();
        }
        ByteSpan::new(at, stop.max(at + 1))
    });
    LexicalCoverage {
        covered,
        base,
        first_stall,
    }
}

pub fn lex_and_enumerate(
    sentence: &Sentence<'_>,
    pass_rank: u8,
    symbols: &[SymbolRef],
    ledger: &mut Ledger,
    path: &str,
    source: &str,
) -> Result<Vec<TokenStream>, Diagnostic> {
    let edges = build_edges(sentence, pass_rank, symbols, source);
    for edge in &edges {
        ledger.add(
            Resource::LatticeEdges,
            1,
            Some(sentence.index),
            Some(ByteSpan::new(edge.start, edge.end)),
            path,
            source,
        )?;
    }
    let mut by_start = std::collections::BTreeMap::<usize, Vec<Edge>>::new();
    for edge in edges {
        by_start.entry(edge.start).or_default().push(edge);
    }
    let mut streams = Vec::new();
    let mut current = Vec::new();
    enumerate(
        sentence,
        &by_start,
        sentence.span.start_byte,
        &mut current,
        &mut streams,
        ledger,
        path,
        source,
    )?;
    for stream in &streams {
        for (left, _, right) in stream {
            ledger.add(
                Resource::TokenOccurrences,
                1,
                Some(sentence.index),
                Some(ByteSpan::new(*left, *right)),
                path,
                source,
            )?;
        }
    }
    Ok(streams)
}

fn enumerate(
    sentence: &Sentence<'_>,
    edges: &std::collections::BTreeMap<usize, Vec<Edge>>,
    at: usize,
    current: &mut Vec<(usize, Token, usize)>,
    streams: &mut Vec<TokenStream>,
    ledger: &mut Ledger,
    path: &str,
    source: &str,
) -> Result<(), Diagnostic> {
    let at = skip_spacing(source, at, sentence.span.end_byte);
    if at == sentence.span.end_byte {
        ledger.add_stream(sentence.index, sentence.span, path, source)?;
        streams.push(current.clone());
        return Ok(());
    }
    if let Some(options) = edges.get(&at) {
        for edge in options {
            current.push((edge.start, edge.token.clone(), edge.end));
            enumerate(
                sentence, edges, edge.end, current, streams, ledger, path, source,
            )?;
            current.pop();
        }
    }
    Ok(())
}

fn push_edge(
    edges: &mut BTreeSet<Edge>,
    pass_rank: u8,
    sentence_index: usize,
    start: usize,
    end: usize,
    kind_rank: u8,
    symbol: &str,
    lexeme: &str,
    mut token: Token,
) {
    match &mut token {
        Token::EntityRef(value)
        | Token::PropertyRef(value)
        | Token::ActionRef(value)
        | Token::EventRef(value)
        | Token::StateRef(value)
        | Token::UnitRef(value) => value.span = ByteSpan::new(start, end),
        _ => {}
    }
    edges.insert(Edge {
        pass_rank,
        sentence_index,
        start,
        end,
        kind_rank,
        symbol: symbol.into(),
        lexeme: lexeme.into(),
        token,
    });
}

fn add_symbol_edges(
    edges: &mut BTreeSet<Edge>,
    pass_rank: u8,
    sentence_index: usize,
    start: usize,
    end: usize,
    name: &str,
    exact: bool,
    symbols: &[SymbolRef],
) {
    for symbol in symbols
        .iter()
        .filter(|candidate| candidate.display_name == name)
    {
        let base = if exact { 8 } else { 14 };
        let rank = base
            + match symbol.kind {
                crate::cnl_ast::SymbolKind::Entity => 0,
                crate::cnl_ast::SymbolKind::Property => 1,
                crate::cnl_ast::SymbolKind::Action => 2,
                crate::cnl_ast::SymbolKind::Event => 3,
                crate::cnl_ast::SymbolKind::State => 4,
                crate::cnl_ast::SymbolKind::Unit => 5,
            };
        let token = match symbol.kind {
            crate::cnl_ast::SymbolKind::Entity => Token::EntityRef(symbol.clone()),
            crate::cnl_ast::SymbolKind::Property => Token::PropertyRef(symbol.clone()),
            crate::cnl_ast::SymbolKind::Action => Token::ActionRef(symbol.clone()),
            crate::cnl_ast::SymbolKind::Event => Token::EventRef(symbol.clone()),
            crate::cnl_ast::SymbolKind::State => Token::StateRef(symbol.clone()),
            crate::cnl_ast::SymbolKind::Unit => Token::UnitRef(symbol.clone()),
        };
        push_edge(
            edges,
            pass_rank,
            sentence_index,
            start,
            end,
            rank,
            &symbol.id,
            name,
            token,
        );
    }
}

fn add_bare_symbol_edges(
    edges: &mut BTreeSet<Edge>,
    pass_rank: u8,
    sentence_index: usize,
    at: usize,
    source: &str,
    symbols: &[SymbolRef],
) {
    for symbol in symbols {
        if source[at..].starts_with(&symbol.display_name) {
            add_symbol_edges(
                edges,
                pass_rank,
                sentence_index,
                at,
                at + symbol.display_name.len(),
                &symbol.display_name,
                false,
                std::slice::from_ref(symbol),
            );
        }
    }
}

fn add_number_edges(
    edges: &mut BTreeSet<Edge>,
    pass_rank: u8,
    sentence_index: usize,
    at: usize,
    end: usize,
    source: &str,
) {
    let bytes = source.as_bytes();
    let mut cursor = at;
    if cursor < end && bytes[cursor] == b'-' {
        cursor += 1;
    }
    let digits_start = cursor;
    while cursor < end && bytes[cursor].is_ascii_digit() {
        cursor += 1;
    }
    if cursor == digits_start {
        return;
    }
    let integer = &source[at..cursor];
    let unsigned = !integer.starts_with('-');
    if valid_integer(integer.trim_start_matches('-')) {
        push_edge(
            edges,
            pass_rank,
            sentence_index,
            at,
            cursor,
            4,
            "",
            integer,
            Token::SignedInteger(integer.into()),
        );
        if unsigned {
            push_edge(
                edges,
                pass_rank,
                sentence_index,
                at,
                cursor,
                6,
                "",
                integer,
                Token::UnsignedInteger(integer.into()),
            );
        }
    }
    if cursor < end && bytes[cursor] == b'.' {
        let dot = cursor;
        cursor += 1;
        let fraction = cursor;
        while cursor < end && bytes[cursor].is_ascii_digit() {
            cursor += 1;
        }
        if cursor > fraction && valid_integer(&source[at..dot].trim_start_matches('-')) {
            let decimal = &source[at..cursor];
            push_edge(
                edges,
                pass_rank,
                sentence_index,
                at,
                cursor,
                5,
                "",
                decimal,
                Token::SignedDecimal(decimal.into()),
            );
            if unsigned {
                push_edge(
                    edges,
                    pass_rank,
                    sentence_index,
                    at,
                    cursor,
                    7,
                    "",
                    decimal,
                    Token::UnsignedDecimal(decimal.into()),
                );
            }
        }
    }
}

fn valid_integer(value: &str) -> bool {
    value == "0" || (!value.starts_with('0') && value.bytes().all(|b| b.is_ascii_digit()))
}
pub fn valid_term(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let first_ok =
        first.is_ascii_alphabetic() || matches!(first as u32, 0x3400..=0x4dbf | 0x4e00..=0x9fff);
    first_ok
        && value.chars().count() <= 64
        && chars.all(|ch| {
            ch.is_ascii_alphanumeric()
                || matches!(ch, '_' | '-' | '·')
                || matches!(ch as u32, 0x3400..=0x4dbf | 0x4e00..=0x9fff)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalars(field: &str) -> String {
        field
            .split_whitespace()
            .map(|scalar| {
                char::from_u32(u32::from_str_radix(scalar, 16).expect("hex scalar"))
                    .expect("Unicode scalar value")
            })
            .collect()
    }

    #[test]
    #[ignore = "requires the official Unicode 17.0.0 NormalizationTest.txt"]
    fn unicode_17_official_normalization_conformance() {
        let path = std::env::var("APLS_UNICODE_NORMALIZATION_TEST")
            .expect("set APLS_UNICODE_NORMALIZATION_TEST to Unicode 17.0.0 data");
        let data = std::fs::read_to_string(path).expect("read official conformance data");
        assert!(
            data.contains("NormalizationTest-17.0.0.txt"),
            "conformance data must declare Unicode 17.0.0"
        );
        let mut cases = 0usize;
        for line in data.lines() {
            let body = line.split('#').next().unwrap_or("").trim();
            if body.is_empty() || body.starts_with('@') {
                continue;
            }
            let columns: Vec<_> = body.split(';').map(str::trim).collect();
            assert!(columns.len() >= 5, "malformed conformance row: {line}");
            let c1 = scalars(columns[0]);
            let c2 = scalars(columns[1]);
            let c3 = scalars(columns[2]);
            let c4 = scalars(columns[3]);
            let c5 = scalars(columns[4]);
            assert_eq!(is_nfc(&c1), c1 == c2, "c1: {line}");
            assert!(is_nfc(&c2), "c2: {line}");
            assert_eq!(is_nfc(&c3), c3 == c2, "c3: {line}");
            assert!(is_nfc(&c4), "c4: {line}");
            assert_eq!(is_nfc(&c5), c5 == c4, "c5: {line}");
            cases += 1;
        }
        assert!(cases > 10_000, "official conformance corpus is incomplete");
    }
}
