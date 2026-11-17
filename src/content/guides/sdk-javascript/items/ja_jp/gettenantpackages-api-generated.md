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
const tenantId: string = '5ad8b1e4-8c1b-4f2a-9d3e-7f6a1b2c3d4e';

// テナントパッケージの最初のページを取得
const response: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const firstPackage: TenantPackage | undefined = response.packages?.[0];
if (firstPackage?.id) {
  console.log(`First package: ${firstPackage.id} - active: ${firstPackage.active}`);
}

// skip を使用して次のページを取得
const skip: number = 10;
const nextPage: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
const status: APIStatus = nextPage.status;
console.log(`Fetched ${nextPage.packages?.length ?? 0} packages, status: ${status}`);
[inline-code-end]