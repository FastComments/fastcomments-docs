## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenantPackage(): Promise<void> {
    const tenantId: string = "acme-tenant-2024";
    const packageId: string = "premium-package-01";
    const response: GetTenantPackageResponse1 = await getTenantPackage(tenantId, packageId);

    // Optionele velden in de respons
    const tenantPackage: TenantPackage | undefined = response.tenantPackage;
    const customConfig: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]

---