## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTenantPackage 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_9b3c2a';
const packageId: string = 'pkg_pro_monthly_2026';
const result: GetTenantPackageResponse = await getTenantPackage(tenantId, packageId);
const tenantPackage: TenantPackage | undefined = (result as unknown as { tenantPackage?: TenantPackage }).tenantPackage;
const status: APIStatus | undefined = (result as unknown as { status?: APIStatus }).status
[inline-code-end]

---