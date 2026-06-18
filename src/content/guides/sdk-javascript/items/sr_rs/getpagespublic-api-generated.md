Prikazuje stranice za tenant. Koristi ga FChat desktop klijent za popunjavanje svoje liste soba.
Zahteva da `enableFChat` bude true u razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO se filtriraju prema pristupu grupama korisnika koji podnosi zahtev.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| cursor | string | Ne |  |
| limit | number | Ne |  |
| q | string | Ne |  |
| sortBy | PagesSortBy | Ne |  |
| hasComments | boolean | Ne |  |

## Odgovor

Vraća: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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