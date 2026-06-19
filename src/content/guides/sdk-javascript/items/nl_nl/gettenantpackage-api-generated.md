## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_9b3c2a';
const packageId: string = 'pkg_pro_monthly_2026';
const result: GetTenantPackageResponse = await getTenantPackage(tenantId, packageId);
const tenantPackage: TenantPackage | undefined = (result as unknown as { tenantPackage?: TenantPackage }).tenantPackage;
const status: APIStatus | undefined = (result as unknown as { status?: APIStatus }).status
[inline-code-end]