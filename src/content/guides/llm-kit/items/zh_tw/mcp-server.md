FastComments 運行一個託管的模型上下文協議 (MCP) 伺服器，使 AI 助手和代理客戶端能直接呼叫 FastComments API。MCP 伺服器所公開的每個工具皆由公共 OpenAPI 規範自動生成，因此 REST API 能做到的事，MCP 客戶端也能做到。

此端點是無狀態且基於可串流的 HTTP。沒有需要保持的會話，也沒有每個客戶端的伺服器端狀態。

### 端點

[inline-code-attrs-start title = 'MCP 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### 使用 OAuth 連接

任何支援 OAuth 的遠端伺服器的 MCP 客戶端（如 Claude、ChatGPT、Claude Code、Cursor 等）都可以在 FastComments 端不需任何設定即可連接上述端點。客戶端透過動態客戶端註冊或使用客戶端 ID 中繼資料文件進行註冊，開啟瀏覽器讓您登入 FastComments 並批准存取，然後收到綁定於您登入帳號的令牌。

發現文件位於標準位置：

[inline-code-attrs-start title = '發現'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

您的使用者需要在帳號上具備 API 管理員權限才能批准連接。如果您管理多個帳號，請在儀表板中切換至正確的帳號後再批准。

客戶端可以請求 `read` 範圍、`write` 範圍，或兩者皆請求。未請求任何範圍的客戶端會同時取得兩者。會變更資料的工具不會提供給唯讀令牌。

儀表板提供設定輔助工具與可直接貼上的程式碼片段。開啟 **Integrate -> MCP Server**，或直接前往：

[inline-code-attrs-start title = '設定頁面'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

使用一條指令註冊 FastComments 伺服器，然後在會話中執行 `/mcp` 以登入並列出可用的工具：

[inline-code-attrs-start title = 'Claude Code 設定'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor 與其他設定檔客戶端

將此區塊加入客戶端的 MCP 伺服器設定（Cursor 使用 `mcp.json`）。客戶端在首次使用時會開啟瀏覽器進行登入。

[inline-code-attrs-start title = 'MCP 客戶端設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 撤銷存取

儀表板中 **Integrate -> Connected Apps** 會列出所有已批准的連接。撤銷其中一個會使該應用持有的所有令牌失效。應用在連接時自行註冊，FastComments 並不審核它們，因此請撤銷任何您不認識的應用。

### 使用令牌呼叫 REST API

MCP 客戶端取得的存取令牌即為一般的 FastComments API 憑證。它可作為 Bearer 令牌在所有 `/api/v1` 端點使用，因此透過 MCP 連接的應用也能直接呼叫 REST API：

[inline-code-attrs-start title = 'Bearer 令牌'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

租戶資訊由令牌隱含。仍可傳遞 `tenantId`，但必須相符。`GET` 請求需要 `read` 範圍，其他請求則需要 `write` 範圍。

### 使用 API 金鑰連接

無法完成瀏覽器登入的客戶端（例如無頭伺服器），可改以 API 金鑰驗證。將 `tenantId` 與 `API_KEY` 作為查詢參數傳遞，或在客戶端支援自訂標頭時使用 `x-tenant-id` 與 `x-api-key` HTTP 標頭：

[inline-code-attrs-start title = 'API 金鑰端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

設定頁面會為您的每組 API 金鑰產生此 URL。

### 安全性

包含 API 金鑰的端點 URL 為機密資訊：請勿貼到公開聊天、截圖或提交中。若金鑰外洩，請在儀表板的 API 金鑰頁面重新產生。OAuth 令牌則不會有此風險，因為它們綁定於單一應用，且可於 Connected Apps 中撤銷。

---