## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenantPackage(): Promise<void> {
    const tenantId: string = "acme-tenant-2024";
    const packageId: string = "premium-package-01";
    const response: GetTenantPackageResponse1 = await getTenantPackage(tenantId, packageId);

    // Campos opcionales en la respuesta
    const tenantPackage: TenantPackage | undefined = response.tenantPackage;
    const customConfig: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]

---