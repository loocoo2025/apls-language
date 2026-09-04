#!/usr/bin/env python3
"""Mechanical validation for upstream requirement traceability.

Checks:
- Node coverage for SYS/NFR/IF targets.
- Edge consistency between detailed Traces-From metadata and the
  formal upstream matrix in requirements_traceability.md.

No third-party dependencies.
"""
from __future__ import annotations

from collections import Counter
from pathlib import Path
import re
import sys

ID_RE = re.compile(r"\b(?:PRD|SCN|CON|AC|SYS|NFR|IF)-[A-Z0-9][A-Z0-9-]*\b")
SOURCE_PREFIXES = ("PRD-", "SCN-", "CON-", "AC-")
TARGET_PREFIXES = ("SYS-", "NFR-", "IF-")
TRACE_LABEL_RE = re.compile(r"^\s*-\s*(?:Traces-From(?:（[^）]*）)?|来源|对应需求)\s*[：:]\s*(.*)$", re.I)
HEADING_RE = re.compile(r"^##\s+((?:SYS|NFR|IF)-[A-Z0-9][A-Z0-9-]*)\b")

def ids(text: str):
    return ID_RE.findall(text)

def is_source(x: str):
    return x.startswith(SOURCE_PREFIXES)

def is_target(x: str):
    return x.startswith(TARGET_PREFIXES)

def split_blocks(text: str):
    lines = text.splitlines()
    blocks=[]
    current_id=None; current=[]
    for line in lines:
        m=HEADING_RE.match(line)
        if m:
            if current_id is not None:
                blocks.append((current_id, current))
            current_id=m.group(1); current=[]
        elif current_id is not None:
            current.append(line)
    if current_id is not None:
        blocks.append((current_id,current))
    return blocks

def block_metadata_edges(text: str):
    edges=[]
    noncanonical=[]
    targets=set()
    for target, lines in split_blocks(text):
        targets.add(target)
        i=0
        while i < len(lines):
            m=TRACE_LABEL_RE.match(lines[i])
            if not m:
                i += 1; continue
            label=lines[i]
            if "Traces-From" not in label:
                noncanonical.append((target, label.strip()))
            values=list(ids(m.group(1)))
            j=i+1
            while j < len(lines):
                s=lines[j]
                if re.match(r"^\s*-\s+[^-].*[：:]", s) and not re.match(r"^\s{2,}-\s+", s):
                    break
                if s.startswith("## ") or s.startswith("# "):
                    break
                if re.match(r"^\s{2,}-\s+", s) or (s.strip().startswith("-") and not re.match(r"^\s*-\s+[^-].*[：:]", s)):
                    values.extend(ids(s))
                    j += 1
                    continue
                if not s.strip():
                    j += 1; continue
                break
            for source in values:
                if is_source(source):
                    edges.append((source,target))
            i=max(j,i+1)
    return targets, edges, noncanonical

def parse_markdown_table(lines):
    rows=[]
    for line in lines:
        if not line.lstrip().startswith('|'):
            continue
        cells=[c.strip() for c in line.strip().strip('|').split('|')]
        if cells and not all(re.fullmatch(r":?-{3,}:?", c or "") for c in cells):
            rows.append(cells)
    return rows

def nfr_table_metadata(text: str):
    lines=text.splitlines(); rows=parse_markdown_table(lines)
    targets=set(); edges=[]
    if not rows: return targets, edges
    header=None; trace_idx=None; id_idx=None
    for row in rows:
        if any('Traces-From' in c for c in row) and any(c=='ID' for c in row):
            header=row; id_idx=row.index('ID'); trace_idx=next(i for i,c in enumerate(row) if 'Traces-From' in c); continue
        if header is None: continue
        if max(id_idx,trace_idx) >= len(row): continue
        target_ids=[x for x in ids(row[id_idx]) if x.startswith('NFR-')]
        if not target_ids: continue
        target=target_ids[0]; targets.add(target)
        for source in ids(row[trace_idx]):
            if is_source(source): edges.append((source,target))
    return targets, edges

def declared_product_ids(project_root: Path):
    result=set()
    for p in (project_root/'01_product_requirements').glob('*.md'):
        result.update(x for x in ids(p.read_text(encoding='utf-8')) if is_source(x))
    return result

