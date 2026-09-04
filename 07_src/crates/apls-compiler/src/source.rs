//! Deterministic, host-independent Source Graph construction.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::lexer::{ByteSpan, LexErrorCode};
use crate::limits::{
    MAX_IMPORT_DEPTH, MAX_SOURCE_BYTES, MAX_SOURCE_FILES, MAX_SOURCE_GRAPH_BYTES, MAX_TOKENS,
};
use crate::parser::{ParseFailure, parse_with_token_limit};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct LogicalPath(String);

impl LogicalPath {
    pub(crate) fn parse(path: &str) -> Result<Self, ()> {
        if path.is_empty()
            || !path.ends_with(".apls")
            || path.starts_with('/')
            || path.contains('\\')
            || has_uri_scheme(path)
            || path
                .split('/')
                .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
        {
            return Err(());
        }
        Ok(Self(path.to_owned()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

fn has_uri_scheme(path: &str) -> bool {
    let Some(colon) = path.find(':') else {
        return false;
    };
    let candidate = &path[..colon];
    !candidate.is_empty()
        && candidate.as_bytes()[0].is_ascii_alphabetic()
        && candidate
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PhysicalIdentity(pub Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReadSource {
    pub bytes: Vec<u8>,
    pub physical_identity: PhysicalIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceReadError {
    UnavailableOrNotRegular,
    UnsafeOrAmbiguous,
}

pub(crate) trait SourceReader {
    fn read(&mut self, logical_path: &LogicalPath) -> Result<ReadSource, SourceReadError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceToolCode {
    UnavailableOrNotRegular,
    UnsafeOrAmbiguousPath,
    SourceChanged,
    ResourceLimitExceeded,
}

impl SourceToolCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::UnavailableOrNotRegular => "APLS-T0002",
            Self::UnsafeOrAmbiguousPath => "APLS-T0003",
            Self::SourceChanged => "APLS-T0004",
            Self::ResourceLimitExceeded => "APLS-T0007",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResourceKind {
    SourceBytes,
    SourceGraphBytes,
    SourceCount,
    ImportDepth,
    TokenCount,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceToolFinding {
    pub code: SourceToolCode,
    pub logical_path: Option<LogicalPath>,
    pub span: Option<ByteSpan>,
    pub resource: Option<ResourceKind>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceErrorCode {
    DuplicateImportAlias,
    ImportCycle,
}

impl SourceErrorCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateImportAlias => "APLS-E2002",
            Self::ImportCycle => "APLS-E2004",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceGraphFinding {
    pub code: SourceErrorCode,
    pub logical_path: LogicalPath,
    pub span: ByteSpan,
    pub related_spans: Vec<(LogicalPath, ByteSpan)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SourceGraphFailure {
    Tool(SourceToolFinding),
    Parse {
        logical_path: LogicalPath,
        failure: ParseFailure,
    },
    Source(SourceGraphFinding),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImportEdge {
    pub source: LogicalPath,
    pub target: LogicalPath,
    pub alias: String,
    pub path_span: ByteSpan,
    pub alias_span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LoadedSource {
    pub logical_path: LogicalPath,
    pub bytes: Box<[u8]>,
    pub sha256: [u8; 32],
    physical_identity: PhysicalIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LoadedBundle {
    pub sources: Vec<LoadedSource>,
    pub imports: Vec<ImportEdge>,
    pub token_count: usize,
}

impl LoadedBundle {
    pub(crate) fn verify_unchanged(
        &self,
        reader: &mut impl SourceReader,
    ) -> Result<(), SourceGraphFailure> {
        for source in &self.sources {
            let reread = reader
                .read(&source.logical_path)
                .map_err(|error| read_failure(source.logical_path.clone(), error))?;
            let digest: [u8; 32] = Sha256::digest(&reread.bytes).into();
            if reread.physical_identity != source.physical_identity || digest != source.sha256 {
                return Err(tool_failure(
                    SourceToolCode::SourceChanged,
                    Some(source.logical_path.clone()),
                    None,
                    None,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct SourceLimits {
    source_bytes: usize,
    graph_bytes: usize,
    source_count: usize,
    import_depth: usize,
    token_count: usize,
}

impl Default for SourceLimits {
    fn default() -> Self {
        Self {
            source_bytes: MAX_SOURCE_BYTES,
            graph_bytes: MAX_SOURCE_GRAPH_BYTES,
            source_count: MAX_SOURCE_FILES,
            import_depth: MAX_IMPORT_DEPTH,
            token_count: MAX_TOKENS,
        }
    }
}

pub(crate) fn load_source_graph(
    entry: &str,
    reader: &mut impl SourceReader,
) -> Result<LoadedBundle, SourceGraphFailure> {
    load_source_graph_with_limits(entry, reader, SourceLimits::default())
}

fn load_source_graph_with_limits(
    entry: &str,
    reader: &mut impl SourceReader,
    limits: SourceLimits,
) -> Result<LoadedBundle, SourceGraphFailure> {
    let entry = LogicalPath::parse(entry)
        .map_err(|()| tool_failure(SourceToolCode::UnsafeOrAmbiguousPath, None, None, None))?;
    let mut pending = BTreeSet::from([entry.clone()]);
    let mut loaded = BTreeMap::<LogicalPath, LoadedSource>::new();
    let mut imports = Vec::<ImportEdge>::new();
    let mut graph_bytes = 0_usize;
    let mut token_count = 0_usize;

    while let Some(path) = pending.pop_first() {
        if loaded.contains_key(&path) {
            continue;
        }
        if loaded.len() >= limits.source_count {
            return Err(resource_failure(
                ResourceKind::SourceCount,
                Some(path),
                None,
            ));
        }

        let read = reader
            .read(&path)
            .map_err(|error| read_failure(path.clone(), error))?;
        if read.bytes.len() > limits.source_bytes {
            return Err(resource_failure(
                ResourceKind::SourceBytes,
                Some(path),
                None,
            ));
        }
        let Some(new_graph_bytes) = graph_bytes.checked_add(read.bytes.len()) else {
            return Err(resource_failure(
                ResourceKind::SourceGraphBytes,
                Some(path),
                None,
            ));
        };
        if new_graph_bytes > limits.graph_bytes {
            return Err(resource_failure(
                ResourceKind::SourceGraphBytes,
                Some(path),
                None,
            ));
        }

        let parsed = parse_with_token_limit(&read.bytes, limits.token_count - token_count)
            .map_err(|failure| match failure {
                ParseFailure::Lex(finding) if finding.code == LexErrorCode::TokenLimitExceeded => {
                    resource_failure(
                        ResourceKind::TokenCount,
                        Some(path.clone()),
                        Some(finding.span),
                    )
                }
                failure => SourceGraphFailure::Parse {
                    logical_path: path.clone(),
                    failure,
                },
            })?;
        token_count += parsed.token_count;

        let mut direct_edges = Vec::with_capacity(parsed.program.unit.imports.len());
        for import in parsed.program.unit.imports {
            let target = LogicalPath::parse(&import.path.value).map_err(|()| {
                tool_failure(
                    SourceToolCode::UnsafeOrAmbiguousPath,
                    Some(path.clone()),
                    Some(import.path.span),
                    None,
                )
            })?;
            direct_edges.push(ImportEdge {
                source: path.clone(),
                target,
                alias: import.alias.value,
                path_span: import.path.span,
                alias_span: import.alias.span,
            });
        }
        direct_edges.sort_by(|left, right| {
            left.target
                .cmp(&right.target)
                .then_with(|| left.alias.as_bytes().cmp(right.alias.as_bytes()))
                .then_with(|| left.path_span.start.cmp(&right.path_span.start))
        });
        for edge in &direct_edges {
            if !loaded.contains_key(&edge.target) {
                pending.insert(edge.target.clone());
            }
        }
        imports.extend(direct_edges);

        let sha256 = Sha256::digest(&read.bytes).into();
        graph_bytes = new_graph_bytes;
        loaded.insert(
            path.clone(),
            LoadedSource {
                logical_path: path,
                bytes: read.bytes.into_boxed_slice(),
                sha256,
                physical_identity: read.physical_identity,
            },
        );
    }

    imports.sort_by(|left, right| {
        left.source
            .cmp(&right.source)
            .then_with(|| left.target.cmp(&right.target))
            .then_with(|| left.alias.as_bytes().cmp(right.alias.as_bytes()))
            .then_with(|| left.path_span.start.cmp(&right.path_span.start))
    });

    reject_cycles(&entry, &loaded, &imports)?;
    reject_duplicate_aliases(&imports)?;
    reject_duplicate_physical_files(&entry, &loaded)?;
    enforce_import_depth(&entry, &imports, limits.import_depth)?;

    let mut sources = Vec::with_capacity(loaded.len());
    sources.push(loaded.remove(&entry).expect("entry was loaded"));
    sources.extend(loaded.into_values());
    Ok(LoadedBundle {
        sources,
        imports,
        token_count,
    })
}

fn reject_cycles(
    entry: &LogicalPath,
    loaded: &BTreeMap<LogicalPath, LoadedSource>,
    imports: &[ImportEdge],
) -> Result<(), SourceGraphFailure> {
    let adjacency = adjacency(imports);
    let mut colors = BTreeMap::<LogicalPath, u8>::new();
    let mut stack = Vec::<usize>::new();
    visit_for_cycle(entry, &adjacency, &mut colors, &mut stack, imports)?;
    for path in loaded.keys() {
        visit_for_cycle(path, &adjacency, &mut colors, &mut stack, imports)?;
    }
    Ok(())
}

fn visit_for_cycle(
    path: &LogicalPath,
    adjacency: &BTreeMap<LogicalPath, Vec<usize>>,
    colors: &mut BTreeMap<LogicalPath, u8>,
    stack: &mut Vec<usize>,
    imports: &[ImportEdge],
) -> Result<(), SourceGraphFailure> {
    match colors.get(path).copied().unwrap_or(0) {
        1 => return Ok(()),
        2 => return Ok(()),
        _ => {}
    }
    colors.insert(path.clone(), 1);
    if let Some(edges) = adjacency.get(path) {
        for &edge_index in edges {
            let edge = &imports[edge_index];
            if colors.get(&edge.target).copied() == Some(1) {
                let cycle_start = stack
                    .iter()
                    .position(|&index| imports[index].source == edge.target)
                    .unwrap_or(stack.len());
                let mut related_spans = stack[cycle_start..]
                    .iter()
                    .map(|&index| {
                        let prior = &imports[index];
                        (prior.source.clone(), prior.path_span)
                    })
                    .collect::<Vec<_>>();
                related_spans.sort_by(|left, right| {
                    left.0
                        .cmp(&right.0)
                        .then_with(|| left.1.start.cmp(&right.1.start))
                });
                return Err(SourceGraphFailure::Source(SourceGraphFinding {
                    code: SourceErrorCode::ImportCycle,
                    logical_path: edge.source.clone(),
                    span: edge.path_span,
                    related_spans,
                }));
            }
            if colors.get(&edge.target).copied().unwrap_or(0) == 0 {
                stack.push(edge_index);
                visit_for_cycle(&edge.target, adjacency, colors, stack, imports)?;
                stack.pop();
            }
        }
    }
    colors.insert(path.clone(), 2);
    Ok(())
}

fn reject_duplicate_aliases(imports: &[ImportEdge]) -> Result<(), SourceGraphFailure> {
    let mut seen = BTreeMap::<(LogicalPath, Vec<u8>), ByteSpan>::new();
    let mut by_source_offset = imports.iter().collect::<Vec<_>>();
    by_source_offset.sort_by(|left, right| {
        left.source
            .cmp(&right.source)
            .then_with(|| left.alias_span.start.cmp(&right.alias_span.start))
    });
    for edge in by_source_offset {
        let key = (edge.source.clone(), edge.alias.as_bytes().to_vec());
        if let Some(first_span) = seen.insert(key, edge.alias_span) {
            return Err(SourceGraphFailure::Source(SourceGraphFinding {
                code: SourceErrorCode::DuplicateImportAlias,
                logical_path: edge.source.clone(),
                span: edge.alias_span,
                related_spans: vec![(edge.source.clone(), first_span)],
            }));
        }
    }
    Ok(())
}

fn reject_duplicate_physical_files(
    entry: &LogicalPath,
    loaded: &BTreeMap<LogicalPath, LoadedSource>,
) -> Result<(), SourceGraphFailure> {
    let mut seen = BTreeMap::<PhysicalIdentity, LogicalPath>::new();
    let entry_source = loaded.get(entry).expect("entry was loaded");
    seen.insert(
        entry_source.physical_identity.clone(),
        entry_source.logical_path.clone(),
    );
    for source in loaded
        .values()
        .filter(|source| &source.logical_path != entry)
    {
        if let Some(first_path) = seen.insert(
            source.physical_identity.clone(),
            source.logical_path.clone(),
        ) {
            if first_path != source.logical_path {
                return Err(tool_failure(
                    SourceToolCode::UnsafeOrAmbiguousPath,
                    Some(source.logical_path.clone()),
                    None,
                    None,
                ));
            }
        }
    }
    Ok(())
}

fn enforce_import_depth(
    entry: &LogicalPath,
    imports: &[ImportEdge],
    limit: usize,
) -> Result<(), SourceGraphFailure> {
    let adjacency = adjacency(imports);
    let mut memo = BTreeMap::<LogicalPath, usize>::new();
    let depth = longest_depth(entry, &adjacency, imports, &mut memo);
    if depth > limit {
        return Err(resource_failure(
            ResourceKind::ImportDepth,
            Some(entry.clone()),
            None,
        ));
    }
    Ok(())
}

fn longest_depth(
    path: &LogicalPath,
    adjacency: &BTreeMap<LogicalPath, Vec<usize>>,
    imports: &[ImportEdge],
    memo: &mut BTreeMap<LogicalPath, usize>,
) -> usize {
    if let Some(depth) = memo.get(path) {
        return *depth;
    }
    let depth = adjacency.get(path).map_or(0, |edges| {
        edges
            .iter()
            .map(|&index| 1 + longest_depth(&imports[index].target, adjacency, imports, memo))
            .max()
            .unwrap_or(0)
    });
    memo.insert(path.clone(), depth);
    depth
}

fn adjacency(imports: &[ImportEdge]) -> BTreeMap<LogicalPath, Vec<usize>> {
    let mut result = BTreeMap::<LogicalPath, Vec<usize>>::new();
    for (index, edge) in imports.iter().enumerate() {
        result.entry(edge.source.clone()).or_default().push(index);
    }
    result
}

fn read_failure(path: LogicalPath, error: SourceReadError) -> SourceGraphFailure {
    let code = match error {
        SourceReadError::UnavailableOrNotRegular => SourceToolCode::UnavailableOrNotRegular,
        SourceReadError::UnsafeOrAmbiguous => SourceToolCode::UnsafeOrAmbiguousPath,
    };
    tool_failure(code, Some(path), None, None)
}

fn resource_failure(
    resource: ResourceKind,
    logical_path: Option<LogicalPath>,
    span: Option<ByteSpan>,
) -> SourceGraphFailure {
    tool_failure(
        SourceToolCode::ResourceLimitExceeded,
        logical_path,
        span,
        Some(resource),
    )
}

fn tool_failure(
    code: SourceToolCode,
    logical_path: Option<LogicalPath>,
    span: Option<ByteSpan>,
    resource: Option<ResourceKind>,
) -> SourceGraphFailure {
    SourceGraphFailure::Tool(SourceToolFinding {
        code,
        logical_path,
        span,
        resource,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[derive(Default)]
    struct MemoryReader {
        sources: BTreeMap<String, Vec<ReadSource>>,
        reads: Vec<String>,
    }

    impl MemoryReader {
        fn add(&mut self, path: &str, source: &str) {
            self.add_with_identity(path, source, path.as_bytes());
        }

        fn add_with_identity(&mut self, path: &str, source: &str, identity: &[u8]) {
            self.sources
                .entry(path.to_owned())
                .or_default()
                .push(ReadSource {
                    bytes: source.as_bytes().to_vec(),
                    physical_identity: PhysicalIdentity(identity.to_vec()),
                });
        }
    }

    impl SourceReader for MemoryReader {
        fn read(&mut self, logical_path: &LogicalPath) -> Result<ReadSource, SourceReadError> {
            self.reads.push(logical_path.as_str().to_owned());
            let versions = self
                .sources
                .get_mut(logical_path.as_str())
                .ok_or(SourceReadError::UnavailableOrNotRegular)?;
            if versions.len() > 1 {
                Ok(versions.remove(0))
            } else {
                versions
                    .first()
                    .cloned()
                    .ok_or(SourceReadError::UnavailableOrNotRegular)
            }
        }
    }

    fn source(name: &str, imports: &[(&str, &str)]) -> String {
        let imports = imports
            .iter()
            .map(|(path, alias)| format!("import \"{path}\" as {alias};"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("apls \"0.1\";\n{imports}\nspec {name} {{ version \"1\"; }}")
    }

    fn limits() -> SourceLimits {
        SourceLimits {
            source_bytes: usize::MAX,
            graph_bytes: usize::MAX,
            source_count: usize::MAX,
            import_depth: usize::MAX,
            token_count: usize::MAX,
        }
    }

    #[test]
    fn builds_an_immutable_graph_in_entry_then_utf8_byte_order() {
        let mut reader = MemoryReader::default();
        reader.add(
            "entry.apls",
            &source("Entry", &[("b.apls", "B"), ("a.apls", "A")]),
        );
        reader.add("a.apls", &source("A", &[("c.apls", "C")]));
        reader.add("b.apls", &source("B", &[]));
        reader.add("c.apls", &source("C", &[]));

        let bundle = load_source_graph("entry.apls", &mut reader).unwrap();
        let paths = bundle
            .sources
            .iter()
            .map(|source| source.logical_path.as_str())
            .collect::<Vec<_>>();
        assert_eq!(paths, ["entry.apls", "a.apls", "b.apls", "c.apls"]);
        assert_eq!(reader.reads, paths);
        assert!(bundle.token_count > 0);
        let expected_sha256: [u8; 32] = Sha256::digest(&bundle.sources[0].bytes).into();
        assert_eq!(bundle.sources[0].sha256, expected_sha256);
    }

    #[test]
    fn rejects_invalid_paths_and_unavailable_sources_without_guessing_imports() {
        let mut invalid = MemoryReader::default();
        invalid.add("entry.apls", &source("Entry", &[("../x.apls", "X")]));
        let failure = load_source_graph("entry.apls", &mut invalid).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Tool(finding)
                if finding.code == SourceToolCode::UnsafeOrAmbiguousPath
                    && finding.code.as_str() == "APLS-T0003"
        ));

        let mut missing = MemoryReader::default();
        missing.add("entry.apls", &source("Entry", &[("missing.apls", "M")]));
        let failure = load_source_graph("entry.apls", &mut missing).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Tool(finding)
                if finding.code == SourceToolCode::UnavailableOrNotRegular
                    && finding.code.as_str() == "APLS-T0002"
        ));
    }

    #[test]
    fn rejects_cycles_and_duplicate_aliases_with_stable_source_codes() {
        let mut cyclic = MemoryReader::default();
        cyclic.add("entry.apls", &source("Entry", &[("a.apls", "A")]));
        cyclic.add("a.apls", &source("A", &[("entry.apls", "Entry")]));
        let failure = load_source_graph("entry.apls", &mut cyclic).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Source(finding)
                if finding.code == SourceErrorCode::ImportCycle
                    && finding.code.as_str() == "APLS-E2004"
        ));

        let mut duplicate = MemoryReader::default();
        duplicate.add(
            "entry.apls",
            &source("Entry", &[("a.apls", "Same"), ("b.apls", "Same")]),
        );
        duplicate.add("a.apls", &source("A", &[]));
        duplicate.add("b.apls", &source("B", &[]));
        let failure = load_source_graph("entry.apls", &mut duplicate).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Source(finding)
                if finding.code == SourceErrorCode::DuplicateImportAlias
                    && finding.code.as_str() == "APLS-E2002"
        ));
    }

    #[test]
    fn rejects_two_logical_paths_for_one_physical_source() {
        let mut reader = MemoryReader::default();
        reader.add(
            "entry.apls",
            &source("Entry", &[("a.apls", "A"), ("b.apls", "B")]),
        );
        reader.add_with_identity("a.apls", &source("A", &[]), b"same-file");
        reader.add_with_identity("b.apls", &source("B", &[]), b"same-file");
        let failure = load_source_graph("entry.apls", &mut reader).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Tool(finding)
                if finding.code == SourceToolCode::UnsafeOrAmbiguousPath
        ));
    }

    #[test]
    fn recheck_detects_changed_bytes_before_publication() {
        let initial = source("Entry", &[]);
        let mut reader = MemoryReader::default();
        reader.add("entry.apls", &initial);
        reader.add("entry.apls", &format!("{initial}\n"));
        let bundle = load_source_graph("entry.apls", &mut reader).unwrap();
        let failure = bundle.verify_unchanged(&mut reader).unwrap_err();
        assert!(matches!(
            failure,
            SourceGraphFailure::Tool(finding)
                if finding.code == SourceToolCode::SourceChanged
                    && finding.code.as_str() == "APLS-T0004"
        ));
    }

    #[test]
    fn a_parse_failure_never_discovers_imports_from_that_source() {
        let mut reader = MemoryReader::default();
        reader.add(
            "entry.apls",
            "apls \"0.1\"; import \"hidden.apls\" as Hidden; spec Broken { version \"1\";",
        );
        reader.add("hidden.apls", &source("Hidden", &[]));
        let failure = load_source_graph("entry.apls", &mut reader).unwrap_err();
        assert!(matches!(failure, SourceGraphFailure::Parse { .. }));
        assert_eq!(reader.reads, ["entry.apls"]);
    }

    #[test]
    fn source_graph_limits_accept_the_boundary_and_fail_first_excess_as_t0007() {
        let entry = source("Entry", &[]);

        let bounded_entry = source("Entry", &[("a.apls", "A")]);
        let bounded_import = source("A", &[]);
        let mut baseline_reader = MemoryReader::default();
        baseline_reader.add("entry.apls", &bounded_entry);
        baseline_reader.add("a.apls", &bounded_import);
        let baseline =
            load_source_graph_with_limits("entry.apls", &mut baseline_reader, limits()).unwrap();
        let exact_limits = SourceLimits {
            source_bytes: bounded_entry.len().max(bounded_import.len()),
            graph_bytes: bounded_entry.len() + bounded_import.len(),
            source_count: 2,
            import_depth: 1,
            token_count: baseline.token_count,
        };
        let mut exact_reader = MemoryReader::default();
        exact_reader.add("entry.apls", &bounded_entry);
        exact_reader.add("a.apls", &bounded_import);
        load_source_graph_with_limits("entry.apls", &mut exact_reader, exact_limits).unwrap();

        let mut single = MemoryReader::default();
        single.add("entry.apls", &entry);
        let mut configured = limits();
        configured.source_bytes = entry.len() - 1;
        assert_resource(
            load_source_graph_with_limits("entry.apls", &mut single, configured),
            ResourceKind::SourceBytes,
        );

        let mut graph = MemoryReader::default();
        graph.add("entry.apls", &entry);
        configured = limits();
        configured.graph_bytes = entry.len() - 1;
        assert_resource(
            load_source_graph_with_limits("entry.apls", &mut graph, configured),
            ResourceKind::SourceGraphBytes,
        );

        let mut count = MemoryReader::default();
        count.add("entry.apls", &source("Entry", &[("a.apls", "A")]));
        count.add("a.apls", &source("A", &[]));
        configured = limits();
        configured.source_count = 1;
        assert_resource(
            load_source_graph_with_limits("entry.apls", &mut count, configured),
            ResourceKind::SourceCount,
        );

        let mut depth = MemoryReader::default();
        depth.add("entry.apls", &source("Entry", &[("a.apls", "A")]));
        depth.add("a.apls", &source("A", &[("b.apls", "B")]));
        depth.add("b.apls", &source("B", &[]));
        configured = limits();
        configured.import_depth = 1;
        assert_resource(
            load_source_graph_with_limits("entry.apls", &mut depth, configured),
            ResourceKind::ImportDepth,
        );

        let mut tokens = MemoryReader::default();
        tokens.add("entry.apls", &entry);
        configured = limits();
        configured.token_count = 1;
        assert_resource(
            load_source_graph_with_limits("entry.apls", &mut tokens, configured),
            ResourceKind::TokenCount,
        );
    }

    fn assert_resource(result: Result<LoadedBundle, SourceGraphFailure>, resource: ResourceKind) {
        assert!(matches!(
            result,
            Err(SourceGraphFailure::Tool(finding))
                if finding.code == SourceToolCode::ResourceLimitExceeded
                    && finding.resource == Some(resource)
                    && finding.code.as_str() == "APLS-T0007"
        ));
    }
}
