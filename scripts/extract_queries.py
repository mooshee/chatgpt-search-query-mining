#!/usr/bin/env python3
"""Extract nested ChatGPT `queries` values from copied response data.

Author: Daniel Hallman
"""

from __future__ import annotations

import argparse
import csv
import io
import json
import re
import sys
from collections.abc import Iterable
from pathlib import Path
from typing import Any


QUERY_KEYS = ("q", "query", "search_query", "text")


def parse_documents(raw: str) -> list[Any]:
    """Parse JSON, JSON Lines, SSE data lines, or embedded `queries` values."""
    raw = raw.strip()
    if not raw:
        return []

    try:
        return [json.loads(raw)]
    except json.JSONDecodeError:
        pass

    documents: list[Any] = []
    for line in raw.splitlines():
        candidate = line.strip()
        if candidate.startswith("data:"):
            candidate = candidate[5:].strip()
        if not candidate or candidate == "[DONE]":
            continue
        try:
            documents.append(json.loads(candidate))
        except json.JSONDecodeError:
            continue

    if documents:
        return documents

    decoder = json.JSONDecoder()
    for match in re.finditer(r'["\']queries["\']\s*:', raw, re.IGNORECASE):
        value_start = match.end()
        try:
            value, _ = decoder.raw_decode(raw[value_start:].lstrip())
            documents.append({"queries": value})
        except json.JSONDecodeError:
            continue
    return documents


def strings_from_query_value(value: Any) -> Iterable[str]:
    if isinstance(value, str):
        yield value
        return

    if isinstance(value, list):
        for item in value:
            yield from strings_from_query_value(item)
        return

    if not isinstance(value, dict):
        return

    matched_keys = [key for key in QUERY_KEYS if key in value]
    values = (value[key] for key in matched_keys) if matched_keys else value.values()
    for item in values:
        yield from strings_from_query_value(item)


def find_queries(value: Any) -> Iterable[str]:
    if isinstance(value, list):
        for item in value:
            yield from find_queries(item)
        return

    if not isinstance(value, dict):
        return

    for key, item in value.items():
        if key.casefold() == "queries":
            yield from strings_from_query_value(item)
        else:
            yield from find_queries(item)


def find_direct_query_value(value: Any) -> Iterable[str]:
    """Accept a copied `queries` value without treating arbitrary JSON as queries."""
    if isinstance(value, str):
        yield value
        return
    if isinstance(value, dict) and any(key in value for key in QUERY_KEYS):
        yield from strings_from_query_value(value)
        return
    if isinstance(value, list) and all(
        isinstance(item, str)
        or (isinstance(item, dict) and any(key in item for key in QUERY_KEYS))
        for item in value
    ):
        yield from strings_from_query_value(value)


def normalize_queries(values: Iterable[str]) -> list[str]:
    output: list[str] = []
    seen: set[str] = set()
    for value in values:
        query = " ".join(value.split()).strip()
        if not query or query.startswith(("http://", "https://")):
            continue
        fingerprint = query.casefold()
        if fingerprint in seen:
            continue
        seen.add(fingerprint)
        output.append(query)
    return output


def render(queries: list[str], output_format: str) -> str:
    if output_format == "json":
        return json.dumps(queries, indent=2, ensure_ascii=False) + "\n"
    if output_format == "markdown":
        return "".join(f"- {query}\n" for query in queries)
    if output_format == "csv":
        buffer = io.StringIO()
        writer = csv.writer(buffer)
        writer.writerow(["query"])
        writer.writerows([query] for query in queries)
        return buffer.getvalue()
    return "".join(f"{query}\n" for query in queries)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Extract and deduplicate nested `queries` from ChatGPT response data."
    )
    parser.add_argument("input", nargs="?", default="-", help="Input path or - for stdin")
    parser.add_argument(
        "--format",
        choices=("text", "markdown", "json", "csv"),
        default="text",
        help="Output format (default: text)",
    )
    args = parser.parse_args()

    raw = sys.stdin.read() if args.input == "-" else Path(args.input).read_text(encoding="utf-8")
    documents = parse_documents(raw)
    extracted = [query for document in documents for query in find_queries(document)]
    if not extracted and len(documents) == 1:
        extracted = list(find_direct_query_value(documents[0]))
    queries = normalize_queries(extracted)
    if not queries:
        print(
            "No queries found. Copy a response body that contains a `queries` field.",
            file=sys.stderr,
        )
        return 1

    sys.stdout.write(render(queries, args.format))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
