## 參數

| 名稱 | Type | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 是 |  |

## 回應

回傳：[`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## 範例

[inline-code-attrs-start title = 'createTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  domainConfiguration: { primaryDomain: 'comments.acme.com', enforceHttps: true } as APIDomainConfiguration,
  billingInfo: { planId: 'enterprise', contactEmail: 'billing@acme.com' } as BillingInfo
  // 可選欄位（例如 ssoConfig 或 customConfig）故意省略
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---