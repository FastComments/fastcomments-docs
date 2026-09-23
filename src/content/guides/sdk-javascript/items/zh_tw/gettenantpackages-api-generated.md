## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTenantPackages 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPackages() {
  const tenantId: string = "acme-corp-001";
  const skip: number = 15;

  const resultWithSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
  const resultWithoutSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId);
}
[inline-code-end]

---