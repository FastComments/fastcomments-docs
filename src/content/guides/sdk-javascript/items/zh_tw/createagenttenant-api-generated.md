為 AI 代理程式建立新的試用帳戶，無需人工註冊。呼叫此功能不需要 API 金鑰。

回應包含租戶 ID、一個可立即用於 REST API 與 MCP 伺服器的 API 金鑰，以及一個認領 URL。將認領 URL 提供給您所服務的人員：在登入 FastComments 的狀態下開啟該 URL，即可將帳戶附加至該人員。未認領的帳戶及其金鑰將在建立後 72 小時刪除。在認領之前，該帳戶受標準試用限制約束。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | 是 |  |

## 回應

回傳：[`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## 範例

[inline-code-attrs-start title = 'createAgentTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]