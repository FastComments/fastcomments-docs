Add this snippet to your project's AI assistant configuration files (like AGENT.md or CLAUDE.md) that teaches AI coding assistants how to search and access FastComments documentation.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

This way your AI assistant can easily get up-to-date information and provide more accurate implementations in less time.

### Usage Example

When using an AI assistant like Claude Code or Cursor, you can ask questions like:

- "How do I install FastComments on React?"
- "Show me how to configure SSO with FastComments"
- "What are the FastComments API authentication options?"

The AI assistant will automatically search the documentation using the provided API endpoint and give you accurate, up-to-date answers based on the official documentation.
