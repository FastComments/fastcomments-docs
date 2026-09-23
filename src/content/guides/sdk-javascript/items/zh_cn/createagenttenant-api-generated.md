创建一个新的 AI 代理的试用账户，无需人工注册。调用此接口不需要 API 密钥。

响应中包含租户 ID、可立即用于 REST API 和 MCP 服务器的 API 密钥，以及认领 URL。将该认领 URL 提供给您为之工作的人员：在登录 FastComments 的情况下打开它即可将账户关联到他们。未认领的账户及其密钥将在创建后 72 小时内被删除。在认领之前，账户拥有标准的试用限制。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## 响应

返回：[`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## 示例

[inline-code-attrs-start title = 'createAgentTenant 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // 可选
  planId: 3 // 可选
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]