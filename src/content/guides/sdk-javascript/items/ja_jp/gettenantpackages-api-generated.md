## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

返却: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantPackages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPackages() {
  const tenantId: string = "acme-corp-001";
  const skip: number = 15;

  const resultWithSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
  const resultWithoutSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId);
}
[inline-code-end]

---