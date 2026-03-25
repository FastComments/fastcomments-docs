---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 回應

Returns: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f4b2c1a";
const idOverride: string | undefined = undefined; // 可選的覆寫（若可用）
const id: string = idOverride ?? "site_3e7a6b2f";
const response: GetTenant200Response = await getTenant(tenantId, id);
console.log(response);
[inline-code-end]

---