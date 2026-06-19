## Parámetros

| Name | Type | Obligatorio | Descripción |
|------|------|------------|-------------|
| tenantId | string | Sí |  |
| search | string | Sí |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Respuesta

Devuelve: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const search: string = 'golden hour sunset';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 1;
const result: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---