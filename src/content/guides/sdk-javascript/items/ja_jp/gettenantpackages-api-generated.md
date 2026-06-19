---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantPackages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-9f3b";
const packagesPage1: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const packagesPage2: GetTenantPackagesResponse = await getTenantPackages(tenantId, 10);
[inline-code-end]

---