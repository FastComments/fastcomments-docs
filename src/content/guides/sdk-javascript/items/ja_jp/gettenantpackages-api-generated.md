## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

返却値: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## 例

[inline-code-attrs-start title = 'getTenantPackages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-7b3c2f';
const skipCount: number = 10;
const packages: GetTenantPackages200Response = await getTenantPackages(tenantId, skipCount);
const packagesFromStart: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]

---