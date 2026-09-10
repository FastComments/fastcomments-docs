FastComments はホストされた Model Context Protocol (MCP) サーバーを実行し、AI アシスタントやエージェントクライアントが FastComments API を直接呼び出すことができます。MCP サーバーが公開するすべてのツールは公開 OpenAPI 仕様から自動生成されるため、REST API ができることはすべて MCP クライアントでも実行できます。

エンドポイントはステートレスでストリーム可能な HTTP ベースです。セッションを維持する必要はなく、クライアントごとのサーバー側状態もありません。

### Endpoint

[inline-code-attrs-start title = 'MCP エンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### OAuth で接続

OAuth（Claude、ChatGPT、Claude Code、Cursor など）をサポートするリモートサーバー対応の MCP クライアントは、FastComments 側での設定なしに上記エンドポイントに接続できます。クライアントは動的クライアント登録を通じて自身を登録するか、クライアント ID メタデータドキュメントで識別し、ブラウザを開いて FastComments にサインインしアクセスを承認すると、サインインしたアカウントに紐付いたトークンを受け取ります。

ディスカバリードキュメントは標準の場所にあります：

[inline-code-attrs-start title = 'ディスカバリー'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

接続を承認するには、ユーザーがアカウント上で API 管理者権限を持っている必要があります。複数のアカウントを管理している場合は、承認する前にダッシュボードで対象のアカウントに切り替えてください。

クライアントは `read` スコープ、`write` スコープ、またはその両方を要求できます。何も要求しないクライアントは両方を取得します。データを変更するツールは読み取り専用トークンには提供されません。

ダッシュボードには貼り付け可能なスニペットが用意されたセットアップヘルパーがあります。**Integrate -> MCP Server** を開くか、直接以下にアクセスしてください：

[inline-code-attrs-start title = 'セットアップページ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

1 つのコマンドで FastComments サーバーを登録し、セッション内で `/mcp` を実行してサインインし、利用可能なツールを一覧表示します：

[inline-code-attrs-start title = 'Claude Code 設定'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor とその他の設定ファイルクライアント

このブロックをクライアントの MCP サーバー設定（Cursor の場合は `mcp.json`）に追加します。クライアントは初回使用時にブラウザを開いてサインインします。

[inline-code-attrs-start title = 'MCP クライアント設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### アクセスの取り消し

承認されたすべての接続はダッシュボードの **Integrate -> Connected Apps** に一覧表示されます。1 つを取り消すと、そのアプリケーションが保持するすべてのトークンが無効になります。アプリケーションは接続時に自ら登録され、FastComments はそれらを審査しないため、認識できないものはすべて取り消してください。

### REST API でトークンを使用する

MCP クライアントが取得するアクセストークンは通常の FastComments API 資格情報です。`/api/v1` のすべてのエンドポイントでベアラートークンとして機能するため、MCP 経由で接続したアプリケーションは REST API を直接呼び出すこともできます：

[inline-code-attrs-start title = 'ベアラートークン'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

テナントはトークンに暗黙的に含まれます。`tenantId` を渡すこともできますが、一致している必要があります。`GET` リクエストは `read` スコープが必要で、その他の操作は `write` スコープが必要です。

### API キーで接続

ヘッドレスサーバーなど、ブラウザでのサインインができないクライアントは、代わりに API キーで認証できます。`tenantId` と `API_KEY` をクエリパラメータとして渡すか、クライアントがカスタムヘッダーをサポートしている場合は `x-tenant-id` と `x-api-key` HTTP ヘッダーとして渡してください：

[inline-code-attrs-start title = 'API キーエンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

セットアップページは各 API キーごとにこの URL を生成します。

### セキュリティ

API キーを含むエンドポイント URL は機密情報です：公開チャットやスクリーンショット、コミットに貼り付けないでください。キーが漏洩した場合は、ダッシュボードの API キーページでローテーションしてください。OAuth トークンは 1 つのアプリケーションに紐付いており、Connected Apps から取り消すことができるため、このようなリスクはありません。