## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_6b3e2a';
const id: string = 'site_42f1';
const tenantResponse: GetTenant200Response = await getTenant(tenantId, id);
const tenant: APITenant | undefined = tenantResponse.tenant;
const primaryDomain: APIDomainConfiguration | undefined = tenant?.domainConfiguration?.[0];
[inline-code-end]

---