Seznam strani za najemnika. Uporablja se v namiznem odjemalcu FChat za izpolnitev seznama njegovih sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop skupin uporabnika, ki poizveduje.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| cursor | string | Ne |  |
| limit | number | Ne |  |
| q | string | Ne |  |
| sortBy | PagesSortBy | Ne |  |
| hasComments | boolean | Ne |  |

## Odgovor

Vrača: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---