将此代码片段添加到项目的AI助手配置文件（如AGENT.md或CLAUDE.md）中，教AI编程助手如何搜索和访问FastComments文档。

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

这样您的AI助手可以轻松获取最新信息，并在更短的时间内提供更准确的实现。

### 使用示例

使用Claude Code或Cursor等AI助手时，您可以提出以下问题：

- "如何在React上安装FastComments？"
- "向我展示如何配置FastComments的SSO"
- "FastComments API有哪些身份验证选项？"

AI助手将自动使用提供的API端点搜索文档，并根据官方文档为您提供准确、最新的答案。
