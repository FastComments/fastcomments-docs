## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| meta | string | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetTenantsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getTenants Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8e7d6c";
  const resultOnlyId: GetTenantsResponse1 = await getTenants(tenantId);
  const resultWithMeta: GetTenantsResponse1 = await getTenants(tenantId, "full");
  const resultAllParams: GetTenantsResponse1 = await getTenants(tenantId, "full", 15);
  console.log(resultOnlyId, resultWithMeta, resultAllParams);
})();
[inline-code-end]

---