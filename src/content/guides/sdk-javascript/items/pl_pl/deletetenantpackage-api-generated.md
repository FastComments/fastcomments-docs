## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odpowiedź

Zwraca: [`DeleteTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantPackageResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeTenantPackage(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const packageId: string = "pkg_67890";

  const result: DeleteTenantPackageResponse = await deleteTenantPackage(tenantId, packageId);
  // use result as needed
}

removeTenantPackage();
[inline-code-end]