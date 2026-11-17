## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳：[`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTenantPackages 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5ad8b1e4-8c1b-4f2a-9d3e-7f6a1b2c3d4e';

// 取得租戶方案的第一頁
const response: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const firstPackage: TenantPackage | undefined = response.packages?.[0];
if (firstPackage?.id) {
  console.log(`First package: ${firstPackage.id} - active: ${firstPackage.active}`);
}

// 使用 skip 取得下一頁
const skip: number = 10;
const nextPage: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
const status: APIStatus = nextPage.status;
console.log(`Fetched ${nextPage.packages?.length ?? 0} packages, status: ${status}`);
[inline-code-end]