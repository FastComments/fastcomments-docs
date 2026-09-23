Creates a new trial account for an AI agent without a human signup. No API key is needed to call this.

為 AI 代理建立一個新的試用帳戶，無需人工註冊。呼叫此功能不需要 API 金鑰。

The response contains the tenant id, an API key that works immediately against the REST API and the
MCP server, and a claim URL. Give the claim URL to the human you are working for: opening it while
logged in to FastComments attaches the account to them. Unclaimed accounts, and their keys, are
deleted 72 hours after creation. Until claimed, the account has the standard trial limits.

回應中包含租戶 ID、可立即用於 REST API 與 MCP 伺服器的 API 金鑰，以及領取 URL。將領取 URL 提供給您所協助的人員：在登入 FastComments 後開啟該 URL，即可將帳戶與其關聯。未領取的帳戶及其金鑰會在建立後 72 小時內被刪除。未領取前，帳戶仍具備標準的試用限制。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

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