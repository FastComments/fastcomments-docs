## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTenantPackage Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_9b3c2a';
const packageId: string = 'pkg_pro_monthly_2026';
const result: GetTenantPackageResponse = await getTenantPackage(tenantId, packageId);
const tenantPackage: TenantPackage | undefined = (result as unknown as { tenantPackage?: TenantPackage }).tenantPackage;
const status: APIStatus | undefined = (result as unknown as { status?: APIStatus }).status
[inline-code-end]

---