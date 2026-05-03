## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| search | string | Sí |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Respuesta

Devuelve: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---