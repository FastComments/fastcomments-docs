將此程式碼片段添加到專案的AI助手配置檔案（如AGENT.md或CLAUDE.md）中，教AI程式設計助手如何搜尋和存取FastComments文件。

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

這樣您的AI助手可以輕鬆獲取最新資訊，並在更短的時間內提供更準確的實作。

### 使用範例

使用Claude Code或Cursor等AI助手時，您可以提出以下問題：

- "如何在React上安裝FastComments？"
- "向我展示如何配置FastComments的SSO"
- "FastComments API有哪些身份驗證選項？"

AI助手將自動使用提供的API端點搜尋文件，並根據官方文件為您提供準確、最新的答案。
