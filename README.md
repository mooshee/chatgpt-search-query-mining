# ChatGPT Search Query Mining

A Codex skill for recovering the background web-search query cluster from a ChatGPT conversation you control, then turning that cluster into a useful, source-backed content plan.

The query list is a research artifact. It shows how ChatGPT broke down one question; it does not prove search volume, ranking difficulty, or demand. The skill deduplicates and groups the queries before any drafting so one good page can cover one reader need.

## Install

Clone the repository into your Codex skills directory:

```bash
git clone https://github.com/mooshee/chatgpt-search-query-mining.git \
  ~/.codex/skills/chatgpt-search-query-mining
```

Restart Codex if the skill is not discovered in the current session.

## Use

Invoke the skill with a captured ChatGPT response or ask it to guide the capture:

```text
Use $chatgpt-search-query-mining to extract this ChatGPT search-query cluster and turn it into a source-backed content plan.
```

The included parser accepts JSON, JSON Lines, and SSE response data:

```bash
python3 scripts/extract_queries.py response.txt --format markdown
```

Only copy response data from a ChatGPT session you may inspect. Never share cookies, authorization values, request headers, or a full HAR with a content service.

If you use RankDesk or another writing tool, give it the approved query clusters and briefs rather than the raw ChatGPT response.

## License

MIT
