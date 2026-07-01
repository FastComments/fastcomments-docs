## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Returnerer: [`DeleteTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantPackageResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteTenantPackage Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeTenantPackage(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const packageId: string = "pkg_67890";

  const result: DeleteTenantPackageResponse = await deleteTenantPackage(tenantId, packageId);
  // use result as needed
}

removeTenantPackage();
[inline-code-end]