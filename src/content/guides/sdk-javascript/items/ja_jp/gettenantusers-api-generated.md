## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## 例

[inline-code-attrs-start title = 'getTenantUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8a3f2c';
const skip: number = 50;
const usersWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const usersNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]

---