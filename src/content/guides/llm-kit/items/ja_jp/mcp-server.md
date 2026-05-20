FastComments はホストされた Model Context Protocol (MCP) サーバーを運用しており、AI コーディングアシスタントやエージェント型クライアントが FastComments API を直接呼び出せるようにします。MCP サーバーが公開するすべてのツールは公開された OpenAPI スペックから自動生成されるため、REST API ができることは MCP クライアントでもすべて実行できます。

このエンドポイントはステートレスで、ストリーミング可能な HTTP ベースです。セッションを維持する必要はなく、クライアントの登録手順もなく、クライアントごとのサーバー側の状態もありません。

### エンドポイント

[inline-code-attrs-start title = 'MCP エンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

認証は REST API と同じ API キーを使用します。クライアントがカスタムヘッダをサポートしている場合は、`tenantId` とキーを HTTP ヘッダ `x-tenant-id` および `x-api-key` として渡すこともできます。

### 事前設定

ダッシュボードには、人気のある MCP クライアント向けに URL とそのまま貼り付け可能な設定スニペットを生成するセットアップヘルパーがあります。アカウントのダッシュボードで **Integrate -> MCP Server** を開くか、直接以下にアクセスしてください：

[inline-code-attrs-start title = 'セットアップ ページ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

ドロップダウンから使用する API キーを選択し、生成されたスニペットのいずれかをコピーしてください。

### Claude Code

次のコマンドで FastComments サーバーを登録します：

[inline-code-attrs-start title = 'Claude Code セットアップ'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

登録が完了したら、Claude Code セッション内で /mcp を実行して接続を確認し、利用可能なツールを一覧表示してください。

### Claude Desktop / Cursor

このブロックをクライアントの MCP サーバー設定（Claude Desktop の場合は `claude_desktop_config.json`、Cursor の場合は `mcp.json`）に追加してください：

[inline-code-attrs-start title = 'MCP クライアント設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### セキュリティ

API キーは URL に埋め込まれています。URL は機密情報として扱ってください：公開チャット、スクリーンショット、コミットに貼り付けないでください。もしキーが漏洩した場合は、ダッシュボードの API Keys ページでキーをローテーションしてください。

---