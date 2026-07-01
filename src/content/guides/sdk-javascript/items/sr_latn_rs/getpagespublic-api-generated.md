Lista stranica za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje liste soba.  
Zahteva da `enableFChat` bude true u razrešenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtevaju SSO se filtriraju prema grupnom pristupu korisnika koji traži.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Odgovor

Vraća: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]