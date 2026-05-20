---
FastComments 運行一個託管的 模型上下文協定 (MCP) 伺服器，讓 AI 程式碼助理與自主代理客戶端可以直接呼叫 FastComments API。MCP 伺服器所暴露的每個工具皆由公開的 OpenAPI 規格自動產生，因此 REST API 能做到的，MCP 客戶端也能做到。

該端點為無狀態且基於可串流的 HTTP。無需維持任何會話，無需客戶端註冊步驟，且伺服器端不會為每個客戶端保存狀態。

### 端點

[inline-code-attrs-start title = 'MCP 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

驗證使用與 REST API 相同的 API 金鑰。若您的客戶端支援自訂標頭，也可以將 `tenantId` 與該金鑰作為 `x-tenant-id` 和 `x-api-key` HTTP 標頭傳遞。

### 預先填好的設定

儀表板有一個設定助手，可為常見的 MCP 客戶端產生 URL 與可直接貼上的設定片段。前往您的帳戶儀表板並開啟 **Integrate -> MCP Server**，或直接造訪：

[inline-code-attrs-start title = '設定頁面'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

從下拉選單選擇要使用的 API 金鑰，然後複製任一產生的片段。

### Claude Code

Register the FastComments server with one command:

[inline-code-attrs-start title = 'Claude Code 設定'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

註冊後，在 Claude Code 會話中執行 `/mcp` 以確認連線並列出可用工具。

### Claude Desktop / Cursor

Add this block to your client's MCP servers config (`claude_desktop_config.json` for Claude Desktop, `mcp.json` for Cursor):

[inline-code-attrs-start title = 'MCP 客戶端設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 安全性

API 金鑰已嵌入 URL。請將該 URL 當作機密：不要將它貼到公開聊天、截圖或提交記錄。如果金鑰外洩，請在儀表板的 API 金鑰頁面上重置它。

---