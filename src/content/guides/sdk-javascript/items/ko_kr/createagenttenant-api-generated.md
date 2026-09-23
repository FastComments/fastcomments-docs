Creates a new trial account for an AI agent without a human signup. No API key is needed to call this.

The response contains the tenant id, an API key that works immediately against the REST API and the
MCP server, and a claim URL. Give the claim URL to the human you are working for: opening it while
logged in to FastComments attaches the account to them. Unclaimed accounts, and their keys, are
deleted 72 hours after creation. Until claimed, the account has the standard trial limits.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | 예 |  |

## 응답

반환: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## 예시

[inline-code-attrs-start title = 'createAgentTenant 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // 선택 사항
  planId: 3 // 선택 사항
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]