## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| meta | string | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---