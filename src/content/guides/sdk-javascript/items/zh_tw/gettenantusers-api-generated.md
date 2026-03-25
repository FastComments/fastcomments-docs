## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## 回應

Returns: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenantUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8a3f2c';
const skip: number = 50;
const usersWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const usersNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]

---