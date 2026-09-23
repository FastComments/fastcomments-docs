## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantPackages Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPackages() {
  const tenantId: string = "acme-corp-001";
  const skip: number = 15;

  const resultWithSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
  const resultWithoutSkip: GetTenantPackagesResponse = await getTenantPackages(tenantId);
}
[inline-code-end]

---