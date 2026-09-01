---
name: chatgpt-query-research
description: Extract a web-search query cluster from response data copied from a ChatGPT conversation the user controls, then turn it into a deduplicated, source-backed content plan. Use when a user wants to inspect a ChatGPT network response for `queries`, cluster the resulting research trail, or create useful niche content from it.
---

# ChatGPT Query Research

Use ChatGPT's own research trail as topic-discovery input. The extracted queries show how ChatGPT broke down one question; they do not prove keyword volume, ranking difficulty, or demand. This workflow is experimental because the capture step depends on an internal response shape that may change.

## Capture the query cluster

1. Choose one important question for the niche, audience, offer, and market. Submit it to ChatGPT in a mode that searches the web, and wait for the answer to finish.
2. Work only in a ChatGPT conversation the user owns or may inspect. Copy the conversation ID from the URL segment after `/c/`.
3. Open the browser's developer tools and select **Network**. Paste the conversation ID into the request filter, then reload the page.
4. Inspect the matching Fetch/XHR responses. Find a response whose **Response** or **Preview** data contains a `queries` field and the expected search terms. Browser colors and icons change, so identify the response by its content rather than an orange-bracket icon.
5. Copy the relevant response body or `queries` value. Do not export or share request headers, cookies, authorization values, or a full HAR. Remove unrelated conversation text and personal data before sending the payload to another service.

If no `queries` field appears, confirm that ChatGPT actually searched the web, keep the Network panel open before reloading, clear the conversation-ID filter, and search Fetch/XHR response bodies for `queries`. Stop after those checks; the internal response shape may have changed.

## Extract and assess

For a copied JSON, JSONL, or SSE response, run:

```bash
python3 scripts/extract_queries.py response.txt --format markdown
```

Use `-` to read from standard input. The script finds nested `queries` fields, extracts common query shapes, preserves first-seen order, and removes duplicates.

Before drafting:

- Group near-duplicates by search intent, not just wording.
- Keep queries that match the user's audience, offer, region, and subject expertise.
- Separate informational, comparison, commercial, and action-oriented intent.
- Check current search results and authoritative sources. Treat the extracted cluster as an idea source, not keyword research or factual evidence.
- Combine queries that one strong page can answer. Do not create one thin page per query.

## Produce the content plan

Return a table with these fields:

| Field | Purpose |
|---|---|
| Cluster | A distinct reader need, named in plain language |
| Source queries | The deduplicated queries that support the cluster |
| Search intent | What the reader is trying to learn or do |
| Recommended asset | Article, guide, comparison, FAQ, landing page, or no new page |
| Angle | The useful point of view or first-hand value the publisher can add |
| Evidence needed | Current primary sources, data, examples, or expert input needed before drafting |
| Internal destination | The relevant product, service, or pillar page, when one exists |

Ask for the user's editorial approval before bulk drafting or publishing. When drafting is requested, write original, source-backed pieces that satisfy the clustered intent; do not send every raw query to a writing system unchanged. Do not publish, schedule, or submit content without the user's authorization.

If the user hands the plan to a separate writing or publishing workflow, send only the approved clusters and briefs in a format that workflow accepts. Keep the raw ChatGPT network response local; downstream systems do not need it.
