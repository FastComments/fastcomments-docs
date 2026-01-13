このスニペットをプロジェクトのAIアシスタント設定ファイル（AGENT.mdやCLAUDE.mdなど）に追加してください。これによりAIコーディングアシスタントがFastCommentsのドキュメントを検索してアクセスする方法を学習できます。

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

これにより、AIアシスタントは最新の情報を簡単に取得し、より正確な実装をより短時間で提供できます。

### 使用例

Claude CodeやCursorなどのAIアシスタントを使用する場合、次のような質問ができます：

- 「ReactにFastCommentsをインストールするには？」
- 「FastCommentsでSSOを設定する方法を見せて」
- 「FastComments APIの認証オプションは何ですか？」

AIアシスタントは提供されたAPIエンドポイントを使用して自動的にドキュメントを検索し、公式ドキュメントに基づいた正確で最新の回答を提供します。
