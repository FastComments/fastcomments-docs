## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## レスポンス

返却: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const skip: number = 20;

const firstPage: GetTenantUsersResponse = await getTenantUsers(tenantId);
const secondPage: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
[inline-code-end]

---