# ChatGPT Query Research

> **Experimental:** This skill reads copied response data from a ChatGPT session you control. ChatGPT response formats are internal implementation details and may change without notice.

An open Agent Skill from [Moosh Works](https://mooshworks.com/skills/) for extracting a web-search query cluster from copied ChatGPT response data, then turning that cluster into a useful, source-backed content plan.

The query-mining workflow treats the query list as a research artifact. It shows how ChatGPT broke down one question; it does not prove search volume, ranking difficulty, or demand. The skill deduplicates and groups the queries before any drafting so one strong page can cover one reader need.

## Install

Install globally with the open `skills` CLI:

```bash
npx skills add mooshee/chatgpt-search-query-mining --skill chatgpt-query-research -g
```

The installable payload lives in `skills/chatgpt-query-research/`, so the CLI includes the parser rather than only the instruction file.

Restart the host application if the skill is not discovered in the current session.

## Use

Invoke the skill with a captured ChatGPT response or ask it to guide the capture:

```text
Use $chatgpt-query-research to extract this ChatGPT search-query cluster and turn it into a source-backed content plan.
```

From a repository checkout, the included parser accepts JSON, JSON Lines, and SSE response data:

```bash
python3 skills/chatgpt-query-research/scripts/extract_queries.py response.txt --format markdown
```

Only copy response data from a ChatGPT session you may inspect. Never share cookies, authorization values, request headers, or a full HAR with a content service.

If you use a separate writing or publishing workflow, give it the approved query clusters and briefs rather than the raw ChatGPT response.

## More Open Skills

Explore the [Moosh Works Open Skills catalog](https://mooshworks.com/skills/) or try [Scripted Product Demo](https://github.com/mooshee/product-demo), a telemetry-driven skill for recording and rendering polished real-product walkthroughs.

## Author & License

Built by [Daniel Hallman](https://github.com/mooshee) at [Moosh Works](https://mooshworks.com/).

MIT licensed.
