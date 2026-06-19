## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| meta | string | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenants Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---