## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenant Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f4b2c1a";
const idOverride: string | undefined = undefined; // optionele override, indien beschikbaar
const id: string = idOverride ?? "site_3e7a6b2f";
const response: GetTenant200Response = await getTenant(tenantId, id);
console.log(response);
[inline-code-end]

---