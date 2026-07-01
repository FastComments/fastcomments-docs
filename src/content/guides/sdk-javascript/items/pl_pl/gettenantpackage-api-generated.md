## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odpowiedź

Zwraca: [`GetTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenantPackage(): Promise<void> {
    const tenantId: string = "acme-tenant-2024";
    const packageId: string = "premium-package-01";
    const response: GetTenantPackageResponse1 = await getTenantPackage(tenantId, packageId);

    // Opcjonalne pola w odpowiedzi
    const tenantPackage: TenantPackage | undefined = response.tenantPackage;
    const customConfig: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]