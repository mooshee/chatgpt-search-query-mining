# ChatGPT Query Research: Turn Search Queries Into a Content Plan

Copy response data from a ChatGPT conversation you control. This open Agent Skill from [Moosh Works](https://mooshworks.com/skills/) extracts the web-search queries, removes duplicates, groups related searches by reader intent, and turns the result into a source-backed content plan.

The query list shows how ChatGPT broke down one question. It does not show search volume, ranking difficulty, or market demand, so use it to find content angles and then check them against current search results and primary sources.

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

## Need the Demo Skill?

[Scripted Product Demo](https://github.com/mooshee/product-demo) records one product walkthrough and renders it with camera moves, cursor motion, click feedback, privacy masks, and 6 supported layouts. See both install commands on the [Moosh Works skills page](https://mooshworks.com/skills/).

## Author & License

Built by [Daniel Hallman](https://github.com/mooshee) at [Moosh Works](https://mooshworks.com/).

MIT licensed.
