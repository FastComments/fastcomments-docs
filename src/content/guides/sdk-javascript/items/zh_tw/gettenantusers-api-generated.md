## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳：[`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenantUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c1a';
const skip: number = 50;
const firstPage: GetTenantUsers200Response = await getTenantUsers(tenantId);
const nextPage: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
[inline-code-end]

---