## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| meta | string | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---