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
const tenantId: string = 'tenant_8f3b2c1a';
const skip: number = 50;
const firstPage: GetTenantUsers200Response = await getTenantUsers(tenantId);
const nextPage: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
[inline-code-end]

---