Dohvaća stranice za najmoprimca (tenant). Koristi FChat desktop klijent za popunjavanje svog popisa soba.
Zahtijeva da `enableFChat` bude true na razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupi zatražujućeg korisnika.

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

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]