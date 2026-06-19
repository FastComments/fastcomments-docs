Popis stranica za tenanta. Koristi ga FChat desktop klijent za popunjavanje svog popisa soba.
Za svaku stranicu zahtijeva da je `enableFChat` postavljen na true u razriješenoj prilagođenoj konfiguraciji.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---