def matrix_edges(text: str):
    edges=[]
    for cells in parse_markdown_table(text.splitlines()):
        row=' | '.join(cells)
        relation_tokens=[c.strip().upper() for c in cells]
        # Explicit non-FORMAL relation rows are outside the default gate.
        explicit_rel=[c for c in relation_tokens if c in {'FORMAL_TRACE','SUPPORTS','ALLOCATES_TO','CONSTRAINS'}]
        if explicit_rel and 'FORMAL_TRACE' not in explicit_rel:
            continue
        sources=[x for x in ids(row) if is_source(x)]
        targets=[x for x in ids(row) if is_target(x)]
        for s in sources:
            for t in targets:
                edges.append((s,t))
    return edges

def fmt_edges(edges):
    return [f"{s} -> {t}" for s,t in sorted(edges)]

def main():
    script=Path(__file__).resolve()
    root=script.parents[2]
    req=root/'02_system_requirements'

    f_text=(req/'functional_requirements.md').read_text(encoding='utf-8')
    n_text=(req/'nonfunctional_requirements.md').read_text(encoding='utf-8')
    i_text=(req/'interface_requirements.md').read_text(encoding='utf-8')
    m_text=(req/'requirements_traceability.md').read_text(encoding='utf-8')

    sys_targets, sys_edges, sys_alias=block_metadata_edges(f_text)
    if_targets, if_edges, if_alias=block_metadata_edges(i_text)
    nfr_block_targets, nfr_block_edges, nfr_alias=block_metadata_edges(n_text)
    nfr_table_targets, nfr_table_edges=nfr_table_metadata(n_text)

    expected_targets=sys_targets | if_targets | nfr_block_targets | nfr_table_targets
    metadata_list=sys_edges + if_edges + nfr_block_edges + nfr_table_edges
    matrix_list=matrix_edges(m_text)

    metadata_count=Counter(metadata_list)
    matrix_count=Counter(matrix_list)
    metadata=set(metadata_list)
    matrix=set(matrix_list)

    covered={t for _,t in matrix}
    missing=expected_targets-covered
    unexpected=covered-expected_targets
    detailed_only=metadata-matrix
    matrix_only=matrix-metadata

    product_ids=declared_product_ids(root)
    all_sources={s for s,_ in metadata|matrix}
    invalid_sources=all_sources-product_ids
    duplicate_metadata=sum(v-1 for v in metadata_count.values() if v>1)
    duplicate_matrix=sum(v-1 for v in matrix_count.values() if v>1)
    aliases=sys_alias+if_alias+nfr_alias

    print('TRACEABILITY VALIDATION')
    print()
    print('NODE CHECK')
    print(f'Expected IDs: {len(expected_targets)}')
    print(f'Covered IDs: {len(covered & expected_targets)}')
    print(f'Missing IDs: {len(missing)}')
    print(f'Unexpected IDs: {len(unexpected)}')
    print()
    print('EDGE CHECK')
    print(f'Detailed Metadata Edges: {len(metadata)}')
    print(f'Traceability Matrix Edges: {len(matrix)}')
    print(f'Intersection: {len(metadata & matrix)}')
    print(f'Detailed-only: {len(detailed_only)}')
    print(f'Matrix-only: {len(matrix_only)}')
    print(f'Duplicate Metadata Edges: {duplicate_metadata}')
    print(f'Duplicate Matrix Edges: {duplicate_matrix}')
    print(f'Invalid Source IDs: {len(invalid_sources)}')
    print()

    def section(title, items):
        if items:
            print(title)
            for item in items: print(item)
            print()
    section('MISSING IDS', sorted(missing))
    section('UNEXPECTED IDS', sorted(unexpected))
    section('DETAILED_ONLY', fmt_edges(detailed_only))
    section('MATRIX_ONLY', fmt_edges(matrix_only))
    section('INVALID SOURCE IDS', sorted(invalid_sources))
    if aliases:
        print('WARNING: NON-CANONICAL TRACE LABELS')
        for target,label in aliases:
            print(f'{target}: {label}')
        print('Prefer canonical Traces-From metadata.')
        print()

    ok=(not missing and not unexpected and not detailed_only and not matrix_only
        and duplicate_metadata==0 and duplicate_matrix==0 and not invalid_sources)
    print('RESULT: PASS' if ok else 'RESULT: FAIL')
    return 0 if ok else 1

if __name__=='__main__':
    sys.exit(main())
