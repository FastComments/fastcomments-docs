Prikaže strani za najemnika. Uporablja ga namizni odjemalec FChat za napolnitev seznama sob.
Zahteva, da je `enableFChat` nastavljeno na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, se filtrirajo glede na dostop skupine uporabnika, ki poizveduje.

## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| cursor | string | Ne |  |
| limit | number | Ne |  |
| q | string | Ne |  |
| sortBy | PagesSortBy | Ne |  |
| hasComments | boolean | Ne |  |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---