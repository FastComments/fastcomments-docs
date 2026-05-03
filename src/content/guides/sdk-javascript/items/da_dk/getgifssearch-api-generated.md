## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Ja |  |
| locale | string | Nej |  |
| rating | string | Nej |  |
| page | number | Nej |  |

## Svar

Returnerer: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---